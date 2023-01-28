

use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use enums::vanish::VanishType;
use packets::packets::PacketZcNotifyVanish;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::movement::{Movable, Movement};
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::events::map_event::{MapEvent, MobLocation};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::tick::get_tick;
use crate::enums::EnumWithNumberValue;
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
                                    let mobs = map_instance_state.mobs_mut();
                                    if let Some(mut mob) = mobs.get_mut(&damage.target_id) {
                                        mob.add_attack(damage.attacker_id, damage.damage);
                                        mob.last_attacked_at = tick;
                                        if mob.should_die() {
                                            let delay = damage.attacked_at - tick;
                                            let id = mob.id;
                                            map_instance.add_to_delayed_tick(MapEvent::MobDeathClientNotification(MobLocation { mob_id: mob.id, x: mob.x, y: mob.y }), delay);
                                            drop(mobs);
                                            MapInstanceService::instance().mob_die(map_instance_state.as_mut(), id);
                                        }
                                    }
                                }
                                MapEvent::MobDeathClientNotification(mob_location) => {
                                    let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                                    packet_zc_notify_vanish.set_gid(mob_location.mob_id);
                                    packet_zc_notify_vanish.set_atype(VanishType::Die.value() as u8);
                                    packet_zc_notify_vanish.fill_raw();
                                    map_instance.client_notification_channel().send(Notification::Area(
                                        AreaNotification::new(map_instance.name_with_ext().to_string(), map_instance.id(),
                                                              AreaNotificationRangeType::Fov { x: mob_location.x, y: mob_location.y, exclude_id: None },
                                                              packet_zc_notify_vanish.raw))).expect("Fail to send client notification");
                                }
                                MapEvent::RemoveCharFromMap(char_id) => {
                                    map_instance.state_mut().remove_item_with_id(char_id);
                                }
                                MapEvent::InsertCharToMap(map_item) => {
                                    map_instance.state_mut().insert_item(map_item);
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