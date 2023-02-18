use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::mob::Mob;
use crate::server::model::status::Status;

pub fn create_mob(map_item_id: u32, mob_name: &str) -> Mob {
    let mob = GlobalConfigService::instance().get_mob_by_name(mob_name);
    Mob::new(map_item_id, 90, 90, mob.id as i16, 0, mob.name.clone(), mob.name_english.clone(), Status::from_mob_model(mob))
}