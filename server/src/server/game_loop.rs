use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use rand::RngCore;
use tokio::runtime::Runtime;


use packets::packets::{Packet, PacketZcItemPickupAck3, PacketZcLongparChange, PacketZcNotifyPlayermove, PacketZcNpcackMapmove, PacketZcSpriteChange2};
use crate::PersistenceEvent;
use crate::PersistenceEvent::SaveCharacterPosition;


use crate::server::core::movement::Movement;
use crate::server::events::game_event::{CharacterChangeMap, GameEvent};
use crate::server::core::map::{MAP_EXT};
use crate::server::events::map_event::MapEvent::{*};
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::events::persistence_event::{InventoryItemUpdate, SavePositionUpdate, StatusUpdate};
use crate::server::core::position::Position;
use crate::server::enums::status::StatusTypes;

use crate::server::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::Server;
use crate::util::packet::{chain_packets, chain_packets_raws, chain_packets_raws_by_value};
use crate::util::string::StringUtil;


const MOVEMENT_TICK_RATE: u128 = 20;
const GAME_TICK_RATE: u128 = 40;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, runtime: Runtime) {
        let mut last_mobs_action = Instant::now();
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_task() {
                for task in tasks {
                    match task {
                        GameEvent::CharacterChangeMap(event) => {
                            let character = characters.get_mut(&event.char_id).unwrap();
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
                                persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() }))
                                    .expect("Fail to send persistence notification");
                            } else {
                                error!("Can't change map to {} {}", event.new_map_name, event.new_instance_id);
                            }
                        }
                        GameEvent::CharacterRemoveFromMap(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.movements = vec![];
                        }
                        GameEvent::CharacterClearFov(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.clear_map_view();
                        }
                        GameEvent::CharacterLoadedFromClientSide(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.loaded_from_client_side = true;
                            character.clear_map_view();
                        }
                        GameEvent::CharacterMove(_) => {
                            // handled by dedicated thread
                        }
                        GameEvent::CharacterUpdateLook(character_look) => {
                            let character = characters.get_mut(&character_look.char_id).unwrap();
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
                                persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate {
                                    char_id: character_look.char_id,
                                    db_column,
                                    value: character_look.look_value,
                                })).expect("Fail to send persistence notification");
                            }
                        }
                        GameEvent::CharacterUpdateZeny(zeny_update) => {
                            let character = characters.get_mut(&zeny_update.char_id).unwrap();
                            character.change_zeny(zeny_update.zeny);
                            persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate {
                                char_id: zeny_update.char_id,
                                value: zeny_update.zeny,
                                db_column: "zeny".to_string(),
                            })).expect("Fail to send persistence notification");
                            let mut packet_zc_longpar_change = PacketZcLongparChange::new();
                            packet_zc_longpar_change.set_amount(character.get_zeny() as i32);
                            packet_zc_longpar_change.set_var_id(StatusTypes::Zeny.value() as u16);
                            packet_zc_longpar_change.fill_raw();
                            server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_longpar_change.raw_mut())))).expect("Fail to send client notification");

                        }
                        GameEvent::CharacterAddItems(add_items) => {
                            let character = characters.get_mut(&add_items.char_id).unwrap();
                            runtime.block_on(async {
                                let mut rng = rand::thread_rng();
                                let inventory_item_updates = add_items.items.iter().map(|item| {
                                    if item.item_type.is_stackable() {
                                        InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i16, amount: item.amount as i16, stackable: true, identified: item.is_identified, unique_id: 0}
                                    } else {
                                        InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i16, amount: item.amount as i16, stackable: false, identified: item.is_identified, unique_id: rng.next_u32() as i64}
                                    }
                                }).collect();
                                let result = server_ref.repository.character_inventory_update(&inventory_item_updates).await;
                                if result.is_ok() {
                                    let mut packets = vec![];
                                    character.add_items(add_items.items).iter().for_each(|(index, item)| {
                                        let mut packet_zc_item_pickup_ack3 = PacketZcItemPickupAck3::new();
                                        packet_zc_item_pickup_ack3.set_itid(item.item_id as u16);
                                        packet_zc_item_pickup_ack3.set_count(item.amount as u16);
                                        packet_zc_item_pickup_ack3.set_index(*index as u16);
                                        packet_zc_item_pickup_ack3.set_is_identified(item.is_identified);
                                        packet_zc_item_pickup_ack3.set_atype(item.item_type.value() as u8);
                                        packet_zc_item_pickup_ack3.fill_raw();
                                        packet_zc_item_pickup_ack3.pretty_debug();
                                        packets.push(packet_zc_item_pickup_ack3)
                                    });
                                    // TODO weight to update on client. character.weight
                                    server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id,
                                                                                                                        chain_packets_raws_by_value(packets.iter().map(|packet| packet.raw.clone()).collect()))))
                                        .expect("Fail to send client notification");
                                }
                            });
                        }
                    }
                }
            }
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
                character.load_units_in_fov(server_ref.as_ref(), client_notification_sender_clone.clone())
            }
            for (_, map) in server_ref.maps.iter() {
                for instance in map.instances() {
                    instance.notify_event(SpawnMobs);
                    instance.notify_event(UpdateMobsFov(characters.iter().map(|(_, character)| character.to_map_item_snapshot()).collect()));
                    if last_mobs_action.elapsed().as_secs() > 2 {
                        instance.notify_event(MobsActions);
                        last_mobs_action = Instant::now();
                    }
                }
            }
            sleep(Duration::from_millis((GAME_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(GAME_TICK_RATE)) as u64));
        }
    }

    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>) {
        loop {
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut characters = server_ref.characters.borrow_mut();
            if let Some(tasks) = server_ref.pop_movement_task() {
                for task in tasks {
                    if let GameEvent::CharacterMove(character_movement) = task {
                        let character = characters.get_mut(&character_movement.char_id).unwrap();
                        let speed = character.status.speed;
                        let maybe_previous_movement = character.pop_movement();
                        character.set_movement(character_movement.path);
                        let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
                        if let Some(movement) = character.peek_mut_movement() {
                            if let Some(previous_movement) = maybe_previous_movement {
                                debug!("change path! was {} will {}, move at {}",previous_movement.position(), movement.position(), previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                // movement.set_move_at(previous_movement.move_at() + Movement::delay(speed, movement.is_diagonal()));
                                movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()) + MOVEMENT_TICK_RATE);
                            } else {
                                movement.set_move_at(tick + Movement::delay(speed, movement.is_diagonal()));
                                debug!("will move at {}", movement.move_at());
                            }
                            packet_zc_notify_playermove.set_move_start_time(movement.move_at() as u32); // todo: time conversion check on client side ???
                        }

                        packet_zc_notify_playermove.set_move_data(character_movement.current_position.to_move_data(&character_movement.destination));
                        packet_zc_notify_playermove.fill_raw();
                        client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_notify_playermove.raw_mut()))))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                    }
                }
            }

            // If movement not smooth:
            // teleport in front -> server movement faster than client movement
            // teleport back -> server movement slower than client movement
            let mut character_finished_to_move = vec![];
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.is_moving()) {
                let speed = character.status.speed;
                if let Some(movement) = character.peek_movement() {
                    if tick >= movement.move_at() {
                        debug!("move {} at {}", movement.position(), movement.move_at());
                        let movement = character.pop_movement().unwrap();
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.get_map_instance_from_character(character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                server_ref.add_to_next_tick(GameEvent::CharacterChangeMap(CharacterChangeMap {
                                    char_id: character.char_id,
                                    new_map_name: warp.dest_map_name.clone(),
                                    new_instance_id: 0,
                                    new_position: Some(Position { x: warp.to_x, y: warp.to_y, dir: movement.position().dir }),
                                }));
                                character.clear_movement();
                                continue;
                            }
                        }
                        if let Some(next_movement) = character.peek_mut_movement() {
                            next_movement.set_move_at(tick + Movement::delay(speed, next_movement.is_diagonal()))
                        } else {
                            character_finished_to_move.push(character);
                        }
                    }
                }
            }
            for character in character_finished_to_move {
                persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() })).expect("Fail to send persistence notification");
            }
            sleep(Duration::from_millis((MOVEMENT_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(MOVEMENT_TICK_RATE)) as u64));
        }
    }
}

