use std::collections::HashMap;

use packets::packets::PacketZcNotifyMove;

use crate::server::core::map::Map;
use crate::server::core::map_instance::MapInstanceKey;
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::core::movement::{Movable, Movement};
use crate::server::core::position::Position;
use crate::server::map_item::ToMapItem;
use crate::server::state::status::Status;
use crate::util::tick::get_tick_client;

#[derive(Setters)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub mob_id: i16,
    pub spawn_id: u32,
    pub status: Status,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    pub current_map: MapInstanceKey,
    pub map_view: Vec<MapItem>,
    pub is_view_char: bool,
    pub movements: Vec<Movement>,
}

pub struct MobMovement {
    pub id: u32,
    pub from: Position,
    pub to: Position,
}

impl Movable for Mob {
    fn movements_mut(&mut self) -> &mut Vec<Movement> {
        &mut self.movements
    }
    fn movements(&self) -> &Vec<Movement> {
        &self.movements
    }
    fn set_movement(&mut self, movements: Vec<Movement>) {
        self.movements = movements;
    }
}

impl Mob {
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: MapInstanceKey, status: Status) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status,
            name,
            map_view: vec![],
            current_map,
            is_view_char: false,
            movements: vec![]
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn update_map_view(&mut self, map_items: Vec<MapItem>) {
        self.is_view_char = !map_items.is_empty();
        self.map_view = map_items;
    }

    pub fn action_move(&mut self, cells: &[u16], x_size: u16, y_size: u16) -> Option<MobMovement> {
        if self.is_moving() {
            return None;
        }
        let rng = fastrand::Rng::new();
        let mut movement: Option<MobMovement> = None;
        let rand = rng.i32(0..=100);
        if self.status.speed == 1000 {
            return movement;
        }
        let should_move = if self.is_view_char {
            rand <= 80
        } else {
            rand <= 10
        };

        if should_move {
            let rand_distance = rng.usize(2..=8);
            let current_x = self.x;
            let current_y = self.y;
            if let Some((x, y)) = Map::find_random_walkable_cell_in_max_range(cells, x_size, y_size, current_x, current_y, rand_distance) {
                // Todo: implement server side movement, to avoid desync between client and server
                self.x = x;
                self.y = y;
                let from = Position { x: current_x, y: current_y, dir: 0 };
                let to = Position { x, y, dir: 0 };
                movement = Some(MobMovement { id: self.id, from, to });
            }
        }
        movement
    }
}

impl ToMapItem for Mob {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.mob_id, MapItemType::Mob)
    }
}