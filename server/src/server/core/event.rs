use crate::server::core::character::Character;
use crate::server::core::position::Position;

pub enum Event {
    CharacterInsert(Character),
    CharacterRemove(u32),
    CharacterRemoveFromMap(u32),
    CharacterClearFov(u32),
    CharacterUpdatePosition(CharacterUpdatePosition),
    CharacterChangeMap(CharacterChangeMap),
}

pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
    pub old_map_name: Option<String>,
    pub old_position: Option<Position>,
}

pub struct CharacterUpdatePosition {
    pub char_id: u32,
    pub x: u16,
    pub y: u16,
}