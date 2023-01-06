use std::sync::{Arc};
use std::sync::mpsc::SyncSender;
use std::thread::{sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tokio::runtime::Runtime;

use packets::packets::{Packet, PacketZcNotifyPlayermove, PacketZcReqWearEquipAck};
use crate::PersistenceEvent;
use crate::PersistenceEvent::SaveCharacterPosition;
use crate::server::core::movement::Movement;
use crate::server::events::game_event::{GameEvent};

use crate::server::events::map_event::MapEvent::{*};
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::persistence_event::{SavePositionUpdate};


use crate::server::map_item::{ToMapItemSnapshot};

use crate::server::Server;
use crate::server::service::character::character_service::{CharacterService};
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::item_service::{ItemService};
use crate::server::service::status_service::StatusService;


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
                            CharacterService::instance().change_map(&server_ref, &client_notification_sender_clone, &persistence_event_sender, &event, character)
                        }
                        GameEvent::CharacterRemoveFromMap(character_remove_from_map) => {
                            let character = characters.get_mut(&character_remove_from_map.char_id).unwrap();
                            character.movements = vec![];
                            if let Some(instance) = server_ref.get_map_instance(&character_remove_from_map.map_name, character_remove_from_map.instance_id) {
                                instance.remove_item_with_id(character_remove_from_map.char_id);
                            }
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
                            CharacterService::instance().change_look(&server_ref, &persistence_event_sender, character_look, character)
                        }
                        GameEvent::CharacterUpdateZeny(zeny_update) => {
                            let character = characters.get_mut(&zeny_update.char_id).unwrap();
                            CharacterService::instance().update_zeny(&server_ref, &persistence_event_sender, &runtime, zeny_update, character);
                        }
                        GameEvent::CharacterAddItems(add_items) => {
                            let character = characters.get_mut(&add_items.char_id).unwrap();
                            InventoryService::instance().add_items_in_inventory(&server_ref, &runtime, add_items, character);
                        }
                        GameEvent::CharacterInitInventory(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            InventoryService::instance().reload_inventory(&server_ref, &runtime, char_id, character);
                            InventoryService::instance().reload_equipped_item_sprites(&server_ref, character);
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterUpdateWeight(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            CharacterService::instance().update_weight(&server_ref, character);
                        }
                        GameEvent::CharacterUseItem(character_user_item) => {
                            let character = characters.get_mut(&character_user_item.char_id).unwrap();
                            ItemService::instance().use_item(server_ref.clone(), &runtime, &persistence_event_sender, character_user_item, character);
                        }
                        GameEvent::CharacterAttack(character_attack) => {
                            let character = characters.get_mut(&character_attack.char_id).unwrap();
                            if !character.is_attacking() {
                                character.set_attack(character_attack.target_id, character_attack.repeat, 0);
                            }
                        }
                        GameEvent::CharacterEquipItem(character_equip_item) => {
                            let character = characters.get_mut(&character_equip_item.char_id).unwrap();
                            let index = character_equip_item.index;
                            InventoryService::instance().equip_item(&server_ref, character, &persistence_event_sender, character_equip_item);
                            character.get_item_from_inventory(index)
                                .map(|item| InventoryService::instance().sprite_change_packet_for_item(character, item)
                                    .map(|packet| CharacterService::instance().send_area_notification_around_characters(&server_ref, character, packet)));
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterTakeoffEquipItem(character_takeoff_equip_item) => {
                            let character = characters.get_mut(&character_takeoff_equip_item.char_id).unwrap();
                            let index = character_takeoff_equip_item.index;
                            InventoryService::instance().takeoff_equip_item(&server_ref, character, &persistence_event_sender, index);
                            character.get_item_from_inventory(index)
                                .map(|item| InventoryService::instance().sprite_change_packet_for_item(character, item)
                                    .map(|packet| CharacterService::instance().send_area_notification_around_characters(&server_ref, character, packet)));
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                        GameEvent::CharacterCalculateStats(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            StatusService::instance().calculate_status(&server_ref, character);
                        }
                    }
                }
            }
            for (_, character) in characters.iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
                CharacterService::instance().load_units_in_fov(server_ref.as_ref(), client_notification_sender_clone.clone(), character);
                CharacterService::instance().attack(server_ref.as_ref(), client_notification_sender_clone.clone(), character, tick);
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
            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (GAME_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!("Less than 5 milliseconds of sleep, game loop is too slow - {}ms because game loop took {}ms", sleep_duration, time_spent);
            }
            sleep(Duration::from_millis(sleep_duration));
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
                        character.clear_attack();
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
                        info!("move {} at {}", movement.position(), movement.move_at());
                        let movement = character.pop_movement().unwrap();
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.get_map_instance_from_character(character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                CharacterService::instance().schedule_warp_to_walkable_cell(warp.dest_map_name.as_str(), warp.to_x, warp.to_y, character.char_id, server_ref.as_ref());
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

            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (MOVEMENT_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!("Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop took {}ms", sleep_duration, time_spent);
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }
}

