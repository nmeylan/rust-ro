use std::any::Any;
use std::cmp;
use std::fmt::{Debug};
use std::sync::{Arc, RwLock};
use crate::server::core::character::CharacterSession;
use crate::server::server::UNKNOWN_MAP_ITEM;
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
    pub map_view: RwLock<Vec<Arc<dyn MapItem>>>,
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
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: Arc<RwLock<MapInstance>>) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status: Status::default(),
            name,
            map_view: RwLock::new(vec![UNKNOWN_MAP_ITEM.clone(); ((MOB_FOV * 2) * (MOB_FOV * 2)) as usize]),
            current_map: current_map.clone(),
            is_view_char: RwLock::new(false)
        }
    }

    pub fn load_units_in_fov(&self, map_ref: &MapInstance) {
        let mut is_view_char_guard = write_lock!(self.is_view_char);
        *is_view_char_guard = false;
        let mut map_view_guard = write_lock!(self.map_view);
        *map_view_guard = vec![UNKNOWN_MAP_ITEM.clone(); (((MOB_FOV + 1) * 2) * ((MOB_FOV + 1) * 2)) as usize];
        let start_x = self.get_fov_start_x();
        let start_y = self.get_fov_start_y();
        for i in 0..(MOB_FOV * 2) {
            for j in 0..(MOB_FOV * 2) {
                let char_option = map_ref.get_map_item_at(i + start_x, j + start_y);
                if char_option.as_ref().is_none() {
                    continue;
                }
                let map_item = char_option.as_ref().unwrap();

                if map_item.as_any().downcast_ref::<CharacterSession>().is_some() {
                    // info!("{} {} - seen char_id {} from map view, at {},{}", self.name, self.id, map_item.id(), i + start_x, j + start_y);
                    *is_view_char_guard = true;
                    map_view_guard.insert(coordinate::get_cell_index_of(i, j, MOB_FOV), char_option.as_ref().unwrap().clone());
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