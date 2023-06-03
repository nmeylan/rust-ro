use std::sync::Arc;
use crate::packets::packets::Packet;
use packets::packets::{PacketCaLogin, PacketChDeleteChar4Reserved, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar, PacketCzAckSelectDealtype, PacketCzBlockingPlayCancel, PacketCzChooseMenu, PacketCzContactnpc, PacketCzEnter2, PacketCzInputEditdlg, PacketCzInputEditdlgstr, PacketCzItemPickup, PacketCzItemThrow, PacketCzNotifyActorinit, PacketCzPcPurchaseItemlist, PacketCzPcSellItemlist, PacketCzPlayerChat, PacketCzReqDisconnect2, PacketCzReqname, PacketCzReqnameall2, PacketCzReqNextScript, PacketCzReqTakeoffEquip, PacketCzRequestAct, PacketCzRequestMove, PacketCzRequestMove2, PacketCzRequestTime, PacketCzReqWearEquip, PacketCzRestart, PacketCzStatusChange, PacketCzUpgradeSkilllevel, PacketCzUseItem, PacketUnknown, PacketZcNotifyTime};
use crate::server::model::request::Request;
use crate::server::request_handler::action::action::{handle_action, handle_pickup_item};
use crate::server::request_handler::action::character::{handle_player_skill_allocation, handle_player_status_change};
use crate::server::request_handler::action::item::{handle_player_drop_item, handle_player_equip_item, handle_player_takeoff_equip_item, handle_player_use_item};
use crate::server::request_handler::action::npc::{handle_contact_npc, handle_player_choose_menu, handle_player_input_number, handle_player_input_string, handle_player_next, handle_player_purchase_items, handle_player_select_deal_type, handle_player_sell_items};
use crate::server::request_handler::char::{handle_blocking_play_cancel, handle_char_enter, handle_delete_reserved_char, handle_disconnect, handle_enter_game, handle_make_char, handle_restart, handle_select_char};
use crate::server::request_handler::chat::handle_chat;
use crate::server::request_handler::login::handle_login;
use crate::server::request_handler::map::{handle_char_loaded_client_side, handle_map_item_name};
use crate::server::request_handler::movement::handle_char_move;
use crate::server::Server;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::tick::get_tick_client;

/**
* This module implement client requests handler.
*/
pub mod char;
pub mod login;
pub mod movement;
pub mod action;
pub mod map;
pub mod atcommand;
pub mod chat;


