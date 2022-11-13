pub enum PersistenceEvent {
    SaveCharacterPosition(SavePositionUpdate),
}

pub struct SavePositionUpdate {
    pub map_name: String,
    pub x: u16,
    pub y: u16,
    pub char_id: u32,
    pub account_id: u32,
}