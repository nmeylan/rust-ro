use crate::server::model::status::StatusFromDb;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::mob::Mob;

pub fn create_mob(map_item_id: u32, mob_name: &str) -> Mob {
    let mob = GlobalConfigService::instance().get_mob_by_name(mob_name);
    Mob::new(
        map_item_id,
        90,
        90,
        mob.id as i16,
        0,
        mob.name.clone(),
        mob.name_english.clone(),
        mob.damage_motion as u32,
        StatusFromDb::from_mob_model(mob),
    )
}
pub fn create_mob_by_id(map_item_id: u32, mob_id: u32) -> Mob {
    let mob = GlobalConfigService::instance().get_mob(mob_id as i32);
    Mob::new(
        map_item_id,
        90,
        90,
        mob.id as i16,
        0,
        mob.name.clone(),
        mob.name_english.clone(),
        mob.damage_motion as u32,
        StatusFromDb::from_mob_model(mob),
    )
}
