pub mod session;
pub mod map;
pub mod path;
pub mod map_instance;
pub mod request;
pub mod response;
pub mod tasks_queue;
pub mod position;
pub mod movement;
pub mod configuration;
pub mod map_item;
pub mod action;
pub mod events;
pub mod warp;
pub mod script;
pub mod mob_spawn;
pub mod item;


pub trait Npc {
    fn get_map_name(&self) -> String;
}