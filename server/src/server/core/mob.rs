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
    pub map_view: HashMap<usize, Arc<dyn MapItem>>,
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

    pub fn load_units_in_fov(&mut self) {
        let old_map_view = self.map_view.clone();
        self.map_view.clear();
        let map_ref = self.current_map.clone();
        let map = read_lock!(map_ref);
        let start_x = cmp::max(self.x - MOB_FOV, 0);
        let end_x = cmp::min(self.x + MOB_FOV, map.x_size);
        let start_y = cmp::max(self.y - MOB_FOV, 0);
        let end_y = cmp::min(self.y + MOB_FOV, map.y_size);
        for x in start_x..end_x {
            for y in start_y..end_y {

            }
        }
        let vanish_items = old_map_view.keys().map(|k| *k).collect::<Vec<usize>>();
    }
}