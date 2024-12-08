use std::collections::HashSet;
use std::net::Shutdown::Both;
use std::sync::{Arc, Mutex};

use byteorder::{LittleEndian, WriteBytesExt};
use models::position::Position;
use models::status::KnownSkill;


use packets::packets::{CharacterInfoNeoUnion, Packet, PacketChDeleteChar4Reserved, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar, PacketChSendMapInfo, PacketCzEnter2, PacketCzRestart, PacketHcAcceptEnterNeoUnion, PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptMakecharNeoUnion, PacketHcBlockCharacter, PacketHcDeleteChar4Reserved, PacketHcNotifyZonesvr, PacketHcRefuseEnter, PacketMapConnection, PacketPincodeLoginstate, PacketZcAcceptEnter2, PacketZcInventoryExpansionInfo, PacketZcLoadConfirm, PacketZcOverweightPercent, PacketZcReqDisconnectAck2, PacketZcRestartAck, ZserverAddr};

use crate::repository::model::char_model::{CharacterInfoNeoUnionWrapped, CharInsertModel, CharSelectModel};
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::request::Request;

use crate::server::model::events::game_event::{CharacterRemoveFromMap, GameEvent};
use crate::server::model::events::game_event::GameEvent::{CharacterInitInventory, CharacterJoinGame};
use crate::server::model::hotkey::Hotkey;
use crate::server::model::status::StatusFromDb;
use crate::server::script::ScriptGlobalVariableStore;
use crate::server::Server;
use crate::server::service::server_service::ServerService;

use crate::server::state::character::Character;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

pub fn handle_char_enter(server: &Server, context: Request) {
    let packet_char_enter = cast!(context.packet(), PacketChEnter);
    let server_state = server.state();
    let mut sessions_guard = write_lock!(server_state.sessions());

    if sessions_guard.contains_key(&packet_char_enter.aid) {
        let session = sessions_guard.get(&packet_char_enter.aid).unwrap();
        let session = Arc::new(session.recreate_with_char_socket(context.socket()));
        sessions_guard.insert(packet_char_enter.aid, session.clone());
        if session.auth_code == packet_char_enter.auth_code && session.user_level == packet_char_enter.user_level {
            let packet_hc_accept_enter_neo_union: Box<dyn Packet> = context.runtime().block_on(async {
                let mut hc_accept_enter_neo_union = load_chars_info(session.account_id, server).await;
                if GlobalConfigService::instance().packetver() >= 20130000 {
                    let mut accept_enter_neo_union_header = PacketHcAcceptEnterNeoUnionHeader::new(GlobalConfigService::instance().packetver());
                    accept_enter_neo_union_header.set_char_info(hc_accept_enter_neo_union);
                    accept_enter_neo_union_header.set_char_slot(12);
                    accept_enter_neo_union_header.set_premium_slot_end(12);
                    accept_enter_neo_union_header.set_premium_slot_start(12);
                    accept_enter_neo_union_header.set_packet_len(29);
                    accept_enter_neo_union_header.fill_raw_with_packetver(Some(server.packetver()));
                    let x: Box<dyn Packet> = Box::new(accept_enter_neo_union_header);
                    x
                } else {
                    hc_accept_enter_neo_union.fill_raw_with_packetver(Some(server.packetver()));
                    let x: Box<dyn Packet> = Box::new(hc_accept_enter_neo_union);
                    x
                }
            });
            let mut pincode_loginstate = PacketPincodeLoginstate::new(GlobalConfigService::instance().packetver());
            pincode_loginstate.set_aid(session.account_id);
            pincode_loginstate.set_pincode_seed(session.auth_code);
            pincode_loginstate.fill_raw();
            let mut packet_hc_block_character = PacketHcBlockCharacter::new(GlobalConfigService::instance().packetver());
            packet_hc_block_character.set_packet_length(PacketHcBlockCharacter::base_len(GlobalConfigService::instance().packetver()) as i16);
            packet_hc_block_character.fill_raw();
            // The pincode packet should be appended to PacketHcAcceptEnterNeoUnionHeader packet
            let final_response_packet: Vec<u8> = chain_packets(vec![packet_hc_accept_enter_neo_union.as_ref(), &packet_hc_block_character]);
            let mut wtr = vec![];
            // A "account id packet" should be sent just before char info packet
            wtr.write_u32::<LittleEndian>(session.account_id).expect("Unable to write Little endian u32 from session account id");
            socket_send_raw!(context, wtr);
            socket_send_raw!(context, final_response_packet);
            return;
        }
        // should not happen, but in case of forged packet, remove session
        server.state().remove_session(packet_char_enter.aid);
    }
    let mut res = PacketHcRefuseEnter::new(GlobalConfigService::instance().packetver());
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
        let vit = 5_i16;
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
            str: 5_i16,
            agi: 5_i16,
            vit,
            int,
            dex: 5_i16,
            luk: 5_i16,
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
            hair: packet_make_char.head,
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
        let name = char_model.name.as_str();
        server.repository.character_insert(&char_model).await.unwrap();
        // TODO add default stuff
        let created_char: CharacterInfoNeoUnionWrapped =  server.repository.character_info(char_model.account_id, name).await.unwrap();
        created_char.data
    });
    let mut packet_hc_accept_makechar_neo_union = PacketHcAcceptMakecharNeoUnion::new(GlobalConfigService::instance().packetver());
    packet_hc_accept_makechar_neo_union.set_charinfo(created_char);
    packet_hc_accept_makechar_neo_union.fill_raw();
    socket_send!(context, packet_hc_accept_makechar_neo_union);
}

