use enums::class::JobName;
use enums::look::LookType;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::movement::Movement;
use crate::server::model::position::Position;

#[derive(Debug, PartialEq)]
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
    CharacterEquipItem(CharacterEquipItem),
    CharacterTakeoffEquipItem(CharacterTakeoffEquipItem),
    CharacterAttack(CharacterAttack),
    CharacterCalculateStats(u32),
    CharacterChangeLevel(CharacterChangeLevel),
    CharacterChangeJobLevel(CharacterChangeJobLevel),
    CharacterChangeJob(CharacterChangeJob),
    CharacterKillMonster(CharacterKillMonster),
}

#[derive(Debug, PartialEq)]
pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
}

#[derive(Debug, PartialEq)]
pub struct CharacterRemoveFromMap {
    pub char_id: u32,
    pub map_name: String,
    pub instance_id: u8,
}

#[derive(Debug, PartialEq)]
pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>,
    pub cancel_attack: bool
}

#[derive(Debug, PartialEq)]
pub struct CharacterLook {
    pub char_id: u32,
    pub look_type: LookType,
    pub look_value: u16,
}

#[derive(Debug, PartialEq)]
pub struct CharacterZeny {
    pub char_id: u32,
    pub zeny: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct CharacterAddItems {
    pub char_id: u32,
    pub should_perform_check: bool, // indicate if we should perform checks before adding items to user
    pub buy: bool, // indicate zeny should be used to buy item (zeny will be updated)
    pub items: Vec<InventoryItemModel>
}

#[derive(Debug, PartialEq)]
pub struct CharacterUseItem {
    pub char_id: u32,
    pub target_char_id: u32,
    pub index: usize
}

#[derive(Debug, PartialEq)]
pub struct CharacterEquipItem {
    pub char_id: u32,
    pub index: usize,
    pub location: u32
}
#[derive(Debug, PartialEq)]
pub struct CharacterTakeoffEquipItem {
    pub char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq)]
pub struct CharacterAttack {
    pub char_id: u32,
    pub target_id: u32,
    pub repeat: bool,
}

#[derive(Debug, PartialEq)]
pub struct CharacterChangeLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct CharacterChangeJobLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct CharacterChangeJob {
    pub char_id: u32,
    pub job: JobName,
}

#[derive(Debug, PartialEq)]
pub struct CharacterKillMonster {
    pub char_id: u32,
    pub mob_id: i16,
    pub map_instance_key: MapInstanceKey
}