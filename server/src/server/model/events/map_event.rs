use crate::server::model::action::Damage;
use crate::server::model::events::game_event::CharacterKillMonster;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::{MapItem, MapItemSnapshot};

pub enum MapEvent {
    UpdateMobsFov(Vec<MapItemSnapshot>),
    RemoveCharFromMap(u32),
    InsertCharToMap(MapItem),
    MobDamage(Damage),
    MobDeathClientNotification(MobLocation),
    MonsterDropItems(MonsterDropItems),
}

pub struct MobLocation {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq)]
pub struct MonsterDropItems {
    pub owner_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16
}