pub fn handle_delete_reserved_char(server: &Server, context: Request) {
    let packet_delete_reserved_char = cast!(context.packet(), PacketChDeleteChar4Reserved);
    context.runtime().block_on(async {
        server.repository.character_delete_reserved(context.session().account_id, packet_delete_reserved_char.gid).await.unwrap();
    });
    let mut packet_hc_delete_char4reserved = PacketHcDeleteChar4Reserved::new(GlobalConfigService::instance().packetver());
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
        server.repository.character_fetch(session_id, packet_select_char.char_num).await.unwrap()
    });
    let skills: Vec<KnownSkill> = context.runtime().block_on(async {
        server.repository.character_skills(char_model.char_id as u32).await.unwrap()
    });
    let hotkeys: Vec<Hotkey> = context.runtime().block_on(async {
        server.repository.load_hotkeys(char_model.char_id as u32).await.unwrap()
    });
    let mut sessions_guard = write_lock!(server.state().sessions());
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
        status: StatusFromDb::from_char_model(&char_model, &server.configuration.game, skills),
        loaded_from_client_side: false,
        x: last_x,
        y: last_y,
        dir: 0,
        movements: vec![],
        attack: None,
        skill_in_use: None,
        inventory: vec![], // todo load from db
        map_view: HashSet::new(),
        script_variable_store: Mutex::new(ScriptGlobalVariableStore::default()),
        account_id: session_id,
        map_instance_key: MapInstanceKey::new(last_map, 0),
        last_moved_at: 0,
        hotkeys,
    };
    let char_id = character.char_id;
    let session = Arc::new(context.session().recreate_with_character(char_id));
    let mut map_name = [0 as char; 16];
    character.current_map_name().fill_char_array(map_name.as_mut());
    server.state_mut().insert_character(character);
    sessions_guard.insert(session_id, session);
    if server.packetver() < 20170329 {
        let mut packet_ch_send_map_info = PacketHcNotifyZonesvr::new(GlobalConfigService::instance().packetver());
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        let mut zserver_addr = ZserverAddr::new(GlobalConfigService::instance().packetver());
        zserver_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        zserver_addr.set_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_addr(zserver_addr);
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    } else {
        let mut packet_ch_send_map_info = PacketChSendMapInfo::new(GlobalConfigService::instance().packetver());
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
    let mut sessions_guard = write_lock!(server.state().sessions());
    let session = sessions_guard.get(&aid);
    if session.is_none() {
        write_lock!(context.socket()).shutdown(Both).expect("Unable to shutdown incoming socket. Shutdown was done because session does not exists");
        return;
    }
    let session = session.unwrap();
    if auth_code != session.auth_code {
        write_lock!(context.socket()).shutdown(Both).expect("Unable to shutdown incoming socket. Shutdown was done because packet auth_code mismatching session auth_code");
        server.state().remove_session(aid);
        return;
    }
    let session = Arc::new(session.recreate_with_map_socket(context.socket()));
    sessions_guard.insert(aid, session.clone());
    let mut packet_map_connection = PacketMapConnection::new(GlobalConfigService::instance().packetver());
    packet_map_connection.set_aid(session.account_id);

    socket_send!(context, packet_map_connection);

    /*
    Client expect multiple packets in response to packet PacketCzEnter2
    */
    let mut packet_inventory_expansion_info = PacketZcInventoryExpansionInfo::new(GlobalConfigService::instance().packetver());
    packet_inventory_expansion_info.fill_raw();
    let mut packet_overweight_percent = PacketZcOverweightPercent::new(GlobalConfigService::instance().packetver());
    packet_overweight_percent.fill_raw();
    let char_id = session.char_id();
    let character = server.state().get_character_unsafe(char_id);
    let mut packet_accept_enter = PacketZcAcceptEnter2::new(GlobalConfigService::instance().packetver());
    packet_accept_enter.set_start_time(get_tick_client());
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.set_pos_dir(Position { x: character.x(), y: character.y(), dir: character.dir() }.to_pos());
    packet_accept_enter.fill_raw();

    server.add_to_next_tick(CharacterJoinGame(char_id));
    server.server_service.schedule_warp_to_walkable_cell(server.state_mut().as_mut(), &Map::name_without_ext(character.current_map_name()), character.x(), character.y(), session.char_id());
    socket_send!(context, packet_accept_enter);


    /*
    * Inventory
    */
    server.add_to_next_tick(CharacterInitInventory(char_id));
}