pub fn handle(server: Arc<Server>, mut context: Request) {
    if context.packet().as_any().downcast_ref::<PacketUnknown>().is_some() {
        error!("Unknown packet {} of length {}: {:02X?}", context.packet().id(GlobalConfigService::instance().packetver()), context.packet().raw().len(), context.packet().raw());
        return;
    }
    // Login
    if context.packet().as_any().downcast_ref::<PacketCaLogin>().is_some() {
        debug!("PacketCaLogin");
        return handle_login(server, context);
    }
    // Char selection
    if context.packet().as_any().downcast_ref::<PacketChEnter>().is_some() {
        debug!("PacketChEnter");
        return handle_char_enter(server.as_ref(), context);
    }

    // Enter game
    if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
        debug!("PacketCzEnter2");
        // A char session exist, but not yet map session
        return handle_enter_game(server.as_ref(), context);
    }
    /*
     *  Having a session is required for any packets below
     */
    let session_id = server.ensure_session_exists(&context.socket());
    if session_id.is_none() {
        return;
    }
    let session = server.state().get_session(session_id.unwrap());
    context.set_session(session);
    // Char creation
    if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
        debug!("PacketChMakeChar");
        return handle_make_char(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
        debug!("PacketChMakeChar2");
        return handle_make_char(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
        debug!("PacketChMakeChar3");
        return handle_make_char(server.as_ref(), context);
    }
    // Delete char reservation
    if context.packet().as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
        debug!("PacketChDeleteChar4Reserved");
        return handle_delete_reserved_char(server.as_ref(), context);
    }
    // Select char
    if context.packet().as_any().downcast_ref::<PacketChSelectChar>().is_some() {
        debug!("PacketChSelectChar");
        return handle_select_char(server.as_ref(), context);
    }
    // Game menu "Character select"
    if context.packet().as_any().downcast_ref::<PacketCzRestart>().is_some() {
        debug!("PacketCzRestart");
        return handle_restart(server.as_ref(), context);
    }
    // Game menu "Exit to windows"
    if context.packet().as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some() {
        debug!("PacketCzReqDisconnect2");
        return handle_disconnect(server.as_ref(), context);
    }
    // Player click on map cell
    if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
        debug!("PacketCzRequestMove2");
        return handle_char_move(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzRequestMove>().is_some() {
        debug!("PacketCzRequestMove");
        return handle_char_move(server.as_ref(), context);
    }
    // Client notify player has been loaded
    if context.packet().as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
        debug!("PacketCzNotifyActorinit");
        return handle_char_loaded_client_side(server.as_ref(), context);
    }
    // Client send PACKET_CZ_BLOCKING_PLAY_CANCEL after char has loaded
    if context.packet().as_any().downcast_ref::<PacketCzBlockingPlayCancel>().is_some() {
        debug!("PacketCzBlockingPlayCancel");
        return handle_blocking_play_cancel(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzRequestAct>().is_some() {
        debug!("PacketCzRequestAct");
        return handle_action(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzItemPickup>().is_some() {
        debug!("PacketCzItemPickup");
        return handle_pickup_item(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
        debug!("PacketCzReqnameall2");
        return handle_map_item_name(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
        debug!("PacketCzReqname");
        return handle_map_item_name(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzPlayerChat>().is_some() {
        debug!("PacketCzPlayerChat");
        return handle_chat(server.as_ref(), context);
    }

    // NPC interactions
    if context.packet().as_any().downcast_ref::<PacketCzContactnpc>().is_some() {
        debug!("PacketCzContactnpc");
        return handle_contact_npc(server, context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqNextScript>().is_some() {
        debug!("PacketCzReqNextScript");
        return handle_player_next(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzChooseMenu>().is_some() {
        debug!("PacketCzChooseMenu");
        return handle_player_choose_menu(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzInputEditdlg>().is_some() {
        debug!("PacketCzInputEditdlg");
        return handle_player_input_number(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzInputEditdlgstr>().is_some() {
        debug!("PacketCzInputEditdlgstr");
        return handle_player_input_string(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzAckSelectDealtype>().is_some() {
        debug!("PacketCzAckSelectDealtype");
        return handle_player_select_deal_type(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzPcPurchaseItemlist>().is_some() {
        debug!("PacketCzPcPurchaseItemlist");
        return handle_player_purchase_items(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzPcSellItemlist>().is_some() {
        debug!("PacketCzPcSellItemlist");
        return handle_player_sell_items(context);
    }
    // End NPC interaction

    // Item interaction
    if context.packet().as_any().downcast_ref::<PacketCzUseItem>().is_some() {
        debug!("PacketCzUseItem");
        return handle_player_use_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqWearEquip>().is_some() {
        debug!("PacketCzReqWearEquip");
        return handle_player_equip_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqTakeoffEquip>().is_some() {
        debug!("PacketCzReqTakeoffEquip");
        return handle_player_takeoff_equip_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzItemThrow>().is_some() {
        debug!("PacketCzItemThrow");
        return handle_player_drop_item(server.as_ref(), context);
    }
    // End Item interaction

    // Stats
    if context.packet().as_any().downcast_ref::<PacketCzStatusChange>().is_some() {
        debug!("PacketCzStatusChange");
        return handle_player_status_change(server.as_ref(), context);
    }
    // End stats
    // Skills
    if context.packet().as_any().downcast_ref::<PacketCzUpgradeSkilllevel>().is_some() {
        debug!("PacketCzUpgradeSkilllevel");
        return handle_player_skill_allocation(server.as_ref(), context);
    }
    // End Skills

    if context.packet().as_any().downcast_ref::<PacketCzRequestTime>().is_some() {
        let mut packet_zc_notify_time = PacketZcNotifyTime::new(GlobalConfigService::instance().packetver());
        packet_zc_notify_time.set_time(get_tick_client());
        packet_zc_notify_time.fill_raw();
        socket_send!(context, packet_zc_notify_time);
        return;
    }

    if context.packet().id(GlobalConfigService::instance().packetver()) == "0x6003" // PacketCzRequestTime2
        || context.packet().id(GlobalConfigService::instance().packetver()) == "0x187" // PacketPing
    {
        // TODO handle those packets
        return;
    }
    context.packet().display();
    context.packet().pretty_debug();
}