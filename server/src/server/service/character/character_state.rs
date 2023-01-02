use std::sync::mpsc::SyncSender;
use std::sync::Once;
use tokio::runtime::Runtime;
use enums::status::StatusTypes;
use packets::packets::{Packet, PacketZcLongparChange, PacketZcNpcackMapmove, PacketZcParChange, PacketZcSpriteChange2};
use crate::server::events::game_event::{CharacterChangeMap, CharacterLook, CharacterRemoveFromMap, CharacterZeny, GameEvent};
use crate::server::core::map::{Map, MAP_EXT, RANDOM_CELL};
use crate::server::core::position::Position;
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::events::persistence_event::{PersistenceEvent, SavePositionUpdate, StatusUpdate};
use crate::server::events::persistence_event::PersistenceEvent::SaveCharacterPosition;
use crate::server::map_item::ToMapItem;
use crate::server::Server;

use crate::server::state::character::Character;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;

static mut SERVICE_INSTANCE: Option<CharacterService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct CharacterService {}

impl CharacterService {
    pub fn instance() -> &'static CharacterService {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(CharacterService::new());
        });
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    fn new() -> Self {
        Self {}
    }

    pub fn schedule_warp_to_walkable_cell(&self, destination_map: &str, x: u16, y: u16, char_id: u32, server: &Server) {
        server.add_to_next_tick(GameEvent::CharacterClearFov(char_id));
        let character_ref = server.get_character_unsafe(char_id);
        server.add_to_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap { char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance() }), 0);
        drop(character_ref);

        let map_name: String = Map::name_without_ext(destination_map);
        debug!("Char enter on map {}", map_name);
        let map_ref = server.maps.get(&map_name).unwrap();
        let map_instance = map_ref.player_join_map(server);
        let (x, y) = if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
            let walkable_cell = Map::find_random_walkable_cell(&map_instance.cells, map_instance.x_size);
            (walkable_cell.0, walkable_cell.1)
        } else {
            (x, y)
        };

        server.add_to_tick(GameEvent::CharacterChangeMap(CharacterChangeMap {
            char_id,
            new_map_name: destination_map.to_owned(),
            new_instance_id: map_instance.id,
            new_position: Some(Position { x, y, dir: 3 }),
        }), 2);
    }

    pub fn change_map(&self, server_ref: &Server, client_notification_sender_clone: &SyncSender<Notification>, persistence_event_sender: &SyncSender<PersistenceEvent>, event: &CharacterChangeMap, character: &mut Character) {
        if let Some(map_instance) = server_ref.get_map_instance(&event.new_map_name, event.new_instance_id) {
            character.join_and_set_map(map_instance);
            let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

            let mut new_current_map: [char; 16] = [0 as char; 16];
            let map_name = format!("{}{}", event.new_map_name, MAP_EXT);
            map_name.fill_char_array(new_current_map.as_mut());
            packet_zc_npcack_mapmove.set_map_name(new_current_map);
            let new_position = event.new_position.unwrap();
            packet_zc_npcack_mapmove.set_x_pos(new_position.x as i16);
            packet_zc_npcack_mapmove.set_y_pos(new_position.y as i16);
            packet_zc_npcack_mapmove.fill_raw();
            client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
                .expect("Failed to send notification event with PacketZcNpcackMapmove");

            server_ref.insert_map_item(character.account_id, character.to_map_item());
            character.update_position(new_position.x, new_position.y);
            character.clear_map_view();
            character.loaded_from_client_side = false;
            server_ref.add_to_next_tick(GameEvent::CharacterInitInventory(character.char_id));
            persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() }))
                .expect("Fail to send persistence notification");
        } else {
            error!("Can't change map to {} {}", event.new_map_name, event.new_instance_id);
        }
    }

    pub fn change_look(&self, server_ref: &Server, persistence_event_sender: &SyncSender<PersistenceEvent>, character_look: CharacterLook, character: &mut Character) {
        let db_column = character.change_look(character_look.look_type, character_look.look_value);
        if let Some(db_column) = db_column {
            let mut packet_zc_sprite_change = PacketZcSpriteChange2::new();
            packet_zc_sprite_change.set_gid(character_look.char_id);
            packet_zc_sprite_change.set_atype(character_look.look_type.value() as u8);
            packet_zc_sprite_change.set_value(character_look.look_value as i32);
            packet_zc_sprite_change.fill_raw();
            server_ref.client_notification_sender().send(Notification::Area(AreaNotification {
                map_name: character.current_map_name().clone(),
                map_instance_id: character.current_map_instance(),
                range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y() },
                packet: std::mem::take(packet_zc_sprite_change.raw_mut()),
            })).expect("Fail to send client notification");
            persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character_look.char_id, db_column, value: character_look.look_value })).expect("Fail to send persistence notification");
        }
    }

    pub fn update_zeny(&self, server_ref: &Server, persistence_event_sender: &SyncSender<PersistenceEvent>, runtime: &Runtime, zeny_update: CharacterZeny, character: &mut Character) {
        let zeny = if let Some(zeny) = zeny_update.zeny {
            persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: zeny_update.char_id, value: zeny, db_column: "zeny".to_string() })).expect("Fail to send persistence notification");
            zeny
        } else {
            runtime.block_on(async {
                server_ref.repository.character_zeny_fetch(zeny_update.char_id).await.expect("failed to fetch zeny") as u32
            })
        };
        character.change_zeny(zeny);

        let mut packet_zc_longpar_change = PacketZcLongparChange::new();
        packet_zc_longpar_change.set_amount(character.get_zeny() as i32);
        packet_zc_longpar_change.set_var_id(StatusTypes::Zeny.value() as u16);
        packet_zc_longpar_change.fill_raw();
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_longpar_change.raw_mut())))).expect("Fail to send client notification");
    }

    pub fn update_weight(&self, server_ref: &Server, character: &mut Character) {
        let mut packet_weight = PacketZcParChange::new();
        packet_weight.set_var_id(StatusTypes::Weight.value() as u16);
        packet_weight.set_count(character.weight() as i32);
        packet_weight.fill_raw();
        let mut packet_max_weight = PacketZcParChange::new();
        packet_max_weight.set_var_id(StatusTypes::Maxweight.value() as u16);
        packet_max_weight.set_count(character.max_weight() as i32);
        packet_max_weight.fill_raw();
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, chain_packets(vec![&packet_weight, &packet_max_weight])))).expect("Fail to send client notification");
    }
}