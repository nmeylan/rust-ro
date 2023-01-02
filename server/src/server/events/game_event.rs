use crate::repository::model::item_model::InventoryItemModel;
use crate::server::core::movement::Movement;
use crate::server::core::position::Position;
use crate::server::state::status::LookType;

pub enum GameEvent {
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(CharacterRemoveFromMap),
    CharacterClearFov(u32),
    CharacterMove(CharacterMovement),
    CharacterChangeMap(CharacterChangeMap),
    CharacterUpdateLook(CharacterLook),
    CharacterUpdateZeny(CharacterZeny),
    CharacterUpdateWeight(u32),
    CharacterAddItems(CharacterAddItems),
    CharacterInitInventory(u32),
    CharacterUseItem(CharacterUseItem),
    CharacterAttack(CharacterAttack),
}

pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
}

pub struct CharacterRemoveFromMap {
    pub char_id: u32,
    pub map_name: String,
    pub instance_id: u8,
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
    pub zeny: Option<u32>,
}

pub struct CharacterAddItems {
    pub char_id: u32,
    pub should_perform_check: bool, // indicate if we should perform checks before adding items to user
    pub buy: bool, // indicate zeny should be used to buy item (zeny will be updated)
    pub items: Vec<InventoryItemModel>
}

pub struct CharacterUseItem {
    pub char_id: u32,
    pub target_char_id: u32,
    pub index: usize
}

pub struct CharacterAttack {
    pub char_id: u32,
    pub target_id: u32,
    pub repeat: bool,
}