pub fn handle_restart(server: &Server, context: Request) {
    let packet_restart = cast!(context.packet(), PacketCzRestart);
    let session_id = context.session().account_id;
    let mut sessions_guard = write_lock!(server.state().sessions());
    let session = sessions_guard.get(&session_id).unwrap();
    let char_id = session.char_id();
    let character_ref = server.state().get_character_from_context_unsafe(&context);
    server.add_to_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap { char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance() }), 1);
    server.add_to_tick(GameEvent::CharacterLeaveGame(char_id),2);
    let session = sessions_guard.get(&session_id).unwrap();
    let session = Arc::new(session.recreate_without_character());
    sessions_guard.insert(session_id, session);

    let mut restart_ack = PacketZcRestartAck::new(GlobalConfigService::instance().packetver());
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    socket_send!(context, restart_ack);
}

pub fn handle_disconnect(server: &Server, context: Request) {
    let session = context.session();
    let char_id = session.char_id();
    let character_ref = server.state().get_character_from_context_unsafe(&context);
    server.add_to_next_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap { char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance() }));
    server.state().remove_session(session.account_id);

    let mut disconnect_ack = PacketZcReqDisconnectAck2::new(GlobalConfigService::instance().packetver());
    disconnect_ack.fill_raw();
    socket_send!(context, disconnect_ack);
}


pub fn handle_blocking_play_cancel(context: Request) {
    let mut packet_zc_load_confirm = PacketZcLoadConfirm::new(GlobalConfigService::instance().packetver());
    packet_zc_load_confirm.fill_raw();
    socket_send!(context, packet_zc_load_confirm);
}

async fn load_chars_info(account_id: u32, server: &Server) -> PacketHcAcceptEnterNeoUnion {
    let row_results = server.repository.characters_info(account_id).await;
    let mut accept_enter_neo_union = PacketHcAcceptEnterNeoUnion::new(GlobalConfigService::instance().packetver());
    accept_enter_neo_union.set_packet_length((27 + row_results.len() * CharacterInfoNeoUnion::base_len(server.packetver())) as i16);
    accept_enter_neo_union.set_char_info(row_results.iter().map(|wrapped| {
        wrapped.data.clone()
    }).collect::<Vec<CharacterInfoNeoUnion>>());
    accept_enter_neo_union.set_premium_start_slot(12);
    accept_enter_neo_union.set_premium_end_slot(12);
    accept_enter_neo_union.set_total_slot_num(12);
    accept_enter_neo_union
}