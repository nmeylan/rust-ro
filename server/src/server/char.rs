use crate::server::core::{Server, FeatureState, Session, CharacterSession};
use crate::packets::packets::{Packet, PacketChEnter, PacketHcRefuseEnter, CharacterInfoNeoUnion, PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptEnterNeoUnion, PacketPincodeLoginstate, PacketChMakeChar2, PacketHcAcceptMakecharNeoUnion, PacketChDeleteChar4Reserved, PacketHcDeleteChar4Reserved, PacketChSelectChar, PacketChSendMapInfo, PacketCzEnter2, PacketMapConnection, PacketZcInventoryExpansionInfo, PacketZcOverweightPercent, PacketZcAcceptEnter2, PacketZcNpcackMapmove, PacketZcStatusValues, PacketZcParChange, PacketZcAttackRange, PacketZcNotifyChat, PacketCzRestart, PacketZcRestartAck, PacketZcReqDisconnectAck2, PacketZcMsgColor, PacketZcNotifyMapproperty2, PacketZcHatEffect, PacketZcLoadConfirm};
use crate::repository::lib::Repository;
use sqlx::{MySql, Error, Row};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt, BigEndian};
use crate::repository::model::char_model::{CharInsertModel, CharSelectModel};
use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use crate::util::string::StringUtil;
use std::net::Shutdown::Both;
use crate::util::packet::chain_packets;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::server::movement::Position;
use crate::server::map::{Map, MapPropertyFlags};
use crate::server::enums::status::StatusTypes;
use crate::server::enums::client_messages::ClientMessages;

