use crate::server::core::action::Damage;
use crate::server::core::map_item::MapItemSnapshot;

pub enum MapEvent {
    UpdateMobsFov(Vec<MapItemSnapshot>),
    MobDamage(Damage),
    MobDeathClientNotification(MobLocation),
}

pub struct MobLocation {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
}
