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

    pub fn attack_per_seconds(&self, aspd: f32) -> f32 {
        50_f32 / (200_f32 - aspd.min(199.0))
    }

    pub fn client_aspd(&self, aspd: f32) -> i32 {
        ((200_f32 - aspd.min(199.0)) * 10.0).round() as i32
    }

    ///  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
    /// [] - Square brackets hold the same priority as normal brackets, but indicate that the value of the contents should be rounded down to the nearest whole number (integer) once calculated.
    pub fn aspd(&self, character: &Character) -> f32 {
        let weapon_delay = character.weapon_delay() as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0 - (weapon_delay - ((((weapon_delay * (character.status.agi as f32)) / 25.0).floor() + ((weapon_delay * (character.status.dex as f32)) / 100.0).floor()) / 10.0) * (1.0 - speed_modifier))
    }

    /// PRE-RE https://irowiki.org/classic/Attacks
    /// UI left side atk in status info panel
    pub fn atk1(&self, character: &Character) -> i32 {
        120
    }

    /// UI right side atk in status info panel
    pub fn atk2(&self, character: &Character) -> i32 {
        90
    }

    pub fn base_atk(&self, character: &Character) -> i32 {
        let mut str;
        let mut dex;
        let mut is_ranged_weapon = false;
        let mut right_hand_weapon_atk: u16 = 0;
        let weapon_type = character.right_hand_weapon_type();
        is_ranged_weapon = weapon_type.is_ranged();
        if is_ranged_weapon {
            str = character.status.dex;
            dex = character.status.str;
        } else {
            str = character.status.str;
            dex = character.status.dex;
        }
        // For homunculus
        // dstr = str / 10;
        // str += dstr*dstr;
        let dstr = str / 10;
        str += dstr*dstr;
        str += dex / 5 + character.status.luk / 5;

        (str + right_hand_weapon_atk) as i32
    }
}