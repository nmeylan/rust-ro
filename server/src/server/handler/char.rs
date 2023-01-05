use std::collections::HashSet;
use std::net::Shutdown::Both;
use std::sync::{Arc, Mutex};

use byteorder::{LittleEndian, WriteBytesExt};
use sqlx::Postgres;

use packets::packets::{CharacterInfoNeoUnion, Packet, PacketChDeleteChar4Reserved, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar, PacketChSendMapInfo, PacketCzEnter2, PacketCzRestart, PacketHcAcceptEnterNeoUnion, PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptMakecharNeoUnion, PacketHcDeleteChar4Reserved, PacketHcNotifyZonesvr, PacketHcRefuseEnter, PacketMapConnection, PacketPincodeLoginstate, PacketZcAcceptEnter2, PacketZcAttackRange, PacketZcInventoryExpansionInfo, PacketZcLoadConfirm, PacketZcNotifyChat, PacketZcOverweightPercent, PacketZcParChange, PacketZcReqDisconnectAck2, PacketZcRestartAck, PacketZcStatus, PacketZcStatusValues, ZserverAddr};

use crate::repository::model::char_model::{CharacterInfoNeoUnionWrapped, CharInsertModel, CharSelectModel};
use crate::server::core::map::Map;
use crate::server::core::map_instance::MapInstanceKey;
use crate::server::core::position::Position;
use crate::server::core::request::Request;
use enums::status::StatusTypes;
use crate::server::events::game_event::{CharacterRemoveFromMap, GameEvent};
use crate::server::events::game_event::GameEvent::CharacterInitInventory;
use crate::server::script::ScriptGlobalVariableStore;
use crate::server::Server;
use crate::server::service::battle_service::BattleService;
use crate::server::service::character::character_service::CharacterService;
use crate::server::state::character::Character;
use crate::server::state::status::Status;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

pub fn handle_char_enter(server: &Server, context: Request) {
    let packet_char_enter = cast!(context.packet(), PacketChEnter);
    let mut sessions_guard = write_lock!(server.sessions);

    if sessions_guard.contains_key(&packet_char_enter.aid) {
        let session = sessions_guard.get(&packet_char_enter.aid).unwrap();
        let session = Arc::new(session.recreate_with_char_socket(context.socket()));
        sessions_guard.insert(packet_char_enter.aid, session.clone());
        if session.auth_code == packet_char_enter.auth_code && session.user_level == packet_char_enter.user_level {
            let packet_hc_accept_enter_neo_union = context.runtime().block_on(async {
                load_chars_info(session.account_id, server).await
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

pub fn handle_make_char(server: &Server, context: Request) {
    let mut char_model: Option<CharInsertModel> = None;
    if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
        let vit = 1;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = 1;
        let max_sp = 40 * (100 + int as i32) / 100;
        let packet_make_char = cast!(context.packet(), PacketChMakeChar3);
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
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
            hair: packet_make_char.head,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: if packet_make_char.sex == 1 { "M".to_string() } else { "F".to_string() },
            inventory_slots: context.configuration().game.max_inventory as i32,
        });
    } else if context.packet().as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
        let packet_make_char = cast!(context.packet(), PacketChMakeChar2);
        let vit = 5 as i16;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = 5;
        let max_sp = 40 * (100 + int as i32) / 100;
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: 5 as i16,
            agi: 5 as i16,
            vit,
            int,
            dex: 5 as i16,
            luk: 5 as i16,
            max_hp,
            hp: max_hp,
            max_sp,
            sp: max_sp,
            hair: packet_make_char.head as i16,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: "M".to_string(), // TODO use account sex
            inventory_slots: context.configuration().game.max_inventory as i32,
        });
    } else if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
        let packet_make_char = cast!(context.packet(), PacketChMakeChar);
        let vit = packet_make_char.vit as i16;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = packet_make_char.int as i16;
        let max_sp = 40 * (100 + int as i32) / 100;
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: packet_make_char.str as i16,
            agi: packet_make_char.agi as i16,
            vit,
            int,
            dex: packet_make_char.dex as i16,
            luk: packet_make_char.luk as i16,
            max_hp,
            hp: max_hp,
            max_sp,
            sp: max_sp,
            hair: packet_make_char.head as i16,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: "M".to_string(), // TODO use account sex
            inventory_slots: context.configuration().game.max_inventory as i32,
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
        let created_char: CharacterInfoNeoUnionWrapped = sqlx::query_as::<_, CharacterInfoNeoUnionWrapped>("SELECT * from char WHERE name = $1 AND account_id = $2")
            .bind(char_model.name)
            .bind(char_model.account_id as i32)
            .fetch_one(&server.repository.pool)
            .await.unwrap();
        created_char.data
    });
    let mut packet_hc_accept_makechar_neo_union = PacketHcAcceptMakecharNeoUnion::new();
    packet_hc_accept_makechar_neo_union.set_charinfo(created_char);
    packet_hc_accept_makechar_neo_union.fill_raw();
    socket_send!(context, packet_hc_accept_makechar_neo_union);
}

