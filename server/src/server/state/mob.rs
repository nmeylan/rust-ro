use std::collections::HashMap;


use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType, ToMapItem, ToMapItemSnapshot};
use crate::server::model::movement::{Movable, Movement};

use models::position::Position;

use models::status::StatusSnapshot;


#[derive(Setters, Clone)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub name_english: String,
    pub mob_id: i16,
    pub spawn_id: u32,
    pub status: StatusSnapshot,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    pub map_view: Vec<MapItem>,
    pub is_view_char: bool,
    pub movements: Vec<Movement>,
    pub damages: HashMap<u32, u32>,
    pub last_attacked_at: u128,
    pub to_remove: bool,
    pub last_moved_at: u128,
    pub damage_motion: u32,
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
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, name_english: String, damage_motion: u32, status: StatusSnapshot) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status,
            name,
            name_english,
            map_view: vec![],
            is_view_char: false,
            movements: vec![],
            damages: Default::default(),
            last_attacked_at: 0,
            to_remove: false,
            last_moved_at: 0,
            damage_motion,
        }
    }

    #[inline]
    pub fn x(&self) -> u16 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn update_map_view(&mut self, map_items: Vec<MapItem>) {
        self.is_view_char = !map_items.is_empty();
        self.map_view = map_items;
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        #[cfg(feature = "debug_mob_movement")]
        {
            if  crate::server::model::path::manhattan_distance(self.x, self.y, x, y) > 2 {
                error!("mob teleported old ({},{}) new ({},{})", self.x, self.y, x, y);
            }
        }
        self.x = x;
        self.y = y;
    }

    pub fn add_attack(&mut self, attacker_id: u32, damage: u32) {
        if damage == 0 {
            return;
        }
        let hp = self.status.hp();
        if damage > hp {
            self.set_hp(0);
        } else {
            self.set_hp(hp - damage);
        }

        let entry = self.damages.entry(attacker_id).or_insert(0);
        *entry += damage;
    }

    pub fn should_die(&self) -> bool {
        self.status.hp() == 0
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.status.set_hp(hp);
    }

    pub fn hp(&self) -> u32 {
        self.status.hp()
    }

    pub fn set_to_remove(&mut self) {
        self.to_remove = true;
    }

    pub fn is_present(&self) -> bool {
        !self.to_remove
    }

    pub fn attacker_with_higher_damage(&self) -> u32 {
        let mut higher_damage: u32 = 0;
        let mut attacker_with_higher_damage = 0;
        for (attacker_id, damage) in self.damages.iter() {
            if *damage > higher_damage {
                attacker_with_higher_damage = *attacker_id;
                higher_damage = *damage;
            }
        }
        attacker_with_higher_damage
    }
    pub fn set_last_moved_at(&mut self, tick: u128) {
        self.last_moved_at = tick;
    }

    pub fn position(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            dir: 0,
        }
    }
}

impl ToMapItem for Mob {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.mob_id, MapItemType::Mob)
    }
}

impl ToMapItemSnapshot for Mob {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot {
        MapItemSnapshot {
            map_item: self.to_map_item(),
            position: Position {
                x: self.x,
                y: self.y,
                dir: 3, // TODO
            }
        }
    }
}