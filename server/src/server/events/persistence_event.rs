pub enum PersistenceEvent {
    SaveCharacterPosition(SavePositionUpdate),
    UpdateCharacterStatusU32(StatusUpdate<u32>),
    UpdateCharacterInventory(Vec<InventoryItemUpdate>)
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

pub struct InventoryItemUpdate {
    pub item_id: i16,
    pub char_id: i32,
    pub unique_id: i64,
    pub amount: i16,
    pub identified: bool,
    pub stackable: bool
}
