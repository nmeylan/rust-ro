use std::collections::HashSet;
use packets::packets::{Packet, PacketChEnter, PacketHcRefuseEnter, CharacterInfoNeoUnion, PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptEnterNeoUnion, PacketPincodeLoginstate, PacketHcAcceptMakecharNeoUnion, PacketChDeleteChar4Reserved, PacketHcDeleteChar4Reserved, PacketChSelectChar, PacketChSendMapInfo, PacketCzEnter2, PacketMapConnection, PacketZcInventoryExpansionInfo, PacketZcOverweightPercent, PacketZcAcceptEnter2, PacketZcStatusValues, PacketZcParChange, PacketZcAttackRange, PacketZcNotifyChat, PacketCzRestart, PacketZcRestartAck, PacketZcReqDisconnectAck2, PacketZcLoadConfirm, PacketChMakeChar3, PacketChMakeChar, PacketHcNotifyZonesvr, ZserverAddr};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex, RwLock};
use std::net::TcpStream;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};
use crate::repository::model::char_model::{CharacterInfoNeoUnionWrapped, CharInsertModel, CharSelectModel};
use crate::util::string::StringUtil;
use std::net::Shutdown::Both;
use std::sync::atomic::{AtomicU16};
use crate::util::packet::chain_packets;
use crate::Map;
use crate::server::enums::status::StatusTypes;
use crate::server::core::character::{Character};
use crate::server::core::character_movement::{change_map_packet};
use crate::server::core::request::Request;
use crate::server::core::session::Session;
use crate::server::core::status::Status;
use crate::server::script::ScriptGlobalVariableStore;
use crate::server::server::{Server};
use crate::util::tick::get_tick;

pub fn handle_char_enter(server: Arc<Server>, context: Request) {
    let packet_char_enter = cast!(context.packet(), PacketChEnter);
    let mut sessions_guard = write_lock!(server.sessions);

    if sessions_guard.contains_key(&packet_char_enter.aid) {
        let session = sessions_guard.get(&packet_char_enter.aid).unwrap();
        let session = Arc::new(session.recreate_with_char_socket(context.socket().clone()));
        sessions_guard.insert(packet_char_enter.aid, session.clone());
        if session.auth_code == packet_char_enter.auth_code && session.user_level == packet_char_enter.user_level {
            let packet_hc_accept_enter_neo_union = context.runtime().block_on(async {
                load_chars_info(session.account_id, server.clone()).await
            });
            let mut pincode_loginstate = PacketPincodeLoginstate::new();
            pincode_loginstate.set_aid(session.account_id);
            pincode_loginstate.set_pincode_seed(session.auth_code);
            pincode_loginstate.fill_raw();
            // The pincode packet should be appended to PacketHcAcceptEnterNeoUnionHeader packet
            let final_response_packet: Vec<u8> = chain_packets(vec![&packet_hc_accept_enter_neo_union, &pincode_loginstate]);
            let mut wtr = vec![];
            // A "account id packet" should be sent just before char info packet
            wtr.write_u32::<LittleEndian>(session.account_id).expect("Unable to write Little endian u32 from session account id");
            socket_send_raw!(context, wtr);
            socket_send_raw!(context, final_response_packet);
            return;
        }
        // should not happen, but in case of forged packet, remove session
        server.remove_session(packet_char_enter.aid);
    }
    let mut res = PacketHcRefuseEnter::new();
    res.set_error_code(0);
    res.fill_raw();
    socket_send!(context, res);
}

