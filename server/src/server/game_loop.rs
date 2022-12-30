use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use rand::RngCore;
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use crate::ScriptHandler;


use packets::packets::{EquipmentitemExtrainfo301, EQUIPSLOTINFO, NormalitemExtrainfo3, Packet, PacketZcEquipmentItemlist3, PacketZcItemPickupAck3, PacketZcLongparChange, PacketZcNormalItemlist3, PacketZcNotifyPlayermove, PacketZcNpcackMapmove, PacketZcParChange, PacketZcPcPurchaseResult, PacketZcSpriteChange2};
use crate::PersistenceEvent;
use crate::PersistenceEvent::SaveCharacterPosition;


use crate::server::core::movement::Movement;
use crate::server::events::game_event::{CharacterZeny, GameEvent};
use crate::server::core::map::{MAP_EXT};
use crate::server::events::map_event::MapEvent::{*};
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::events::persistence_event::{InventoryItemUpdate, SavePositionUpdate, StatusUpdate};
use crate::server::enums::status::StatusTypes;
use crate::server::events::game_event::GameEvent::{CharacterUpdateWeight, CharacterUpdateZeny};

use crate::server::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::Server;
use crate::server::service::character_movement::change_map_packet;
use crate::server::service::item::{ItemService};
use crate::util::packet::{chain_packets, chain_packets_raws_by_value};
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
                                server_ref.add_to_next_tick(GameEvent::CharacterInitInventory(character.char_id));
                                persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() }))
                                    .expect("Fail to send persistence notification");
                            } else {
                                error!("Can't change map to {} {}", event.new_map_name, event.new_instance_id);
                            }
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
                        GameEvent::CharacterUpdateZeny(zeny_update) => {
                            let character = characters.get_mut(&zeny_update.char_id).unwrap();
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
                        GameEvent::CharacterAddItems(add_items) => {
                            let character = characters.get_mut(&add_items.char_id).unwrap();
                            runtime.block_on(async {
                                let mut rng = rand::thread_rng();
                                let inventory_item_updates: Vec<InventoryItemUpdate> = add_items.items.iter().map(|item| {
                                    if item.item_type.is_stackable() {
                                        InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: true, identified: item.is_identified, unique_id: 0 }
                                    } else {
                                        InventoryItemUpdate { char_id: add_items.char_id as i32, item_id: item.item_id as i32, amount: item.amount as i16, stackable: false, identified: item.is_identified, unique_id: rng.next_u32() as i64 }
                                    }
                                }).collect();
                                let result = server_ref.repository.character_inventory_update(&inventory_item_updates, add_items.buy).await;
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
                                    let mut packets_raws_by_value = chain_packets_raws_by_value(packets.iter().map(|packet| packet.raw.clone()).collect());
                                    if add_items.buy {
                                        let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new();
                                        packet_zc_pc_purchase_result.set_result(0);
                                        packet_zc_pc_purchase_result.fill_raw();
                                        packets_raws_by_value.extend(packet_zc_pc_purchase_result.raw);
                                        server_ref.add_to_next_tick(CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }));
                                    }
                                    server_ref.add_to_next_tick(CharacterUpdateWeight(character.char_id));
                                    server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets_raws_by_value))).expect("Fail to send client notification");
                                } else {
                                    if add_items.buy {
                                        let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new();
                                        packet_zc_pc_purchase_result.set_result(1);
                                        packet_zc_pc_purchase_result.fill_raw();
                                        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_pc_purchase_result.raw))).expect("Fail to send client notification");
                                    }
                                    error!("{:?}", result.err());
                                }
                            });
                        }
                        GameEvent::CharacterInitInventory(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
                            character.inventory = vec![];
                            runtime.block_on(async {
                                let items = server_ref.repository.character_inventory_fetch(char_id as i32).await.unwrap();
                                character.add_items(items);
                            });
                            //PacketZcNormalItemlist3
                            let mut packet_zc_equipment_itemlist3 = PacketZcEquipmentItemlist3::new();
                            let mut equipments = vec![];
                            character.inventory_equip().iter().for_each(|(index, item)| {
                                let mut equipmentitem_extrainfo301 = EquipmentitemExtrainfo301::new();
                                equipmentitem_extrainfo301.set_itid(item.item_id as u16);
                                equipmentitem_extrainfo301.set_atype(item.item_type.value() as u8);
                                equipmentitem_extrainfo301.set_index(*index as i16);
                                equipmentitem_extrainfo301.set_is_damaged(item.is_damaged);
                                equipmentitem_extrainfo301.set_is_identified(item.is_identified);
                                equipmentitem_extrainfo301.set_location(item.equip as u16);
                                equipmentitem_extrainfo301.set_wear_state(item.equip as u16);
                                equipmentitem_extrainfo301.set_refining_level(item.refine as u8);
                                let mut equipslotinfo = EQUIPSLOTINFO::new();
                                equipslotinfo.set_card1(item.card0 as u16);
                                equipslotinfo.set_card2(item.card1 as u16);
                                equipslotinfo.set_card3(item.card2 as u16);
                                equipslotinfo.set_card4(item.card3 as u16);
                                equipmentitem_extrainfo301.set_slot(equipslotinfo);
                                equipmentitem_extrainfo301.fill_raw();
                                equipments.push(equipmentitem_extrainfo301);
                            });
                            packet_zc_equipment_itemlist3.set_packet_length((PacketZcEquipmentItemlist3::base_len(server_ref.packetver()) + equipments.len() * EquipmentitemExtrainfo301::base_len(server_ref.packetver())) as i16);
                            packet_zc_equipment_itemlist3.set_item_info(equipments);
                            packet_zc_equipment_itemlist3.fill_raw();
                            let mut packet_zc_normal_itemlist3 = PacketZcNormalItemlist3::new();
                            let mut normal_items = vec![];
                            character.inventory_normal().iter().for_each(|(index, item)| {
                                let mut extrainfo3 = NormalitemExtrainfo3::new();
                                extrainfo3.set_itid(item.item_id as u16);
                                extrainfo3.set_atype(item.item_type.to_client_type() as u8);
                                extrainfo3.set_index(*index as i16);
                                extrainfo3.set_count(item.amount);
                                extrainfo3.set_is_identified(item.is_identified);
                                extrainfo3.set_wear_state(item.equip as u16);
                                let mut equipslotinfo = EQUIPSLOTINFO::new();
                                equipslotinfo.set_card1(item.card0 as u16);
                                equipslotinfo.set_card2(item.card1 as u16);
                                equipslotinfo.set_card3(item.card2 as u16);
                                equipslotinfo.set_card4(item.card3 as u16);
                                extrainfo3.set_slot(equipslotinfo);
                                extrainfo3.fill_raw();
                                normal_items.push(extrainfo3);
                            });
                            packet_zc_normal_itemlist3.set_packet_length((PacketZcNormalItemlist3::base_len(server_ref.packetver()) + normal_items.len() * NormalitemExtrainfo3::base_len(server_ref.packetver())) as i16);
                            packet_zc_normal_itemlist3.set_item_info(normal_items);
                            packet_zc_normal_itemlist3.fill_raw();
                            server_ref.add_to_next_tick(CharacterUpdateWeight(character.char_id));
                            server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id,
                                                                                                                chain_packets(vec![&packet_zc_equipment_itemlist3, &packet_zc_normal_itemlist3]))))
                                .expect("Fail to send client notification");
                        }
                        GameEvent::CharacterUpdateWeight(char_id) => {
                            let character = characters.get_mut(&char_id).unwrap();
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
                        GameEvent::CharacterUseItem(character_user_item) => {
                            let mut character = characters.get_mut(&character_user_item.char_id).unwrap();
                            if let Some(item) = character.get_item_from_inventory(character_user_item.index) {
                                if item.item_type.is_consumable() {
                                    let maybe_script_ref = ItemService::instance().get_item_script(item.item_id, server_ref.as_ref(), &runtime);
                                    if maybe_script_ref.is_some() {
                                        let script = maybe_script_ref.as_ref().unwrap();
                                        let script_result = Vm::repl(server_ref.vm.clone(), script, Box::new(&ScriptHandler {}));
                                    }
                                    // TODO handle target not self: scroll
                                    // TODO check map permission
                                    // TODO check if char can use (class restriction, level restriction)
                                }
                            }
                            // check if can use
                            // check if potion has been created by famous (ranked) alch/creator, bonus + 50%

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
            let sleep_duration = (GAME_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(GAME_TICK_RATE)) as u64;
            if GAME_TICK_RATE - (sleep_duration as u128) < 5 {
                warn!("Less than 5 seconds of sleep, game loop is too slow");
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
                                change_map_packet(warp.dest_map_name.as_str(), warp.to_x, warp.to_y, character.char_id, server_ref.as_ref());
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

            let sleep_duration = (MOVEMENT_TICK_RATE - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick).min(0).max(MOVEMENT_TICK_RATE)) as u64;
            if MOVEMENT_TICK_RATE - (sleep_duration as u128) < 5 {
                warn!("Less than 5 seconds of sleep, movement loop is too slow");
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }
}

