use std::sync::Once;
use enums::{EnumWithMaskValue, EnumWithStringValue};
use enums::item::{EquipmentLocation, ItemType};
use enums::weapon::WeaponType;
use crate::get_item;
use crate::server::service::character::item_service::ItemService;
use crate::server::state::character::Character;
use crate::server::state::status::LookType::Weapon;

static mut SERVICE_INSTANCE: Option<BattleService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct BattleService {}
impl BattleService {
    pub fn instance() -> &'static BattleService {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(BattleService::new());
        });
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    fn new() -> Self {
        BattleService {}
    }
}