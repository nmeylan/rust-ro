use std::borrow::Borrow;

use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::server::model::events::game_event::{CharacterRemoveItems, GameEvent};
use crate::server::model::movement::{Movable, Movement};
use crate::PersistenceEvent;
use crate::PersistenceEvent::SaveCharacterPosition;
use models::enums::skill_enums::SkillEnum;
use packets::packets::{Packet, PacketZcNotifyMove, PacketZcNotifyPlayermove};

use crate::server::model::events::client_notification::{AreaNotification, CharNotification, Notification};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::events::persistence_event::SavePositionUpdate;

use crate::server::model::map_item::{ToMapItem, ToMapItemSnapshot};

use crate::server::Server;

use crate::server::service::global_config_service::GlobalConfigService;

use crate::server::service::status_service::StatusService;

const MOVEMENT_TICK_RATE: u128 = 16;
pub const GAME_TICK_RATE: u128 = 40;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>) {
        loop {
            if !server_ref.is_alive() {
                break;
            }
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            Self::game_loop_iteration(server_ref.clone().as_ref(), tick);
            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (GAME_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!("Less than 5 milliseconds of sleep, game loop is too slow - {}ms because game loop took {}ms", sleep_duration, time_spent);
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }

    pub(crate) fn game_loop_iteration(server_ref: &Server, tick: u128) {
        let mut server_state_mut = server_ref.state_mut();

        for (_, character) in server_state_mut.characters_mut().iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
            server_ref.server_service.character_remove_expired_bonuses(character, tick);
        }
        if let Some(tasks) = server_ref.pop_task() {
            for task in tasks {
                match task {
                    GameEvent::CharacterLeaveGame((char_id, _atype)) => {
                        server_ref.disconnect_character(char_id);
                    }
                    GameEvent::CharacterJoinGame(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.skill_tree_service().send_skill_tree(character);
                    }
                    GameEvent::CharacterChangeMap(event) => {
                        let map_instance = server_ref.state().get_map_instance(&event.new_map_name, event.new_instance_id)
                            .unwrap_or_else(|| server_ref.server_service.create_map_instance(server_state_mut.as_mut(), GlobalConfigService::instance().get_map(&event.new_map_name), event.new_instance_id));
                        let character = server_state_mut.characters_mut().get_mut(&event.char_id).unwrap();
                        server_ref.character_service().change_map(map_instance.key(), event.new_position.unwrap(), character);
                        let char_map_item = character.to_map_item();
                        map_instance.add_to_next_tick(MapEvent::InsertCharToMap(char_map_item));
                        server_ref.add_to_next_tick(GameEvent::CharacterInitInventory(character.char_id));
                        let account_id = character.account_id;
                        server_state_mut.insert_map_item(account_id, char_map_item);
                    }
                    GameEvent::CharacterRemoveFromMap(character_remove_from_map) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_remove_from_map.char_id).unwrap();
                        character.movements = vec![];
                        if let Some(instance) = server_ref.state().get_map_instance(&character_remove_from_map.map_name, character_remove_from_map.instance_id) {
                            instance.add_to_next_tick(MapEvent::RemoveCharFromMap(character_remove_from_map.char_id));
                        }
                    }
                    GameEvent::CharacterClearFov(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.clear_map_view();
                    }
                    GameEvent::CharacterLoadedFromClientSide(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.loaded_from_client_side = true;
                        character.clear_map_view();
                    }
                    GameEvent::CharacterMove(_) => {
                        // handled by dedicated thread
                    }
                    GameEvent::CharacterUpdateLook(character_look) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_look.char_id).unwrap();
                        server_ref.character_service().change_look(character_look, character)
                    }
                    GameEvent::CharacterUpdateZeny(zeny_update) => {
                        let character = server_state_mut.characters_mut().get_mut(&zeny_update.char_id).unwrap();
                        server_ref.character_service().update_zeny(server_ref.runtime.as_ref(), zeny_update, character);
                    }
                    GameEvent::CharacterAddItems(add_items) => {
                        let character = server_state_mut.characters_mut().get_mut(&add_items.char_id).unwrap();
                        server_ref.inventory_service().add_items_in_inventory(server_ref.runtime.as_ref(), add_items, character);
                    }
                    GameEvent::CharacterInitInventory(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.inventory_service().reload_inventory(server_ref.runtime.as_ref(), char_id, character);
                        server_ref.inventory_service().reload_equipped_item_sprites(character);
                        server_ref.character_service().reload_client_side_status(character);
                        server_ref.character_service().reload_client_side_hotkeys(character);
                    }
                    GameEvent::CharacterUpdateWeight(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().notify_weight(character);
                    }
                    GameEvent::CharacterUseItem(character_user_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_user_item.char_id).unwrap();
                        server_ref.item_service().use_item(server_ref, server_ref.runtime.as_ref(), character_user_item, character);
                    }
                    GameEvent::CharacterAttack(character_attack) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_attack.char_id).unwrap();
                        if !character.is_attacking() {
                            character.set_attack(character_attack.target_id, character_attack.repeat, 0);
                        }
                    }
                    GameEvent::CharacterEquipItem(character_equip_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_equip_item.char_id).unwrap();
                        let equipped_item = server_ref.inventory_service().equip_item(character, character_equip_item);
                        equipped_item
                            .map(|item| server_ref.inventory_service().sprite_change_packet_for_item(character, &item, false)
                                .map(|packet| server_ref.character_service().send_area_notification_around_characters(character, packet)));
                    }
                    GameEvent::CharacterTakeoffEquipItem(character_takeoff_equip_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_takeoff_equip_item.char_id).unwrap();
                        let index = character_takeoff_equip_item.index;
                        server_ref.inventory_service().takeoff_equip_item(character, index)
                            .map(|item| server_ref.inventory_service().sprite_change_packet_for_item(character, &item, true)
                                .map(|packet| server_ref.character_service().send_area_notification_around_characters(character, packet)));
                    }
                    GameEvent::CharacterUpdateClientSideStats(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().reload_client_side_status(character);
                    }
                    GameEvent::CharacterChangeLevel(character_change_level) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                        let delta = server_ref.character_service().update_base_level(character, character_change_level.set_level, character_change_level.add_level);
                        if delta < 0 {
                            // TODO ensure equip required min level
                        }
                    }
                    GameEvent::CharacterChangeJobLevel(character_change_level) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                        let delta = server_ref.character_service().update_job_level(character, character_change_level.set_level, character_change_level.add_level);
                        if delta < 0 {
                            // TODO ensure equip required min level
                        }
                    }
                    GameEvent::CharacterChangeJob(character_change_job) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_job.char_id).unwrap();
                        server_ref.character_service().change_job(character, character_change_job.job, character_change_job.should_reset_skills);
                        // TODO ensure equip required class
                    }
                    GameEvent::CharacterKillMonster(character_kill_monster) => {
                        if let Some(character) = server_state_mut.characters_mut().get_mut(&character_kill_monster.char_id) {
                            let map_instance = server_ref.state().get_map_instance(character_kill_monster.map_instance_key.map_name(), character_kill_monster.map_instance_key.map_instance()).unwrap();
                            server_ref.character_service().character_kill_monster(character, character_kill_monster, map_instance.as_ref());
                        }
                    }
                    GameEvent::CharacterPickUpItem(character_pickup_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_pickup_item.char_id).unwrap();
                        let map_instance = server_ref.state().get_map_instance(character.current_map_name(), character.current_map_instance()).unwrap();
                        server_ref.server_service.character_pickup_item(server_ref.state_mut().as_mut(), character, character_pickup_item.map_item_id, map_instance.as_ref(), server_ref.runtime.as_ref());
                    }
                    GameEvent::MapNotifyItemRemoved(map_item_id) => {
                        server_state_mut.remove_locked_map_item(map_item_id);
                    }
                    GameEvent::CharacterUpdateStat(character_update_stat) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_update_stat.char_id).unwrap();
                        server_ref.character_service().character_increase_stat(character, character_update_stat);
                    }
                    GameEvent::CharacterSkillUpgrade(character_skill_upgrade) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_skill_upgrade.char_id).unwrap();
                        server_ref.character_service().allocate_skill_point(character, SkillEnum::from_id(character_skill_upgrade.skill_id as u32));
                    }
                    GameEvent::CharacterDropItem(character_drop_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_drop_item.char_id).unwrap();
                        let map_instance = server_ref.state().get_map_instance(character.current_map_name(), character.current_map_instance()).unwrap();
                        let character_remove_items = CharacterRemoveItems { char_id: character.char_id, sell: false, items: vec![character_drop_item] };
                        server_ref.inventory_service().character_drop_items(server_ref.runtime.as_ref(), character, character_remove_items, map_instance.as_ref());
                    }
                    GameEvent::CharacterSellItems(character_remove_items) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_remove_items.char_id).unwrap();
                        server_ref.inventory_service().character_sell_items(server_ref.runtime.as_ref(), character, character_remove_items);
                    }
                    GameEvent::CharacterResetSkills(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().reset_skills(character, true);
                    }
                    GameEvent::CharacterResetStats(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().reset_stats(character);
                    }
                    GameEvent::CharacterUseSkill(character_use_skill) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_use_skill.char_id).unwrap();
                        server_ref.server_service.character_start_use_skill(server_ref.state(), character, character_use_skill, tick);
                    }
                    GameEvent::CharacterDamage(_damage) => {
                        println!("GameEvent::CharacterDamage: Not implemented yet!")
                    }
                    GameEvent::CharacterUpdateSpeed(char_id, speed) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.status.set_speed(speed);
                        server_ref.character_service().reload_client_side_status(character);
                    }
                    GameEvent::CharacterHotkeyAdd(char_id,hotkey) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().hotkey_add(character, hotkey);
                    }
                    GameEvent::CharacterHotkeyRemove(char_id,hotkey_index) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().hotkey_remove(character, hotkey_index);

                    }
                    GameEvent::CharacterRestoreAllHpAndSP(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        let status = StatusService::instance().to_snapshot(&character.status);
                        server_ref.character_service().update_hp_sp(character, status.max_hp(), status.max_sp());
                    }
                    GameEvent::CharacterSit(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().sit(character);
                    }
                    GameEvent::CharacterStand(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().stand(character);
                    }
                    GameEvent::CharacterCancelMove(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().cancel_movement(character, tick);
                    }
                    GameEvent::CharacterSlotCard(char_equip_card) => {
                        
                    }
                }
            }
        }
        for (_, character) in server_state_mut.characters_mut().iter_mut().filter(|(_, character)| character.loaded_from_client_side) {
            let map_instance = server_ref.state().get_map_instance(character.current_map_name(), character.current_map_instance());
            if let Some(map_instance) = map_instance {
                server_ref.character_service().load_units_in_fov(server_ref.state(), character, map_instance.state().borrow().as_ref());
            }
            server_ref.server_service.character_attack(server_ref.state(), tick, character);
            server_ref.server_service.character_use_skill(server_ref.state(), tick, character);
            server_ref.character_service().regen_hp(character, tick);
            server_ref.character_service().regen_sp(character, tick);
        }
        for (_, map) in server_ref.state().map_instances().iter() {
            for instance in map.iter() {
                instance.add_to_next_tick(MapEvent::UpdateMobsFov(server_state_mut.characters_mut().iter().map(|(_, character)| character.to_map_item_snapshot()).collect()));
            }
        }
    }


    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>) {
        loop {
            if !server_ref.is_alive() {
                break;
            }
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let mut server_state_mut = server_ref.state_mut();
            if let Some(tasks) = server_ref.pop_movement_task() {
                for task in tasks {
                    if let GameEvent::CharacterMove(character_movement) = task {
                        let character = server_state_mut.characters_mut().get_mut(&character_movement.char_id).unwrap();
                        if character_movement.cancel_attack {
                            character.clear_attack();
                        }
                        let status_snapshot = server_ref.server_service.get_status_snapshot(&character.status, tick);
                        let speed = status_snapshot.speed();
                        if character_movement.path.last().is_none(){
                            continue;
                        }
                        let new_movement = character_movement.path.last().unwrap();
                        let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new(GlobalConfigService::instance().packetver());

                        let move_at = tick + Movement::delay(speed, new_movement.is_diagonal());
                        if let Some(previous_movement) = character.peek_movement() {
                            #[cfg(feature = "debug_movement")]
                            {
                                if tick >= previous_movement.move_at() {
                                    info!("change path! was {} will {}, move at {}",previous_movement.position(), new_movement.position(), move_at );
                                    crate::util::debug::debug_in_game_chat(client_notification_sender_clone.clone(), character, format!("change path! was {} will {}, move at {}", previous_movement.position(), new_movement.position(), move_at));
                                } else {
                                    server_ref.add_to_next_movement_tick(GameEvent::CharacterMove(character_movement));
                                    continue;
                                }
                            }
                            #[cfg(not(feature = "debug_movement"))]
                            {
                                if tick < previous_movement.move_at() {
                                    server_ref.add_to_next_movement_tick(GameEvent::CharacterMove(character_movement));
                                    continue;
                                }
                            }
                        }
                        character.set_movement(character_movement.path);
                        let movement = character.peek_mut_movement().unwrap();
                        movement.set_move_at(move_at);
                        let moved_at = movement.move_at() as u32;
                        packet_zc_notify_playermove.set_move_start_time(moved_at); // todo: time conversion check on client side ???
                        let movement = character_movement.current_position.to_move_data(&character_movement.destination);
                        packet_zc_notify_playermove.set_move_data(movement.clone());
                        packet_zc_notify_playermove.fill_raw();
                        client_notification_sender_clone.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_notify_playermove.raw_mut()))))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                        let mut packet_zc_notify_move = PacketZcNotifyMove::new(GlobalConfigService::instance().packetver());
                        packet_zc_notify_move.set_move_data(movement);
                        packet_zc_notify_move.set_gid(character.char_id);
                        packet_zc_notify_move.set_move_start_time(moved_at as u32);
                        packet_zc_notify_move.fill_raw();
                        client_notification_sender_clone.send(Notification::Area(AreaNotification::from_character_exclude_self(character, std::mem::take(packet_zc_notify_move.raw_mut()))))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                    }
                }
            }

            // If movement not smooth:
            // teleport in front -> server movement faster than client movement
            // teleport back -> server movement slower than client movement
            let mut character_finished_to_move = vec![];
            for (_, character) in server_state_mut.characters_mut().iter_mut().filter(|(_, character)| character.is_moving()) {
                let status_snapshot = server_ref.server_service.get_status_snapshot(&character.status, tick);
                let speed = status_snapshot.speed();
                if let Some(movement) = character.peek_movement() {
                    if tick >= movement.move_at() {
                        let movement = character.pop_movement().unwrap();
                        #[cfg(feature = "debug_movement")]
                        {
                            let last_move_at = character.last_moved_at;
                            info!("move {} at {} after {}ms since last move", movement.position(), tick, tick - last_move_at);
                            crate::util::debug::debug_in_game_chat(client_notification_sender_clone.clone(), character, format!("move {} at {} after {}ms since last move", movement.position(), tick, tick - last_move_at));
                        }
                        character.set_last_moved_at(tick);
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.state().get_map_instance_from_character(character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.state().is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                server_ref.server_service.schedule_warp_to_walkable_cell(server_ref.state_mut().as_mut(), warp.dest_map_name.as_str(), warp.to_x, warp.to_y, character.char_id);
                                character.clear_movement();
                                continue;
                            }
                        }
                        #[cfg(feature = "debug_movement")]
                        {
                            if let Some(next_movement) = character.peek_movement() {
                                let next_move_at = tick + Movement::delay(speed, next_movement.is_diagonal());
                                #[cfg(feature = "debug_movement")]
                                {
                                    let last_move_at = character.last_moved_at;
                                    info!("move {} at {} after {}ms since last move, next move will be {} at {}", movement.position(), tick, tick - last_move_at, next_movement.position(), next_move_at);
                                    crate::util::debug::debug_in_game_chat(client_notification_sender_clone.clone(), character, format!("move {} at {} after {}ms since last move, next move will be {} at {}", movement.position(), tick, tick - last_move_at, next_movement.position(), next_move_at));
                                }
                                let next_movement = character.peek_mut_movement().unwrap();
                                next_movement.set_move_at(next_move_at);
                            } else {
                                character_finished_to_move.push(character);
                            }
                        }
                        #[cfg(not(feature = "debug_movement"))]
                        {
                            if let Some(next_movement) = character.peek_mut_movement() {
                                let next_move_at = tick + Movement::delay(speed, next_movement.is_diagonal());
                                next_movement.set_move_at(next_move_at);
                            } else {
                                character_finished_to_move.push(character);
                            }
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

