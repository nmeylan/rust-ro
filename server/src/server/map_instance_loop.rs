

use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};


use crate::server::model::map_instance::MapInstance;
use crate::server::model::movement::{Movable, Movement};

use crate::server::model::events::map_event::{MapEvent};
use crate::server::service::global_config_service::GlobalConfigService;

use crate::util::tick::get_tick;
use crate::server::service::map_instance_service::MapInstanceService;

pub struct MapInstanceLoop;

pub const MOVEMENT_TICK_RATE: u128 = 20;
pub const MAP_LOOP_TICK_RATE: u128 = 40;

impl MapInstanceLoop {
    pub fn start_map_instance_thread(map_instance: Arc<MapInstance>) {
        let map_instance_clone_for_thread = map_instance.clone();
        info!("Start thread for {}", map_instance.name());
        thread::Builder::new().name(format!("map_instance_{}_loop_thread", map_instance.name()))
            .spawn(move || {
                let map_instance = map_instance_clone_for_thread;
                let mut last_mobs_spawn = Instant::now();
                let mut last_mobs_action = Instant::now();
                loop {
                    let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    let now = Instant::now();
                    if last_mobs_action.elapsed().as_secs() > 2 {
                        let map_instance_state = map_instance.state_mut().as_mut();
                        MapInstanceService::instance().mobs_action(map_instance_state);
                        last_mobs_action = now;
                    }
                    if last_mobs_spawn.elapsed().as_secs() >= 1 {
                        MapInstanceService::instance().spawn_mobs(map_instance.map(), map_instance.state_mut().as_mut());
                        last_mobs_spawn = now;
                    }
                    if let Some(tasks) = map_instance.pop_task() {
                        for task in tasks {
                            match task {
                                MapEvent::UpdateMobsFov(characters) => {
                                    let map_instance_state = map_instance.state_mut().as_mut();
                                    MapInstanceService::instance().update_mobs_fov(map_instance_state, characters);
                                }
                                MapEvent::MobDamage(damage) => {
                                    let mut map_instance_state = map_instance.state_mut();
                                    MapInstanceService::instance().mob_being_attacked(map_instance_state.as_mut(), damage, map_instance.task_queue(), tick);
                                }
                                MapEvent::MobDeathClientNotification(mob_location) => {
                                    let map_instance_state = map_instance.state();
                                    MapInstanceService::instance().mob_die_client_notification(map_instance_state.as_ref(), mob_location);
                                }
                                MapEvent::RemoveCharFromMap(char_id) => {
                                    map_instance.state_mut().remove_item_with_id(char_id);
                                }
                                MapEvent::InsertCharToMap(map_item) => {
                                    map_instance.state_mut().insert_item(map_item);
                                }
                                MapEvent::MobDropItems(mob_drop_items) => {
                                    MapInstanceService::instance().mob_drop_items_and_send_packet(map_instance.state_mut().as_mut(), mob_drop_items);
                                }
                                MapEvent::RemoveDroppedItemFromMap(dropped_item_id) => {
                                    MapInstanceService::instance().remove_dropped_item_from_map(map_instance.state_mut().as_mut(), dropped_item_id);
                                }
                                MapEvent::CharDropItems(character_drop_items) => {
                                    MapInstanceService::instance().character_drop_items_and_send_packet( map_instance.state_mut().as_mut(), character_drop_items);
                                }
                            }
                        }
                    }
                    let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
                    let sleep_duration = (MAP_LOOP_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
                    if sleep_duration < 5 {
                        warn!("Less than 5 milliseconds of sleep, map_instance_{}_loop is too slow - {}ms because game loop took {}ms", map_instance.name(), sleep_duration, time_spent);
                    }
                    sleep(Duration::from_millis(sleep_duration));
                }
            }).unwrap();
        thread::Builder::new().name(format!("map_instance_{}_mob_movement_thread", map_instance.name()))
            .spawn(move || {
                loop {
                    let tick = get_tick();
                    let mut map_instance_state = map_instance.state_mut();
                    let mobs = map_instance_state.mobs_mut();
                    for mob in mobs.values_mut().filter(|mob| mob.is_moving()) {
                        let speed = mob.status.speed;
                        if let Some(movement) = mob.peek_movement() {
                            let mob_model = GlobalConfigService::instance().get_mob(mob.mob_id as i32);
                            if tick >= movement.move_at() {
                                if tick < mob.last_attacked_at + (mob_model.damage_motion as u128) {
                                    info!("Mob delayed movement because he is attacked");
                                    continue;
                                }
                                let movement = mob.pop_movement().unwrap();
                                debug!("mob move {} at {}", movement.position(), movement.move_at());
                                mob.update_position(movement.position().x, movement.position().y);

                                if let Some(next_movement) = mob.peek_mut_movement() {
                                    next_movement.set_move_at(tick + Movement::delay(speed, next_movement.is_diagonal()))
                                }
                            }
                        }
                    }
                    let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
                    let sleep_duration = (MOVEMENT_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
                    if sleep_duration < 5 {
                        warn!("Mob Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop took {}ms", sleep_duration, time_spent);
                    }
                    sleep(Duration::from_millis(sleep_duration));
                }
            }).unwrap();
    }
}