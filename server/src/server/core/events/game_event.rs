use crate::server::core::character::Character;
use crate::server::core::character_movement::Movement;
use crate::server::core::position::Position;
use crate::server::core::status::LookType;

pub enum GameEvent {
    CharacterRemove(u32),
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(u32),
    CharacterClearFov(u32),
    CharacterMove(CharacterMovement),
    CharacterClearMove(u32),
    CharacterChangeMap(CharacterChangeMap),
    CharacterUpdateLook(CharacterLook),
    CharacterUpdateZeny(CharacterZeny)
}

pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
    pub old_map_name: Option<String>,
    pub old_position: Option<Position>,
}


pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>
}

pub struct CharacterLook {
    pub char_id: u32,
    pub look_type: LookType,
    pub look_value: u32,
}

pub struct CharacterZeny {
    pub char_id: u32,
    pub zeny: u32,
}