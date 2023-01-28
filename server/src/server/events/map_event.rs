use crate::server::core::map_item::MapItemSnapshot;

pub enum MapEvent {
    UpdateMobsFov(Vec<MapItemSnapshot>),
    MobDamage(MobDamage)
}

pub struct MobDamage {
    pub mob_id: u32,
    pub attacker_id: u32,
    pub damage: u32,
    pub attacked_at: u128,
}