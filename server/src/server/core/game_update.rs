use crate::server::core::character_movement::Position;

pub enum GameUpdate {
    CharacterChangeMap(CharacterChangeMap),
}

pub struct CharacterChangeMap {
    pub account_id: u32,
    pub new_map_name: String,
    pub new_position: Option<Position>,
    pub old_map_name: Option<String>,
    pub old_position: Option<Position>,
}