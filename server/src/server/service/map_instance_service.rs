use std::ops::Deref;
use std::sync::mpsc::SyncSender;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};
use packets::packets::PacketZcNotifyMove;
use crate::server::core::map::Map;

use crate::server::core::map_item::{MapItem, MapItemSnapshot, ToMapItem};
use crate::server::core::path::manhattan_distance;
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::{MOB_FOV, Server};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::mob_service::MobService;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::{Mob, MobMovement};
use crate::server::state::status::Status;
use crate::util::tick::get_tick_client;

static mut SERVICE_INSTANCE: Option<MapInstanceService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct MapInstanceService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    mob_service: MobService,
}

impl MapInstanceService {
    pub fn instance() -> &'static MapInstanceService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, mob_service: MobService) -> Self {
        MapInstanceService { client_notification_sender, configuration_service, mob_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, mob_service: MobService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(MapInstanceService::new(client_notification_sender, configuration_service, mob_service));
        });
    }

    pub fn spawn_mobs(&self, map: &Map, map_instance_state: &mut MapInstanceState) {
        for mob_spawn in map.mob_spawns().iter() {
            let mob_spawn_track = map_instance_state.mob_spawns_tracks_mut().iter().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap().clone();
            if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                continue;
            }
            if mob_spawn.has_delay() {
                // TODO check when respawn is planned
            }
            let mut cell: (u16, u16);
            let spawned = mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount;
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {
                    // if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(map_instance_state.cells_mut().deref(), map.x_size());
                }
                let mob_map_item_id = Server::generate_id(map_instance_state.map_items_mut());
                let mob = Mob::new(mob_map_item_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone(),
                                   map_instance_state.key().clone(),
                                   Status::from_mob_model(&mob_spawn.info));

                // TODO: On mob dead clean up should be down also for items below
                map_instance_state.insert_item(mob.to_map_item());
                map_instance_state.mobs_mut().insert(mob_map_item_id, mob);
                // END
                let mob_spawn_track = map_instance_state.mob_spawns_tracks_mut().iter_mut().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap();
                mob_spawn_track.increment_spawn();
            }
        }
    }

    pub fn update_mobs_fov(&self, map_instance_state: &mut MapInstanceState, characters: Vec<MapItemSnapshot>) {
        for (_, mob) in map_instance_state.mobs_mut().iter_mut() {
            let mut viewed_chars: Vec<MapItem> = Vec::with_capacity(characters.len());
            for character in characters.iter() {
                if manhattan_distance(character.x(), character.y(), mob.x(), mob.y()) <= MOB_FOV {
                    viewed_chars.push(character.map_item());
                }
            }
            mob.update_map_view(viewed_chars);
        }
    }

    pub fn mobs_action(&self, map_instance_state: &mut MapInstanceState) {
        let start_time = get_tick_client();
        let start_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut mob_movements: Vec<MobMovement> = Vec::with_capacity(map_instance_state.mobs().len() / 2);
        let cells = map_instance_state.cells().clone();
        let x_size = map_instance_state.x_size();
        let y_size = map_instance_state.y_size();
        for mob in map_instance_state.mobs_mut().values_mut() {
            if let Some(mob_movement) = self.mob_service.action_move(mob, cells.as_ref(), x_size, y_size, start_at) {
                mob_movements.push(mob_movement);
            }
        }
        for mob_movement in mob_movements {
            let mut packet_zc_notify_move = PacketZcNotifyMove::default();
            packet_zc_notify_move.set_gid(mob_movement.id);
            packet_zc_notify_move.move_data = mob_movement.from.to_move_data(&mob_movement.to);
            packet_zc_notify_move.set_move_start_time(start_time);
            packet_zc_notify_move.fill_raw();
            debug!("Mob moving from {} to {}. Notify area around {},{}", mob_movement.from, mob_movement.to, mob_movement.from.x, mob_movement.from.y);
            self.client_notification_sender.send(Notification::Area(
                AreaNotification::new(map_instance_state.key().map_name().clone(), map_instance_state.key().map_instance(),
                                      AreaNotificationRangeType::Fov { x: mob_movement.from.x, y: mob_movement.from.y, exclude_id: None },
                                      packet_zc_notify_move.raw))).expect("Fail to send client notification");
        }
    }

    pub fn mob_die(&self, map_instance_state: &mut MapInstanceState, id: u32) {
        let mob = {
            let mobs = map_instance_state.mobs_mut();
            mobs.remove(&id).unwrap().to_map_item()
        };
        map_instance_state.remove_item(mob);
    }
}