pub fn handle_make_char(server: Arc<Server>, context: Request) {

    let mut char_model: Option<CharInsertModel> = None;
    if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
        let vit = 1;
        let max_hp = 40 * (100 + vit as u32) / 100 ;
        let int = 1;
        let max_sp = 40 * (100 + int as u32) / 100;
        let packet_make_char = cast!(context.packet(), PacketChMakeChar3);
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id,
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
        });
    } else if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
        let packet_make_char = cast!(context.packet(), PacketChMakeChar);
        let vit = packet_make_char.vit as u16;
        let max_hp = 40 * (100 + vit as u32) / 100 ;
        let int = 1;
        let max_sp = 40 * (100 + int as u32) / 100;
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id,
            char_num: packet_make_char.char_num as i8,
            name: packet_make_char.name.iter().collect(),
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: packet_make_char.str as u16,
            agi: packet_make_char.agi as u16,
            vit,
            int,
            dex: packet_make_char.dex as u16,
            luk: packet_make_char.luk as u16,
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
            sex: "M".to_string(),
            inventory_size: 100
        });
    }
    if char_model.is_none() {
        error!("Char model is not initialized, probably packet was not recognized");
        return;
    }

    let created_char = context.runtime().block_on(async {
        let char_model = char_model.unwrap();
        char_model.insert(&server.repository.pool, "char").await.unwrap();
        // TODO add default stuff
        let created_char: CharacterInfoNeoUnionWrapped = sqlx::query_as::<_, CharacterInfoNeoUnionWrapped>("SELECT * from `char` WHERE `name`= ? AND `account_id` = ?")
            .bind(char_model.name)
            .bind(char_model.account_id)
            .fetch_one(&server.repository.pool)
            .await.unwrap();
        created_char.data
    });
    let mut packet_hc_accept_makechar_neo_union = PacketHcAcceptMakecharNeoUnion::new();
    packet_hc_accept_makechar_neo_union.set_charinfo(created_char);
    packet_hc_accept_makechar_neo_union.fill_raw();
    socket_send!(context, packet_hc_accept_makechar_neo_union);
}

pub fn handle_delete_reserved_char(server: Arc<Server>, context: Request) {
    let packet_delete_reserved_char = cast!(context.packet(), PacketChDeleteChar4Reserved);
    context.runtime().block_on(async {
        sqlx::query("UPDATE `char` SET delete_date = UNIX_TIMESTAMP(now() + INTERVAL 1 DAY) WHERE account_id = ? AND char_id = ?")
            .bind(context.session().account_id)
            .bind(packet_delete_reserved_char.gid)
            .execute(&server.repository.pool).await.unwrap();
    });
    let mut packet_hc_delete_char4reserved = PacketHcDeleteChar4Reserved::new();
    packet_hc_delete_char4reserved.set_gid(packet_delete_reserved_char.gid);
    packet_hc_delete_char4reserved.set_delete_reserved_date(24 * 60 * 60);
    packet_hc_delete_char4reserved.set_result(1);
    packet_hc_delete_char4reserved.fill_raw();
    socket_send!(context, packet_hc_delete_char4reserved);
}