pub fn handle_char_enter(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>) -> FeatureState {
    let packet_char_enter = packet.as_any().downcast_ref::<PacketChEnter>().unwrap();
    let mut server_context_guard = server.server_context.lock().unwrap();
    if packet_char_enter.aid != 2000000 {
        return FeatureState::Unimplemented;
    }
    if server_context_guard.sessions.contains_key(&packet_char_enter.aid) {
        let mut session = server_context_guard.sessions.get_mut(&packet_char_enter.aid).unwrap();
        session.set_char_server_socket(tcp_stream);
        if session.auth_code == packet_char_enter.auth_code && session.user_level == packet_char_enter.user_level {
            let packet_hc_accept_enter_neo_union = runtime.block_on(async {
                load_chars_info(session.account_id, &server.repository).await
            });
            let mut tcp_stream_guard = session.char_server_socket.as_ref().unwrap().lock().unwrap();
            let mut pincode_loginstate = PacketPincodeLoginstate::new();
            pincode_loginstate.set_aid(session.account_id);
            pincode_loginstate.set_pincode_seed(session.auth_code);
            pincode_loginstate.fill_raw();
            // The pincode packet should be appended to PacketHcAcceptEnterNeoUnionHeader packet
            let final_response_packet: Vec<u8> = chain_packets(vec![&packet_hc_accept_enter_neo_union, &pincode_loginstate]);
            let mut wtr = vec![];
            // A "account id packet" should be sent just before char info packet
            wtr.write_u32::<LittleEndian>(session.account_id);
            tcp_stream_guard.write(&wtr);
            tcp_stream_guard.flush();
            tcp_stream_guard.write(&final_response_packet);
            tcp_stream_guard.flush();
            std::mem::drop(tcp_stream_guard);
            std::mem::drop(server_context_guard);
            return FeatureState::Implemented(Box::new(packet_hc_accept_enter_neo_union));
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

pub fn handle_select_char(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let packet_select_char = packet.as_any().downcast_ref::<PacketChSelectChar>().unwrap();
    let row = runtime.block_on(async {
       sqlx::query("SELECT char_id, last_map, last_x, last_y, name FROM `char` WHERE account_id = ? AND char_num = ?")
            .bind(session_id)
            .bind(packet_select_char.char_num)
            .fetch_one(&server.repository.pool).await.unwrap()
    });
    let mut server_context_guard = server.server_context.lock().unwrap();
    let mut session = server_context_guard.sessions.get_mut(&session_id).unwrap();
    let char_id: u32 = row.get("char_id");
    let last_x: u16 = row.get("last_x");
    let last_y: u16 = row.get("last_y");
    let mut last_map: String = row.get("last_map");
    let mut packet_ch_send_map_info = PacketChSendMapInfo::new();
    packet_ch_send_map_info.set_gid(char_id.clone());
    let mut map_name = [0 as char; 16];
    let mut char_name = [0 as char; 24];
    last_map = format!("{}.gat", last_map);
    last_map.fill_char_array(map_name.as_mut());
    row.get::<String, _>("name").fill_char_array(char_name.as_mut());
    let character = CharacterSession {
        name: char_name,
        char_id,
        speed: 100,
        current_map: map_name.clone(),
        current_position: Position { x: last_x, y: last_y, dir: 0 },
        movement_task_id: None
    };
    session.set_character(Arc::new(Mutex::new(character)));
    packet_ch_send_map_info.set_map_name(map_name);
    packet_ch_send_map_info.set_map_server_port(6124);
    packet_ch_send_map_info.set_map_server_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
    packet_ch_send_map_info.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_ch_send_map_info.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_ch_send_map_info));
}


pub fn handle_enter_game(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>) -> FeatureState {
    let packet_enter_game = packet.as_any().downcast_ref::<PacketCzEnter2>().unwrap();
    if packet_enter_game.aid != 2000000 { // Currently only handle this account to be able to still use proxy in other accounts
        return FeatureState::Unimplemented;
    }
    let mut server_context_guard = server.server_context.lock().unwrap();
    let mut session = server_context_guard.sessions.get_mut(&packet_enter_game.aid);
    if session.is_none() {
        tcp_stream.lock().unwrap().shutdown(Both);
        return FeatureState::Unimplemented;
    }
    let mut session = session.unwrap();
    if packet_enter_game.auth_code != session.auth_code {
        tcp_stream.lock().unwrap().shutdown(Both);
        server_context_guard.sessions.remove(&packet_enter_game.aid);
        return FeatureState::Unimplemented;
    }
    session.set_map_server_socket(tcp_stream);
    let mut packet_map_connection = PacketMapConnection::new();
    packet_map_connection.set_aid(session.account_id);
    let mut tcp_stream_guard = session.map_server_socket.as_ref().unwrap().lock().unwrap();
    tcp_stream_guard.write(&packet_map_connection.raw());
    tcp_stream_guard.flush();

    /*
    Client expect multiple packets in response to packet PacketCzEnter2
    */
    let mut packet_inventory_expansion_info = PacketZcInventoryExpansionInfo::new();
    packet_inventory_expansion_info.fill_raw();
    let mut packet_overweight_percent = PacketZcOverweightPercent::new();
    packet_overweight_percent.fill_raw();
    let mut packet_accept_enter = PacketZcAcceptEnter2::new();
    packet_accept_enter.set_start_time(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32);
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.fill_raw();
    let character = session.character.as_ref().unwrap();
    let character_session_guard = character.lock().unwrap();
    let map_name : String = Map::name_without_ext(character_session_guard.get_current_map_name());
    let mut maps_guard = server.maps.lock().unwrap();
    let map = maps_guard.get_mut(&map_name[..]).unwrap();
    map.player_join_map();
    let mut packet_npc_ack_map_move = PacketZcNpcackMapmove::new();
    packet_npc_ack_map_move.set_map_name(character_session_guard.current_map);
    packet_npc_ack_map_move.set_x_pos(character_session_guard.current_position.x as i16);
    packet_npc_ack_map_move.set_y_pos(character_session_guard.current_position.y as i16);
    packet_npc_ack_map_move.fill_raw();
    let final_response_packet: Vec<u8> = chain_packets(vec![&packet_inventory_expansion_info, &packet_overweight_percent, &packet_accept_enter, &packet_npc_ack_map_move]);
    tcp_stream_guard.write(&final_response_packet);
    tcp_stream_guard.flush();

    let mut packet_str = PacketZcStatusValues::new();
    packet_str.set_status_type(StatusTypes::STR.value());
    packet_str.set_default_status(1);
    packet_str.fill_raw();
    let mut packet_agi = PacketZcStatusValues::new();
    packet_agi.set_status_type(StatusTypes::AGI.value());
    packet_agi.set_default_status(1);
    packet_agi.fill_raw();
    let mut packet_dex = PacketZcStatusValues::new();
    packet_dex.set_status_type(StatusTypes::DEX.value());
    packet_dex.set_default_status(1);
    packet_dex.fill_raw();
    let mut packet_int = PacketZcStatusValues::new();
    packet_int.set_status_type(StatusTypes::INT.value());
    packet_int.set_default_status(1);
    packet_int.fill_raw();
    let mut packet_luk = PacketZcStatusValues::new();
    packet_luk.set_status_type(StatusTypes::LUK.value());
    packet_luk.set_default_status(1);
    packet_luk.fill_raw();
    let mut packet_hit = PacketZcParChange::new();
    packet_hit.set_var_id(StatusTypes::HIT.value() as u16);
    packet_hit.set_count(1);
    packet_hit.fill_raw();
    let mut packet_flee = PacketZcParChange::new();
    packet_flee.set_var_id(StatusTypes::FLEE1.value() as u16);
    packet_flee.set_count(1);
    packet_flee.fill_raw();
    let mut packet_aspd = PacketZcParChange::new();
    packet_aspd.set_var_id(StatusTypes::ASPD.value() as u16);
    packet_aspd.set_count(1);
    packet_aspd.fill_raw();
    let mut packet_atk = PacketZcParChange::new();
    packet_atk.set_var_id(StatusTypes::ATK1.value() as u16);
    packet_atk.set_count(1);
    packet_atk.fill_raw();
    let mut packet_def = PacketZcParChange::new();
    packet_def.set_var_id(StatusTypes::DEF1.value() as u16);
    packet_def.set_count(1);
    packet_def.fill_raw();
    let mut packet_def2 = PacketZcParChange::new();
    packet_def2.set_var_id(StatusTypes::DEF2.value() as u16);
    packet_def2.set_count(1);
    packet_def2.fill_raw();
    let mut packet_flee2 = PacketZcParChange::new();
    packet_flee2.set_var_id(StatusTypes::FLEE2.value() as u16);
    packet_flee2.set_count(1);
    packet_flee2.fill_raw();
    let mut packet_crit = PacketZcParChange::new();
    packet_crit.set_var_id(StatusTypes::CRITICAL.value() as u16);
    packet_crit.set_count(1);
    packet_crit.fill_raw();
    let mut packet_matk = PacketZcParChange::new();
    packet_matk.set_var_id(StatusTypes::MATK1.value() as u16);
    packet_matk.set_count(1);
    packet_matk.fill_raw();
    let mut packet_matk2 = PacketZcParChange::new();
    packet_matk2.set_var_id(StatusTypes::MATK2.value() as u16);
    packet_matk2.set_count(1);
    packet_matk2.fill_raw();
    let mut packet_mdef2 = PacketZcParChange::new();
    packet_mdef2.set_var_id(StatusTypes::MDEF2.value() as u16);
    packet_mdef2.set_count(1);
    packet_mdef2.fill_raw();
    let mut packet_attack_range = PacketZcAttackRange::new();
    packet_attack_range.set_current_att_range(1);
    packet_attack_range.fill_raw();
    let mut packet_maxhp = PacketZcParChange::new();
    packet_maxhp.set_var_id(StatusTypes::MAXHP.value() as u16);
    packet_maxhp.set_count(1);
    packet_maxhp.fill_raw();
    let mut packet_maxsp = PacketZcParChange::new();
    packet_maxsp.set_var_id(StatusTypes::MAXSP.value() as u16);
    packet_maxsp.set_count(1);
    packet_maxsp.fill_raw();
    let mut packet_hp = PacketZcParChange::new();
    packet_hp.set_var_id(StatusTypes::HP.value() as u16);
    packet_hp.set_count(1);
    packet_hp.fill_raw();
    let mut packet_sp = PacketZcParChange::new();
    packet_sp.set_var_id(StatusTypes::SP.value() as u16);
    packet_sp.set_count(1);
    packet_sp.fill_raw();
    let mut packet_notify_chat = PacketZcNotifyChat::new();
    packet_notify_chat.set_gid(character_session_guard.char_id);
    packet_notify_chat.set_msg("Hello from rust ragnarok".to_string());

    let final_response_packet: Vec<u8> = chain_packets(vec![
        &packet_str, &packet_agi, &packet_dex, &packet_int, &packet_luk,
        &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_def,
        &packet_def2, &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
        &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
        &packet_sp, &packet_notify_chat
    ]);
    println!("{:02X?}", &final_response_packet[..]);
    tcp_stream_guard.write(&final_response_packet);
    tcp_stream_guard.flush();

    return FeatureState::Implemented(Box::new(packet_map_connection));
}

pub fn handle_restart(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let packet_restart = packet.as_any().downcast_ref::<PacketCzRestart>().unwrap();
    let mut server_context_guard = server.server_context.lock().unwrap();
    let mut session = server_context_guard.sessions.get_mut(&session_id).unwrap();
    session.unset_character();

    let mut restart_ack = PacketZcRestartAck::new();
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&restart_ack.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(restart_ack));
}

