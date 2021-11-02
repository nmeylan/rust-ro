use std::any::Any;
use std::cmp;
use std::fmt::{Debug};
use std::sync::{Arc};
use parking_lot::RwLock;
use crate::server::core::character::CharacterSession;
use crate::server::server::{MOB_FOV_SLICE_LEN, UNKNOWN_MAP_ITEM};
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
    pub current_map: RwLock<Arc<MapInstance>>,
    pub map_view: RwLock<Vec<Option<Arc<dyn MapItem>>>>,
    pub is_view_char: RwLock<bool>
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

    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
       self.y
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
}

impl Mob {
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: Arc<MapInstance>) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status: Status::default(),
            name,
            map_view: RwLock::new(vec![None; MOB_FOV_SLICE_LEN]),
            current_map: RwLock::new(current_map.clone()),
            is_view_char: RwLock::new(false)
        }
    }

    pub fn load_units_in_fov(&self, map_ref: &MapInstance) {
        let mut is_view_char_guard = write_lock!(self.is_view_char);
        *is_view_char_guard = false;
        let mut map_view_guard = write_lock!(self.map_view);
        *map_view_guard = Vec::with_capacity(MOB_FOV_SLICE_LEN);
        map_view_guard.resize(MOB_FOV_SLICE_LEN, None);
        for i in 0..(MOB_FOV * 2) {
            for j in 0..(MOB_FOV * 2) {
                let x = self.get_item_x_from_fov(i as usize);
                let y = self.get_item_y_from_fov(j as usize);
                let char_option = map_ref.get_map_item_at(x, y);
                if char_option.as_ref().is_none() {
                    continue;
                }
                let map_item = char_option.unwrap();

                if map_item.as_any().downcast_ref::<CharacterSession>().is_some() {
                    // info!("{{{}:{}}},{{{}:{}}} {},{}", self.get_fov_start_x(), self.get_fov_start_y(), self.get_fov_start_x()  + (MOB_FOV * 2), self.get_fov_start_y() + (MOB_FOV * 2), self.x, self.y  );
                    // info!("{} {} - seen char_id {} from map view, at {},{}", self.name, self.id, map_item.id(), map_item.x(), map_item.y());
                    *is_view_char_guard = true;
                    map_view_guard[coordinate::get_cell_index_of(i, j, MOB_FOV)] = Some(map_item.clone());
                }
            }
        }
    }

    pub fn get_fov_start_x(&self) -> u16 {
        cmp::max(self.x - MOB_FOV, 0)
    }

    pub fn get_fov_start_y(&self) -> u16 {
        cmp::max(self.y - MOB_FOV, 0)
    }

    pub fn get_item_x_from_fov(&self, i: usize) -> u16 {
        self.get_fov_start_x() + i as u16
    }

    pub fn get_item_y_from_fov(&self, j: usize) -> u16 {
        self.get_fov_start_y() + j as u16
    }
}