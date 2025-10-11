pub mod action;
pub mod events;
pub mod hotkey;
pub mod item;
pub mod map;
pub mod map_instance;
pub mod map_item;
pub mod mob_spawn;
pub mod movement;
pub mod path;
pub mod position;
pub mod request;
pub mod response;
pub mod script;
pub mod session;
pub mod status;
pub mod tasks_queue;
pub mod warp;

pub trait Npc {
    fn get_map_name(&self) -> String;
}