pub fn handle_disconnect(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let mut server_context_guard = server.server_context.lock().unwrap();
    server_context_guard.remove_session(session_id);

    let mut disconnect_ack = PacketZcReqDisconnectAck2::new();
    disconnect_ack.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&disconnect_ack.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(disconnect_ack));
}

pub fn handle_char_loaded_client_side(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let mut packet_zc_msg_color = PacketZcMsgColor::new();
    let mut packet_zc_notify_mapproperty2 = PacketZcNotifyMapproperty2::new();
    let mut packet_zc_hat_effect = PacketZcHatEffect::new();
    packet_zc_msg_color.set_msg_id(ClientMessages::MSGATTENDANCEUNAVAILABLE.value());
    packet_zc_msg_color.set_msg_color(255);
    packet_zc_notify_mapproperty2.set_atype(0x28);
    let mut flags = MapPropertyFlags::new();
    flags.set_is_use_cart(true); // TODO add other flags correctly
    packet_zc_notify_mapproperty2.set_flags(flags.raw());
    packet_zc_notify_mapproperty2.fill_raw();
    packet_zc_hat_effect.set_aid(session_id);
    packet_zc_hat_effect.set_status(1);
    packet_zc_hat_effect.set_len(9 + 0); // len is: 9 (packet len) + number of effects
    packet_zc_hat_effect.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_zc_msg_color.raw());
    tcp_stream_guard.flush();
    let mut final_response_packet: Vec<u8> = chain_packets(vec![&packet_zc_hat_effect, &packet_zc_notify_mapproperty2]);
    tcp_stream_guard.write(&final_response_packet);
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_zc_msg_color));
}

pub fn handle_blocking_play_cancel(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let mut packet_zc_load_confirm = PacketZcLoadConfirm::new();
    packet_zc_load_confirm.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_zc_load_confirm.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_zc_load_confirm));
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