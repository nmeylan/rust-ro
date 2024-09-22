use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::server::model::map::Map;
use crate::server::model::movement::{Movable, Movement};
use crate::server::model::path::path_search_client_side_algorithm;
use models::position::Position;
use crate::server::model::events::client_notification::Notification;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::mob::{Mob, MobMovement};

#[allow(dead_code)]
pub struct MobService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
}

impl MobService {

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) -> Self {
        MobService { client_notification_sender, configuration_service }
    }

    pub fn action_move(&self, mob: &mut Mob, cells: &[u16], x_size: u16, y_size: u16, start_at: u128) -> Option<MobMovement> {
        if !mob.is_present() || mob.is_moving() || mob.status.speed() == 1000 || start_at < mob.last_moved_at || start_at - mob.last_moved_at < 500 {
            return None;
        }
        let mut rng = fastrand::Rng::new();
        let mut movement: Option<MobMovement> = None;
        let rand = rng.i32(0..=100);
        let should_move = if mob.is_view_char {
            rand <= (self.configuration_service.config().game.mob_move_frequency_when_player_around * 100.0) as i32
        } else {
            rand <= (self.configuration_service.config().game.mob_move_frequency_when_no_player_around * 100.0) as i32
        };

        if should_move {
            let rand_distance = rng.usize(2..=8);
            let current_x = mob.x;
            let current_y = mob.y;
            if let Some((x, y)) = Map::find_random_walkable_cell_in_max_range(cells, x_size, y_size, current_x, current_y, rand_distance) {
                if current_x == x && current_y == y {
                    return None;
                }
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