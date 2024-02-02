use std::sync::{Once};
use models::status::{Status, StatusSnapshot};
use models::enums::EnumWithStringValue;
use crate::server::service::global_config_service::GlobalConfigService;


static mut SERVICE_INSTANCE: Option<StatusService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct StatusService {
    configuration_service: &'static GlobalConfigService,
}

impl StatusService {
    pub fn new(configuration_service: &'static GlobalConfigService) -> StatusService {
        StatusService { configuration_service }
    }
    pub fn instance() -> &'static StatusService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(StatusService::new(configuration_service));
        });
    }

    pub fn to_snapshot(&self, status: &Status) -> StatusSnapshot {
        let mut snapshot = StatusSnapshot::from(status);
        snapshot.set_aspd(self.aspd(&snapshot));
        self.configuration_service.get_job_config(snapshot.job()).bonus_stats()
            .get((status.job_level.max(1) - 1) as usize)
            .map(|bonus| {
                snapshot.set_bonus_str(*bonus.get("str").unwrap_or(&0_u16));
                snapshot.set_bonus_agi(*bonus.get("agi").unwrap_or(&0_u16));
                snapshot.set_bonus_dex(*bonus.get("dex").unwrap_or(&0_u16));
                snapshot.set_bonus_vit(*bonus.get("vit").unwrap_or(&0_u16));
                snapshot.set_bonus_int(*bonus.get("int").unwrap_or(&0_u16));
                snapshot.set_bonus_luk(*bonus.get("luk").unwrap_or(&0_u16));
            });
        for equipment in status.all_equipped_items() {
            let item_model = self.configuration_service.get_item(equipment.item_id());
            if item_model.item_bonuses_are_dynamic {
                // TODO
            } else {
                item_model.bonuses.iter().for_each(|bonus| bonus.add_bonus_to_status(&mut snapshot))
            }
        }
        snapshot.set_matk_min(((snapshot.int() + ((snapshot.int() as f32 / 7.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor() as u16);
        snapshot.set_matk_max(((snapshot.int() + ((snapshot.int() as f32 / 5.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor() as u16);
        // TODO add bonuses from item and cards snapshot.bonuses.push(...)
        snapshot
    }

    #[inline]
    pub fn attack_per_seconds(&self, aspd: f32) -> f32 {
        50_f32 / (200_f32 - aspd.min(199.0))
    }

    #[inline]
    pub fn attack_motion(&self, status: &StatusSnapshot) -> u32 {
        let aspd = status.aspd();
        (1000.0 / self.attack_per_seconds(aspd)).round() as u32
    }

    pub fn client_aspd(&self, aspd: f32) -> i32 {
        ((200_f32 - aspd.min(199.0)) * 10.0).round() as i32
    }

    ///  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
    /// [] - Square brackets hold the same priority as normal brackets, but indicate that the value of the contents should be rounded down to the nearest whole number (integer) once calculated.
    /// http://calc.free-ro.com/
    fn aspd(&self, status: &StatusSnapshot) -> f32 {
        let weapon_delay = self.weapon_delay(status) as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0 - (weapon_delay - ((((weapon_delay * (status.agi() as f32)) / 25.0).floor() + ((weapon_delay * (status.dex() as f32)) / 100.0).floor()) / 10.0) * (1.0 - speed_modifier))
    }

    fn weapon_delay(&self, status: &StatusSnapshot) -> u32 {
        let weapon =  status.right_hand_weapon_type();
        *self.configuration_service.get_job_config(status.job()).base_aspd().get(weapon.as_str()).unwrap_or(&2000)
    }

    /// PRE-RE https://irowiki.org/classic/Attacks
    /// UI left side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    ///
    /// Atk stands for Attack and gives an indication of how much damage you will do when you hit something.
    ///The visible components of the Atk score are your Strength plus the Atk of the weapon you are using on the left and the damage bonus from any pluses the weapon might have on the right.
    ///The real value on the left of your Atk score includes hidden bonuses from Strength, Dexterity and Luck.
    ///For fists, the true value is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///For weapons, the true value is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///For missile weapons, the true value is equal to: DEX + [DEX/10]^2 + [STR/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///Not counting the value of WeaponAtk and AtkBonusCards, this true value is often referred to as the base damage. This base damage is basically the your Atk with bare fists.
    pub fn status_atk_left_side(&self, status: &StatusSnapshot) -> i32 {
        let imposito_magnus = 0;
        let _upgrade_damage = 0;
        let _atk_cards = 0;
        (self.fist_atk(status, status.right_hand_weapon_type().is_ranged()) + status.weapon_atk() + imposito_magnus + status.weapon_upgrade_damage() + status.base_atk()) as i32
    }

    pub(crate) fn fist_atk(&self, status: &StatusSnapshot, is_ranged: bool) -> u16 {
        let mut str;
        let dex;

        if is_ranged {
            str = status.dex();
            dex = status.str();
        } else {
            str = status.str();
            dex = status.dex();
        }
        // For homunculus
        // dstr = str / 10;
        // str += dstr*dstr;
        let dstr = str / 10;
        str += dstr * dstr;
        str += dex / 5 + status.luk() / 5;
        str
    }

    pub fn atk_cards(&self, _status: &StatusSnapshot) -> u16 {
        0
    }

    /// UI right side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    /// https://web.archive.org/web/20060717222819/http://rodatazone.simgaming.net/items/upgrading.php
    pub fn status_atk_right_side(&self, _status: &StatusSnapshot) -> i32 {
        // TODO: it is refinement damage. do not mix with refinement bonus which refers to random additional atk for over upgrade
        // refinement
        //    Weapon Lv. 1 - Every +1 upgrade gives +2 ATK (+1~3 ATK for every overupgrade).
        //     Weapon Lv. 2 - Every +1 upgrade gives +3 ATK (+1~5 ATK for every overupgrade).
        //     Weapon Lv. 3 - Every +1 upgrade gives +5 ATK (+1~7 ATK for every overupgrade).
        //     Weapon Lv. 4 - Every +1 upgrade gives +7 ATK (+1~13(?) ATK for every overupgrade).
        //    Weapon Lv. 1 - Safety Level +7
        //     Weapon Lv. 2 - Safety Level +6
        //     Weapon Lv. 3 - Safety Level +5
        //     Weapon Lv. 4 - Safety Level +4
        0
    }

    /// VIT + rnd(0,[VIT/20]^2-1).
    pub fn mob_vit_def(&self, vit: u32) -> u32 {
        let mut rng = fastrand::Rng::new();
        vit + rng.u32(0..1.max(1.max(((vit as f32 / 20.0).ceil() as u32).pow(2)) - 1))
    }
    /// [VIT*0.5] + rnd([VIT*0.3], max([VIT*0.3],[VIT^2/150]-1)).
    pub fn character_vit_def(&self, _vit: u32) -> u32 {
        0
    }


}