pub fn handle_delete_reserved_char(server: &Server, context: Request) {
    let packet_delete_reserved_char = cast!(context.packet(), PacketChDeleteChar4Reserved);
    context.runtime().block_on(async {
        sqlx::query("UPDATE `char` SET delete_date = UNIX_TIMESTAMP(now() + INTERVAL 1 DAY) WHERE account_id = $1 AND char_id = $2")
            .bind(context.session().account_id as i32)
            .bind(packet_delete_reserved_char.gid as i32)
            .execute(&server.repository.pool).await.unwrap();
    });
    let mut packet_hc_delete_char4reserved = PacketHcDeleteChar4Reserved::new();
    packet_hc_delete_char4reserved.set_gid(packet_delete_reserved_char.gid);
    packet_hc_delete_char4reserved.set_delete_reserved_date(24 * 60 * 60);
    packet_hc_delete_char4reserved.set_result(1);
    packet_hc_delete_char4reserved.fill_raw();
    socket_send!(context, packet_hc_delete_char4reserved);
}

pub fn handle_select_char(server: &Server, context: Request) {
    let packet_select_char = cast!(context.packet(), PacketChSelectChar);
    let session_id = context.session().account_id;
    let char_model: CharSelectModel = context.runtime().block_on(async {
        sqlx::query_as::<_, CharSelectModel>("SELECT * FROM char WHERE account_id = $1 AND char_num = $2")
            .bind(session_id as i32)
            .bind(packet_select_char.char_num as i16)
            .fetch_one(&server.repository.pool).await.unwrap()
    });
    let mut sessions_guard = write_lock!(server.sessions);
    let _session = sessions_guard.get(&session_id).unwrap();
    let char_id: u32 = char_model.char_id as u32;
    let last_x: u16 = char_model.last_x as u16;
    let last_y: u16 = char_model.last_y as u16;
    let mut last_map: String = char_model.last_map.clone();
    if last_map.is_empty() {
        last_map = "prontera".to_string();
    }

    let character = Character {
        name: char_model.name.clone(),
        char_id,
        status: Status::from_char_model(&char_model, &server.configuration.game),
        loaded_from_client_side: false,
        x: last_x,
        y: last_y,
        dir: 0,
        movements: vec![],
        attack: None,
        inventory: vec![], // todo load from db
        map_view: HashSet::new(),
        script_variable_store: Mutex::new(ScriptGlobalVariableStore::default()),
        account_id: session_id,
        map_instance_key: MapInstanceKey::new(last_map, 0),
    };
    let char_id = character.char_id;
    let session = Arc::new(context.session().recreate_with_character(char_id));
    let mut map_name = [0 as char; 16];
    character.current_map_name().fill_char_array(map_name.as_mut());
    server.insert_character(character);
    sessions_guard.insert(session_id, session);
    if server.packetver() < 20170329 {
        let mut packet_ch_send_map_info = PacketHcNotifyZonesvr::new();
        packet_ch_send_map_info.set_gid(char_id);
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


pub fn handle_enter_game(server: &Server, context: Request) {
    let aid;
    let auth_code;
    info!("handle_enter_game");
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
    let session = Arc::new(session.recreate_with_map_socket(context.socket()));
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
    let char_id = session.char_id();
    let character = server.get_character_unsafe(char_id);
    let mut packet_accept_enter = PacketZcAcceptEnter2::new();
    packet_accept_enter.set_start_time(get_tick_client());
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.set_pos_dir(Position { x: character.x(), y: character.y(), dir: character.dir() }.to_pos());
    packet_accept_enter.fill_raw();

    CharacterService::instance().schedule_warp_to_walkable_cell(&Map::name_without_ext(character.current_map_name()), character.x(), character.y(), session.char_id(), server);
    socket_send!(context, packet_accept_enter);

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
    let aspd = BattleService::instance().aspd(character.as_ref());
    packet_aspd.set_count(BattleService::instance().client_aspd(aspd));
    packet_aspd.fill_raw();
    let mut packet_atk = PacketZcParChange::new();
    packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
    packet_atk.set_count(BattleService::instance().base_atk(character.as_ref()) as i32);
    packet_atk.fill_raw();
    let mut packet_atk2 = PacketZcParChange::new();
    packet_atk2.set_var_id(StatusTypes::Atk2.value() as u16);
    packet_atk2.set_count(BattleService::instance().atk2(character.as_ref()) as i32);
    packet_atk2.fill_raw();
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
        &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_atk2, &packet_def,
        &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
        &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
        &packet_sp, &packet_speed, &packet_notify_chat,
    ]);
    socket_send_raw!(context, final_response_packet);

    /*
    * Inventory
    */
    server.add_to_next_tick(CharacterInitInventory(char_id));
}

pub fn handle_restart(server: &Server, context: Request) {
    let packet_restart = cast!(context.packet(), PacketCzRestart);
    let session_id = context.session().account_id;
    let mut sessions_guard = write_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let char_id = session.char_id();
    let character_ref = server.get_character_from_context_unsafe(&context);
    server.add_to_next_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap{char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance()}));
    drop(character_ref);
    let session = sessions_guard.get(&session_id).unwrap();
    let session = Arc::new(session.recreate_without_character());
    sessions_guard.insert(session_id, session);

    let mut restart_ack = PacketZcRestartAck::new();
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    socket_send!(context, restart_ack);
}

pub fn handle_disconnect(server: &Server, context: Request) {
    let session = context.session();
    let char_id = session.char_id();
    let character_ref = server.get_character_from_context_unsafe(&context);
    server.add_to_next_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap{char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance()}));
    drop(character_ref);
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

async fn load_chars_info(account_id: u32, server: &Server) -> PacketHcAcceptEnterNeoUnionHeader {
    let row_results = sqlx::query_as::<Postgres, CharacterInfoNeoUnionWrapped>("SELECT * FROM char WHERE account_id = $1")
        .bind(account_id as i32)
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