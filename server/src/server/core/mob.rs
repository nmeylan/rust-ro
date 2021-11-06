use std::any::Any;
use std::sync::{Arc, RwLock};
use crate::server::core::character::Character;
use crate::server::server::{MOB_FOV_SLICE_LEN};
use crate::server::core::map::MapItem;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::status::Status;
use crate::server::server::MOB_FOV;

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

    fn as_any(&self) -> &dyn Any {
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
        let mut items = Vec::with_capacity(MOB_FOV_SLICE_LEN);
        let mut has_seen_char = false;
        let map_items = map_ref.get_map_items(self.x, self.y, MOB_FOV);
        for map_item in map_items {
            if map_item.as_any().downcast_ref::<Character>().is_some() {
                // info!("{{{}:{}}},{{{}:{}}} {},{}", self.get_fov_start_x(), self.get_fov_start_y(), self.get_fov_start_x()  + (MOB_FOV * 2), self.get_fov_start_y() + (MOB_FOV * 2), self.x, self.y  );
                // info!("{} {} {},{} - seen char_id {} from map view, at {},{}", self.name, self.id, self.x, self.y, map_item.id(), map_item.x(), map_item.y());
                has_seen_char = true;
                items.push(Some(map_item.clone()));
            }
        }
        let mut is_view_char_guard = write_lock!(self.is_view_char);
        let mut map_view_guard = write_lock!(self.map_view);
        *is_view_char_guard = has_seen_char;
        *map_view_guard = items;
    }

}