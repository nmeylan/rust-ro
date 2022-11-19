
use crate::server::core::movement::Movement;
use crate::server::core::position::Position;
use crate::server::state::status::LookType;

pub enum GameEvent {
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(u32),
    CharacterClearFov(u32),
    CharacterMove(CharacterMovement),
    CharacterChangeMap(CharacterChangeMap),
    CharacterUpdateLook(CharacterLook),
    CharacterUpdateZeny(CharacterZeny)
}

pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
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