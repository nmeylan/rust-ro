use crate::server::map_item::{MapItemSnapshot};

pub enum MapEvent {
    SpawnMobs,
    UpdateMobsFov(Vec<MapItemSnapshot>),
    MobsActions
}