pub fn handle_select_char(server: Arc<Server>, context: Request) {
    let packet_select_char = cast!(context.packet(), PacketChSelectChar);
    let session_id = context.session().account_id;
    let char_model: CharSelectModel = context.runtime().block_on(async {
        sqlx::query_as::<_, CharSelectModel>("SELECT * FROM `char` WHERE account_id = ? AND char_num = ?")
            .bind(session_id)
            .bind(packet_select_char.char_num)
            .fetch_one(&server.repository.pool).await.unwrap()
    });
    let mut sessions_guard = write_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let char_id: u32 = char_model.char_id;
    let last_x: u16 = char_model.last_x;
    let last_y: u16 = char_model.last_y;
    let mut last_map: String = char_model.last_map.clone();
    if last_map.is_empty() {
        last_map = "prontera".to_string();
    }
    let mut map_name = [0 as char; 16];
    last_map = format!("{}.gat", last_map);
    last_map.fill_char_array(map_name.as_mut());
    let character = Character {
        name: char_model.name.clone(),
        char_id,
        status: Status::from_char_model(&char_model, &server.configuration.game),
        current_map_name: RwLock::new(map_name),
        x: AtomicU16::new(last_x),
        y: AtomicU16::new(last_y),
        movement_tasks: Mutex::new(vec![]),
        map_view: RwLock::new(HashSet::new()),
        current_map: RwLock::new(None),
        self_ref: RwLock::new(None),
        script_variable_store: Mutex::new(ScriptGlobalVariableStore::default()),
        account_id: session_id
    };
    let session = Arc::new(context.session().recreate_with_character(character.char_id));
    sessions_guard.insert(session_id, session);
    if server.packetver() < 20170329 {
        let mut packet_ch_send_map_info = PacketHcNotifyZonesvr::new();
        packet_ch_send_map_info.set_gid(character.char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        let mut zserver_addr = ZserverAddr::new();
        zserver_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        zserver_addr.set_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_addr(zserver_addr);
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    } else {
        let mut packet_ch_send_map_info = PacketChSendMapInfo::new();
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        packet_ch_send_map_info.set_map_server_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_map_server_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    }
}


pub fn handle_enter_game(server: Arc<Server>, context: Request) {

    let aid;
    let auth_code;
    if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
        let packet_enter_game = cast!(context.packet(), PacketCzEnter2);
        aid = packet_enter_game.aid;
        auth_code = packet_enter_game.auth_code;
    } else {
        error!("Not recognized PacketCzEnterX");
        return;
    }
    let mut sessions_guard = write_lock!(server.sessions);
    let session = sessions_guard.get(&aid);
    if session.is_none() {
        write_lock!(context.socket()).shutdown(Both).expect("Unable to shutdown incoming socket. Shutdown was done because session does not exists");
        return;
    }
    let session = session.unwrap();
    if auth_code != session.auth_code {
        write_lock!(context.socket()).shutdown(Both).expect("Unable to shutdown incoming socket. Shutdown was done because packet auth_code mismatching session auth_code");
        server.remove_session(aid);
        return;
    }
    let session = Arc::new(session.recreate_with_map_socket(context.socket().clone()));
    sessions_guard.insert(aid, session.clone());
    let mut packet_map_connection = PacketMapConnection::new();
    packet_map_connection.set_aid(session.account_id);

    socket_send!(context, packet_map_connection);

    /*
    Client expect multiple packets in response to packet PacketCzEnter2
    */
    let mut packet_inventory_expansion_info = PacketZcInventoryExpansionInfo::new();
    packet_inventory_expansion_info.fill_raw();
    let mut packet_overweight_percent = PacketZcOverweightPercent::new();
    packet_overweight_percent.fill_raw();
    let mut packet_accept_enter = PacketZcAcceptEnter2::new();
    packet_accept_enter.set_start_time(get_tick());
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.fill_raw();
    let char_id = session.char_id();
    let character = server.get_character_unsafe(char_id);

    let packet_zc_npcack_mapmove = change_map_packet(&Map::name_without_ext(character.get_current_map_name()), character.x(), character.y(), session.clone(), server.clone());
    let final_response_packet: Vec<u8> = chain_packets(vec![&packet_accept_enter, &packet_zc_npcack_mapmove]);
    // let final_response_packet: Vec<u8> = chain_packets(vec![&packet_inventory_expansion_info, &packet_overweight_percent, &packet_accept_enter, &packet_zc_npcack_mapmove]);
    socket_send_raw!(context, final_response_packet);

    let mut packet_str = PacketZcStatusValues::new();
    packet_str.set_status_type(StatusTypes::Str.value());
    packet_str.set_default_status(character.status.str as i32);
    packet_str.fill_raw();
    let mut packet_agi = PacketZcStatusValues::new();
    packet_agi.set_status_type(StatusTypes::Agi.value());
    packet_agi.set_default_status(character.status.agi as i32);
    packet_agi.fill_raw();
    let mut packet_dex = PacketZcStatusValues::new();
    packet_dex.set_status_type(StatusTypes::Dex.value());
    packet_dex.set_default_status(character.status.dex as i32);
    packet_dex.fill_raw();
    let mut packet_int = PacketZcStatusValues::new();
    packet_int.set_status_type(StatusTypes::Int.value());
    packet_int.set_default_status(character.status.int as i32);
    packet_int.fill_raw();
    let mut packet_luk = PacketZcStatusValues::new();
    packet_luk.set_status_type(StatusTypes::Luk.value());
    packet_luk.set_default_status(character.status.luk as i32);
    packet_luk.fill_raw();
    let mut packet_hit = PacketZcParChange::new();
    packet_hit.set_var_id(StatusTypes::Hit.value() as u16);
    packet_hit.set_count(character.status.hit as i32);
    packet_hit.fill_raw();
    let mut packet_flee = PacketZcParChange::new();
    packet_flee.set_var_id(StatusTypes::Flee1.value() as u16);
    packet_flee.set_count(character.status.flee as i32);
    packet_flee.fill_raw();
    let mut packet_aspd = PacketZcParChange::new();
    packet_aspd.set_var_id(StatusTypes::Aspd.value() as u16);
    packet_aspd.set_count(character.status.aspd as i32);
    packet_aspd.fill_raw();
    let mut packet_atk = PacketZcParChange::new();
    packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
    packet_atk.set_count(character.status.base_atk as i32);
    packet_atk.fill_raw();
    let mut packet_def = PacketZcParChange::new();
    packet_def.set_var_id(StatusTypes::Def1.value() as u16);
    packet_def.set_count(character.status.def as i32);
    packet_def.fill_raw();
    let mut packet_flee2 = PacketZcParChange::new();
    packet_flee2.set_var_id(StatusTypes::Flee2.value() as u16);
    packet_flee2.set_count(character.status.flee as i32);
    packet_flee2.fill_raw();
    let mut packet_crit = PacketZcParChange::new();
    packet_crit.set_var_id(StatusTypes::Critical.value() as u16);
    packet_crit.set_count(character.status.crit as i32);
    packet_crit.fill_raw();
    let mut packet_matk = PacketZcParChange::new();
    packet_matk.set_var_id(StatusTypes::Matk1.value() as u16);
    packet_matk.set_count(character.status.matk_min as i32);
    packet_matk.fill_raw();
    let mut packet_matk2 = PacketZcParChange::new();
    packet_matk2.set_var_id(StatusTypes::Matk2.value() as u16);
    packet_matk2.set_count(character.status.matk_max as i32);
    packet_matk2.fill_raw();
    let mut packet_mdef2 = PacketZcParChange::new();
    packet_mdef2.set_var_id(StatusTypes::Mdef2.value() as u16);
    packet_mdef2.set_count(character.status.mdef as i32);
    packet_mdef2.fill_raw();
    let mut packet_attack_range = PacketZcAttackRange::new();
    packet_attack_range.set_current_att_range(1);
    packet_attack_range.fill_raw();
    let mut packet_maxhp = PacketZcParChange::new();
    packet_maxhp.set_var_id(StatusTypes::Maxhp.value() as u16);
    packet_maxhp.set_count(character.status.max_hp as i32);
    packet_maxhp.fill_raw();
    let mut packet_maxsp = PacketZcParChange::new();
    packet_maxsp.set_var_id(StatusTypes::Maxsp.value() as u16);
    packet_maxsp.set_count(character.status.max_sp as i32);
    packet_maxsp.fill_raw();
    let mut packet_hp = PacketZcParChange::new();
    packet_hp.set_var_id(StatusTypes::Hp.value() as u16);
    packet_hp.set_count(character.status.hp as i32);
    packet_hp.fill_raw();
    let mut packet_sp = PacketZcParChange::new();
    packet_sp.set_var_id(StatusTypes::Sp.value() as u16);
    packet_sp.set_count(character.status.sp as i32);
    packet_sp.fill_raw();
    let mut packet_speed = PacketZcParChange::new();
    packet_speed.set_var_id(StatusTypes::Speed.value() as u16);
    packet_speed.set_count(character.status.speed as i32);
    packet_speed.fill_raw();
    let mut packet_notify_chat = PacketZcNotifyChat::new();
    packet_notify_chat.set_gid(character.char_id);
    packet_notify_chat.set_msg("Hello from rust ragnarok".to_string());
    packet_notify_chat.set_packet_length((packet_notify_chat.msg.len() + 8) as i16);
    packet_notify_chat.fill_raw();

    let final_response_packet: Vec<u8> = chain_packets(vec![
        &packet_str, &packet_agi, &packet_dex, &packet_int, &packet_luk,
        &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_def,
        &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
        &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
        &packet_sp, &packet_speed, &packet_notify_chat
    ]);
    socket_send_raw!(context, final_response_packet);
}

