use enums::class::JobName;
use enums::look::LookType;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::map_event::CharacterDropItems;
use crate::server::model::item::DroppedItem;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::movement::Movement;
use crate::server::model::position::Position;

#[derive(Debug, PartialEq, Clone)]
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
    CharacterPickUpItem(CharacterPickUpItem),
    CharacterUpdateStat(CharacterUpdateStat),
    MapNotifyItemRemoved(u32),
    CharacterDropItem(CharacterRemoveItem)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveFromMap {
    pub char_id: u32,
    pub map_name: String,
    pub instance_id: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>,
    pub cancel_attack: bool
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterLook {
    pub char_id: u32,
    pub look_type: LookType,
    pub look_value: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterZeny {
    pub char_id: u32,
    pub zeny: Option<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAddItems {
    pub char_id: u32,
    pub should_perform_check: bool, // indicate if we should perform checks before adding items to user
    pub buy: bool, // indicate zeny should be used to buy item (zeny will be updated)
    pub items: Vec<InventoryItemModel>
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveItems {
    pub char_id: u32,
    pub sell: bool, // indicate zeny should be given to character after item sell (zeny will be updated)
    pub items: Vec<CharacterRemoveItem>
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseItem {
    pub char_id: u32,
    pub target_char_id: u32,
    pub index: usize
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterEquipItem {
    pub char_id: u32,
    pub index: usize,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterTakeoffEquipItem {
    pub char_id: u32,
    pub index: usize,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CharacterRemoveItem {
    pub char_id: u32,
    pub index: usize,
    pub amount: i16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAttack {
    pub char_id: u32,
    pub target_id: u32,
    pub repeat: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJobLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJob {
    pub char_id: u32,
    pub job: JobName,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterKillMonster {
    pub char_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16,
    pub map_instance_key: MapInstanceKey,
    pub mob_base_exp: u32,
    pub mob_job_exp: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterPickUpItem {
    pub char_id: u32,
    pub map_item_id: u32,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateStat {
    pub char_id: u32,
    pub stat_id: u16,
    pub change_amount: u16,
}