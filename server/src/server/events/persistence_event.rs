use crate::repository::model::item_model::InventoryItemUpdate;

pub enum PersistenceEvent {
    SaveCharacterPosition(SavePositionUpdate),
    UpdateCharacterStatusU32(StatusUpdate<u32>)
}

pub struct SavePositionUpdate {
    pub char_id: u32,
    pub account_id: u32,
    pub map_name: String,
    pub x: u16,
    pub y: u16,
}

pub struct StatusUpdate<T: Sized> {
    pub char_id: u32,
    pub(crate) db_column: String,
    pub(crate) value: T,
}