pub fn handle_restart(server: Arc<Server>, context: Request) {
    let packet_restart = cast!(context.packet(), PacketCzRestart);
    let session_id = context.session().account_id;
    let mut sessions_guard = write_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let char_id = session.char_id();
    let character = server.get_character_unsafe(char_id);
    character.remove_from_existing_map();

    let session = sessions_guard.get(&session_id).unwrap();
    let session = Arc::new(session.recreate_without_character());
    sessions_guard.insert(session_id, session);

    let mut restart_ack = PacketZcRestartAck::new();
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    socket_send!(context, restart_ack);
}

pub fn handle_disconnect(server: Arc<Server>, context: Request) {
    let session = context.session();
    let char_id = session.char_id();
    let character = server.get_character_unsafe(char_id);
    character.remove_from_existing_map();
    server.remove_session(session.account_id);

    let mut disconnect_ack = PacketZcReqDisconnectAck2::new();
    disconnect_ack.fill_raw();
    socket_send!(context, disconnect_ack);
}


pub fn handle_blocking_play_cancel(context: Request) {
    let mut packet_zc_load_confirm = PacketZcLoadConfirm::new();
    packet_zc_load_confirm.fill_raw();
    socket_send!(context, packet_zc_load_confirm);
}

async fn load_chars_info(account_id: u32, server: Arc<Server>) -> PacketHcAcceptEnterNeoUnionHeader {
    let row_results = sqlx::query_as::<_, CharacterInfoNeoUnionWrapped>("SELECT * FROM `char` WHERE account_id = ?")
        .bind(account_id)
        .fetch_all(&server.repository.pool).await.unwrap();
    let mut accept_enter_neo_union_header = PacketHcAcceptEnterNeoUnionHeader::new();
    let mut accept_enter_neo_union = PacketHcAcceptEnterNeoUnion::new();
    accept_enter_neo_union.set_packet_length((27 + row_results.len() * CharacterInfoNeoUnion::base_len(server.packetver())) as i16);
    accept_enter_neo_union.set_char_info(row_results.iter().map(|wrapped| {
        debug!("{}", wrapped.data);
        wrapped.data.clone()
    }).collect::<Vec<CharacterInfoNeoUnion>>());
    accept_enter_neo_union.set_premium_start_slot(12);
    accept_enter_neo_union.set_premium_end_slot(12);
    accept_enter_neo_union.set_total_slot_num(12);
    accept_enter_neo_union_header.set_char_info(accept_enter_neo_union);
    accept_enter_neo_union_header.set_char_slot(12);
    accept_enter_neo_union_header.set_premium_slot_end(12);
    accept_enter_neo_union_header.set_premium_slot_start(12);
    accept_enter_neo_union_header.set_packet_len(29);
    accept_enter_neo_union_header.fill_raw_with_packetver(Some(server.packetver()));
    accept_enter_neo_union_header
}