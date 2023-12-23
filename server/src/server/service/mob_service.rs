use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::server::model::map::Map;
use crate::server::model::movement::{Movable, Movement};
use crate::server::model::path::path_search_client_side_algorithm;
use models::position::Position;
use crate::server::model::events::client_notification::Notification;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::mob::{Mob, MobMovement};

static mut SERVICE_INSTANCE: Option<MobService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct MobService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
}

impl MobService {
    pub fn instance() -> &'static MobService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) -> Self {
        MobService { client_notification_sender, configuration_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(MobService::new(client_notification_sender, configuration_service));
        });
    }

    pub fn action_move(&self, mob: &mut Mob, cells: &[u16], x_size: u16, y_size: u16, start_at: u128) -> Option<MobMovement> {
        if mob.is_moving() || mob.status.speed() == 1000 {
            return None;
        }
        let mut rng = fastrand::Rng::new();
        let mut movement: Option<MobMovement> = None;
        let rand = rng.i32(0..=100);
        let should_move = if mob.is_view_char {
            rand <= 80
        } else {
            rand <= 10
        };

        if should_move {
            let rand_distance = rng.usize(2..=8);
            let current_x = mob.x;
            let current_y = mob.y;
            if let Some((x, y)) = Map::find_random_walkable_cell_in_max_range(cells, x_size, y_size, current_x, current_y, rand_distance) {
                let from = Position { x: current_x, y: current_y, dir: 0 };
                let to = Position { x, y, dir: 0 };
                movement = Some(MobMovement { id: mob.id, from, to });
                let path = path_search_client_side_algorithm(x_size, y_size, cells, mob.x, mob.y, to.x, to.y);
                let path = Movement::from_path(path, start_at);
                mob.movements = path;
            }
        }
        movement
    }
}