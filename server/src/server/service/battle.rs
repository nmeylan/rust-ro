use std::sync::Once;
use crate::server::state::character::Character;

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

    pub fn aspd(&self, character: &Character) -> f32 {
        //  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
        let weapon_delay = character.weapon_delay() as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0 - (weapon_delay - ((((weapon_delay * (character.status.agi as f32)) / 25.0) + ((weapon_delay * (character.status.dex as f32)) / 100.0)) / 10.0) * (1.0 - speed_modifier))
    }
}