use crate::server::core::character_movement::Position;

pub enum Event {
    CharacterRemoveFromMap(u32),
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