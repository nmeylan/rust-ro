use std::fmt::{Debug};
use crate::server::core::map::MapItem;
use crate::server::core::status::Status;

#[derive(Setters, Debug)]
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
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status: Status::default(),
            name
        }
    }
}