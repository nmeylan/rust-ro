use crate::server::core::{Server, FeatureState, Session};
use crate::packets::packets::{Packet, PacketChEnter, PacketHcRefuseEnter, CharacterInfoNeoUnion, PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptEnterNeoUnion, PacketPincodeLoginstate, PacketChMakeChar2, PacketHcAcceptMakecharNeoUnion, PacketChDeleteChar4Reserved, PacketHcDeleteChar4Reserved};
use crate::repository::lib::Repository;
use sqlx::{MySql, Error, Row};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt, BigEndian};
use crate::repository::model::char_model::{CharInsertModel, CharSelectModel};
use sqlx::mysql::{MySqlQueryResult, MySqlRow};

pub fn handle_char_enter(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>) -> FeatureState {
    let packet_char_enter = packet.as_any().downcast_ref::<PacketChEnter>().unwrap();
    let mut server_context_guard = server.server_context.lock().unwrap();
    if packet_char_enter.aid != 2000000 {
        return FeatureState::Unimplemented;
    }
    if server_context_guard.sessions.contains_key(&packet_char_enter.aid) {
        let mut session = server_context_guard.sessions.get_mut(&packet_char_enter.aid).unwrap();
        session.set_char_server_socket(tcp_stream);
        if session.login_id1 == packet_char_enter.auth_code && session.login_id2 == packet_char_enter.user_level {
            let res = runtime.block_on(async {
                load_chars_info(session.account_id, &server.repository).await
            });
            let mut tcp_stream_guard = session.char_server_socket.as_ref().unwrap().lock().unwrap();
            // The pincode packet should be appended to PacketHcAcceptEnterNeoUnionHeader packet
            let mut pincode_loginstate = PacketPincodeLoginstate::new();
            pincode_loginstate.set_aid(session.account_id);
            pincode_loginstate.set_pincode_seed(session.login_id1);
            pincode_loginstate.fill_raw();
            let mut packet_res = res.raw();
            let mut pincode_res = pincode_loginstate.raw();
            let char_info_packet: Vec<u8> = packet_res.iter().cloned().chain(pincode_res.iter().cloned()).collect();
            let mut wtr = vec![];
            // A "account id packet" should be sent just before char info packet
            wtr.write_u32::<LittleEndian>(session.account_id);
            tcp_stream_guard.write(&wtr);
            tcp_stream_guard.flush();
            tcp_stream_guard.write(&char_info_packet);
            tcp_stream_guard.flush();
            std::mem::drop(tcp_stream_guard);
            std::mem::drop(server_context_guard);
            return FeatureState::Implemented(Box::new(res));
        }
        // should not happen, but in case of forged packet, remove session
        server_context_guard.sessions.remove(&packet_char_enter.aid);
        std::mem::drop(server_context_guard);
    }
    let mut res = PacketHcRefuseEnter::new();
    res.set_error_code(0);
    res.fill_raw();
    return FeatureState::Implemented(Box::new(res));
}

pub fn handle_make_char(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let packet_make_char = packet.as_any().downcast_ref::<PacketChMakeChar2>().unwrap();
    let vit = 1;
    let max_hp = (40 * (100 + vit as u32) / 100) ;
    let int = 1;
    let max_sp = (40 * (100 + int as u32) / 100);
    let char_model = CharInsertModel {
        account_id: session_id,
        char_num: packet_make_char.char_num as i8,
        name: packet_make_char.name.iter().collect(),
        class: 0,
        zeny: 10000, // make this configurable
        status_point: 48,
        str: 1,
        agi: 1,
        vit,
        int,
        dex: 1,
        luk: 1,
        max_hp,
        hp: max_hp,
        max_sp,
        sp: max_sp,
        hair: packet_make_char.head as u16,
        hair_color: packet_make_char.head_pal as u32,
        last_map: "new_1-1".to_string(), // make this configurable
        last_x: 53,
        last_y: 111,
        save_map: "new_1-1".to_string(), // make this configurable
        save_x: 53,
        save_y: 111,
        sex: if packet_make_char.sex == 1 { "M".to_string() } else { "F".to_string() },
        inventory_size: 100
    };

    let created_char = runtime.block_on(async {
        char_model.insert(&server.repository.pool, "char").await.unwrap();
        // TODO add default stuff
        let created_char = sqlx::query_as::<_, CharacterInfoNeoUnion>("SELECT * from `char` WHERE `name`= ? AND `account_id` = ?")
            .bind(char_model.name)
            .bind(char_model.account_id)
            .fetch_one(&server.repository.pool)
            .await.unwrap();
        created_char
    });
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    let mut packet_hc_accept_makechar_neo_union = PacketHcAcceptMakecharNeoUnion::new();
    packet_hc_accept_makechar_neo_union.set_charinfo(created_char);
    packet_hc_accept_makechar_neo_union.fill_raw();
    tcp_stream_guard.write(&packet_hc_accept_makechar_neo_union.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_hc_accept_makechar_neo_union));
}

pub fn handle_delete_reserved_char(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let packet_delete_reserved_char = packet.as_any().downcast_ref::<PacketChDeleteChar4Reserved>().unwrap();
    runtime.block_on(async {
        sqlx::query("UPDATE `char` SET delete_date = UNIX_TIMESTAMP(now() + INTERVAL 1 DAY) WHERE account_id = ? AND char_id = ?")
            .bind(session_id)
            .bind(packet_delete_reserved_char.gid)
            .execute(&server.repository.pool).await.unwrap();
    });
    let mut packet_hc_delete_char4reserved = PacketHcDeleteChar4Reserved::new();
    packet_hc_delete_char4reserved.set_gid(packet_delete_reserved_char.gid);
    packet_hc_delete_char4reserved.set_delete_reserved_date(24 * 60 * 60);
    packet_hc_delete_char4reserved.set_result(1);
    packet_hc_delete_char4reserved.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_hc_delete_char4reserved.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_hc_delete_char4reserved));
}

async fn load_chars_info(account_id: u32, repository: &Repository<MySql>) -> PacketHcAcceptEnterNeoUnionHeader {
    let mut row_results = sqlx::query_as::<_, CharacterInfoNeoUnion>("SELECT * FROM `char` WHERE account_id = ?")
        .bind(account_id)
        .fetch_all(&repository.pool).await.unwrap();
    let mut accept_enter_neo_union_header = PacketHcAcceptEnterNeoUnionHeader::new();
    let mut accept_enter_neo_union = PacketHcAcceptEnterNeoUnion::new();
    accept_enter_neo_union.set_packet_length((27 + row_results.len() * 155) as i16);
    accept_enter_neo_union.set_char_info(row_results);
    accept_enter_neo_union.set_premium_start_slot(12);
    accept_enter_neo_union.set_premium_end_slot(12);
    accept_enter_neo_union.set_total_slot_num(12);
    accept_enter_neo_union_header.set_char_info(accept_enter_neo_union);
    accept_enter_neo_union_header.set_char_slot(12);
    accept_enter_neo_union_header.set_premium_slot_end(12);
    accept_enter_neo_union_header.set_premium_slot_start(12);
    accept_enter_neo_union_header.set_packet_len(29);
    accept_enter_neo_union_header.fill_raw();
    accept_enter_neo_union_header
}