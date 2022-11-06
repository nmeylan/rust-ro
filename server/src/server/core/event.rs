use crate::server::core::character::Character;
use crate::server::core::character_movement::Movement;
use crate::server::core::position::Position;

pub enum Event {
    CharacterRemove(u32),
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(u32),
    CharacterClearFov(u32),
    CharacterMove(CharacterMovement),
    CharacterClearMove(u32),
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

pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>
}