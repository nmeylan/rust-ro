use std::cmp;
use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::{Arc, RwLock};
use crate::server::core::map::MapItem;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::status::Status;
use crate::server::server::MOB_FOV;
use crate::util::coordinate;

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
    pub current_map: Arc<RwLock<MapInstance>>,
    pub map_view: HashMap<usize, u32>,
}

impl MapItem for Mob {
    fn id(&self) -> u32 {
        self.id
    }
    fn client_item_class(&self) -> i16 {
        self.mob_id
    }

    fn object_type(&self) -> i16 {
        5
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl MapItem for std::sync::RwLock<Mob> {
    fn id(&self) -> u32 {
        let self_guard = read_lock!(self);
        self_guard.id
    }
    fn client_item_class(&self) -> i16 {
        let self_guard = read_lock!(self);
        self_guard.mob_id
    }
    fn object_type(&self) -> i16 {
        5
    }

    fn name(&self) -> String {
        let self_guard = read_lock!(self);
        self_guard.name.clone()
    }
}

impl Mob {
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: Arc<RwLock<MapInstance>>) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status: Status::default(),
            name,
            map_view: Default::default(),
            current_map: current_map.clone()
        }
    }

    pub fn set_map_item_at(&mut self, x: u16, y: u16, item: u32) {
        let i = coordinate::get_cell_index_of(x, y, MOB_FOV);
        self.map_view.insert(i, item);
    }

    pub fn load_units_in_fov(&mut self, map_ref: &MapInstance) {
        let old_map_view = self.map_view.clone();
        self.map_view.clear();
        let start_x = cmp::max(self.x - MOB_FOV, 0);
        let end_x = cmp::min(self.x + MOB_FOV, map_ref.x_size);
        let start_y = cmp::max(self.y - MOB_FOV, 0);
        let end_y = cmp::min(self.y + MOB_FOV, map_ref.y_size);
        for x in start_x..end_x {
            for y in start_y..end_y {
                let char_option = map_ref.get_char_at(x, y);
                if char_option.is_some() {
                    info!("{} {} - Add char_id {} to map view, {},{}", self.name, self.id, char_option.unwrap(), x, y);
                    self.set_map_item_at(x,y,char_option.unwrap());
                }
            }
        }
        let vanish_items = old_map_view.keys().map(|k| *k).collect::<Vec<usize>>();
        for item in vanish_items {
            if !self.map_view.contains_key(&item) {
                info!("{} {} - removed char_id {} from map view", self.name, self.id, item);
            }
        }
    }
}