use std::collections::HashMap;
use std::sync::Arc;
use fastrand::u32;

use packets::packets::PacketZcNotifyMove;

use crate::server::core::map::Map;
use crate::server::core::map_instance::{MapInstance, MapInstanceKey};
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::core::movement::{Movable, Movement};
use crate::server::core::path::path_search_client_side_algorithm;
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
    pub damages: HashMap<u32, u32>,
    pub last_attacked_at: u128
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
            movements: vec![],
            damages: Default::default(),
            last_attacked_at: 0
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

    pub fn action_move(&mut self, map_instance: &MapInstance, cells: &[u16], x_size: u16, y_size: u16, start_at: u128) -> Option<MobMovement> {
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
                let from = Position { x: current_x, y: current_y, dir: 0 };
                let to = Position { x, y, dir: 0 };
                movement = Some(MobMovement { id: self.id, from, to });
                let path = path_search_client_side_algorithm(map_instance, self.x, self.y, to.x, to.y);
                let path = Movement::from_path(path, start_at);
                self.movements = path;
            }
        }
        movement
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn add_attack(&mut self, attacker_id: u32, damage: u32) {
        if damage == 0 {
            return;
        }
        let entry = self.damages.entry(attacker_id).or_insert(0);
        *entry += damage;
    }

    pub fn should_die(&self) -> bool {
        let total_damage: u32 = self.damages.values().sum();
        self.status.max_hp <= total_damage
    }
}

impl ToMapItem for Mob {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.mob_id, MapItemType::Mob)
    }
}