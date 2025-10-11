use std::sync::{Arc, Once};

use base64::Engine;
use base64::engine::general_purpose;
use models::enums::bonus::BonusType;
use models::enums::class::JobName;
use models::enums::{EnumStackable, EnumWithNumberValue, EnumWithStringValue};
use models::item::Wearable;
use models::status::{Status, StatusSnapshot};
use models::status_bonus::StatusBonus;
use rathena_script_lang_interpreter::lang::compiler::Compiler;
use rathena_script_lang_interpreter::lang::vm::Vm;

use crate::repository::model::item_model::ItemModel;
use crate::server::script::item_script_handler::DynamicItemScriptHandler;
use crate::server::service::global_config_service::GlobalConfigService;

static mut SERVICE_INSTANCE: Option<StatusService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct StatusService {
    configuration_service: &'static GlobalConfigService,
    item_script_vm: Arc<Vm>,
}

impl StatusService {
    pub fn new(configuration_service: &'static GlobalConfigService, item_script_vm: Arc<Vm>) -> StatusService {
        StatusService {
            configuration_service,
            item_script_vm,
        }
    }

    pub fn instance() -> &'static StatusService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(configuration_service: &'static GlobalConfigService, item_script_vm: Arc<Vm>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(StatusService::new(configuration_service, item_script_vm));
        });
    }

    #[inline(always)]
    pub fn to_snapshot_cached(&self, status: &Status, tick: u128) -> StatusSnapshot {
        self.to_snapshot(status)
    }

    //#[metrics::elapsed]
    pub fn to_snapshot(&self, status: &Status) -> StatusSnapshot {
        let mut snapshot = StatusSnapshot::_from(status);
        let job = JobName::from_value(status.job as usize);
        let job_config = self.configuration_service.get_job_config(snapshot.job());
        let index_for_job_level = (status.job_level.max(1) - 1) as usize;
        let index_for_base_level = (status.base_level.max(1) - 1).min(99) as usize; // TODO if we want to scale some stats after lvl 99, need to remove min(99) and implement formula
        job_config.bonus_stats().get(index_for_job_level).map(|bonus| {
            snapshot.set_bonus_str(*bonus.get("str").unwrap_or(&0_i16));
            snapshot.set_bonus_agi(*bonus.get("agi").unwrap_or(&0_i16));
            snapshot.set_bonus_dex(*bonus.get("dex").unwrap_or(&0_i16));
            snapshot.set_bonus_vit(*bonus.get("vit").unwrap_or(&0_i16));
            snapshot.set_bonus_int(*bonus.get("int").unwrap_or(&0_i16));
            snapshot.set_bonus_luk(*bonus.get("luk").unwrap_or(&0_i16));
        });
        let mut bonuses: Vec<BonusType> = vec![];

        for equipment in status.equipped_weapons().iter() {
            let item_model = self.configuration_service.get_item(equipment.item_id());
            if equipment.card0 > 0 {
                let item_model = self.configuration_service.get_item(equipment.card0 as i32);
                self.collect_bonuses(status, &mut bonuses, item_model);
            }
            if equipment.card1 > 0 {
                let item_model = self.configuration_service.get_item(equipment.card1 as i32);
                self.collect_bonuses(status, &mut bonuses, item_model);
            }
            if equipment.card2 > 0 {
                let item_model = self.configuration_service.get_item(equipment.card2 as i32);
                self.collect_bonuses(status, &mut bonuses, item_model);
            }
            if equipment.card3 > 0 {
                let item_model = self.configuration_service.get_item(equipment.card3 as i32);
                self.collect_bonuses(status, &mut bonuses, item_model);
            }
            self.collect_bonuses(status, &mut bonuses, item_model);
        }
        status.equipped_ammo().map(|ammo| {
            let item_model = self.configuration_service.get_item(ammo.item_id());
            self.collect_bonuses(status, &mut bonuses, item_model);
        });

        for equipment in status.equipped_gears().iter() {
            let item_model = self.configuration_service.get_item(equipment.item_id());
            if equipment.card0 > 0 {
                let item_model = self.configuration_service.get_item(equipment.card0 as i32);
                self.collect_bonuses(status, &mut bonuses, item_model);
            }
            if let Some(def) = item_model.defense {
                snapshot.set_def(snapshot.def() + def)
            }
            self.collect_bonuses(status, &mut bonuses, item_model);
        }

        // TODO card and item combo

        // Apply skills bonuses
        for temporary_bonus in status.temporary_bonuses.iter() {
            bonuses.push(*temporary_bonus.bonus());
        }

        bonuses = BonusType::merge_enums(&bonuses);

        bonuses.iter().for_each(|bonus| bonus.add_bonus_to_status(&mut snapshot));
        // TODO [([base_hp*(1 + VIT/100)* trans_mod]+HPAdditions)*ItemHPMultipliers] https://irowiki.org/classic/Max_HP
        let hp_rebirth_modifier: f32 = if job.is_rebirth() { 1.25 } else { 1.0 };
        snapshot.set_max_hp(
            (job_config.base_hp()[index_for_base_level] as f32 * (1.0 + snapshot.vit() as f32 / 100.0) * hp_rebirth_modifier).floor()
                as u32,
        );
        // TODO https://irowiki.org/classic/Max_SP
        snapshot.set_max_sp(
            (job_config.base_sp()[index_for_base_level] as f32 * (1.0 + snapshot.int() as f32 / 100.0) * hp_rebirth_modifier).floor()
                as u32,
        );
        // TODO 1 + YourLUK*0.3 + Critical Increasing Cards)*CritModifier - TargetLUK/5
        snapshot.set_crit(Self::truncate(snapshot.crit() + (1.0 + snapshot.luk() as f32 * 0.3), 1));
        snapshot.set_hit((snapshot.hit() + status.base_level as i16 + snapshot.dex() as i16).max(0));
        snapshot.set_flee((snapshot.flee() + status.base_level as i16 + snapshot.agi() as i16).max(0));
        snapshot.set_aspd(snapshot.aspd() + self.aspd(&snapshot));
        snapshot.set_matk_min(
            ((snapshot.int() + ((snapshot.int() as f32 / 7.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor()
                as u16,
        );
        snapshot.set_matk_max(
            ((snapshot.int() + ((snapshot.int() as f32 / 5.0).floor() as u16).pow(2)) as f32 * snapshot.matk_item_modifier()).floor()
                as u16,
        );
        snapshot.set_fist_atk(self.fist_atk(&snapshot, snapshot.right_hand_weapon_type().is_ranged()));
        snapshot.set_atk_left_side(self.status_atk_left_side(&snapshot));
        self.set_status_atk_right_side(&mut snapshot);
        bonuses.iter().for_each(|bonus| bonus.add_percentage_bonus_to_status(&mut snapshot));
        snapshot.set_bonuses(bonuses.iter().map(|b| StatusBonus::new(*b)).collect::<Vec<StatusBonus>>());
        snapshot
    }

    #[inline]
    pub fn collect_bonuses(&self, status: &Status, mut bonuses: &mut Vec<BonusType>, item_model: &ItemModel) {
        if item_model.item_bonuses_are_dynamic {
            self.collect_dynamic_script(status, &mut bonuses, &item_model);
        } else {
            item_model.bonuses.iter().for_each(|bonus| bonuses.push(*bonus))
        }
    }

    // #[metrics::elapsed]
    #[inline]
    fn collect_dynamic_script(&self, status: &Status, bonuses: &mut &mut Vec<BonusType>, item_model: &&ItemModel) {
        let dynamic_item_script_handler = DynamicItemScriptHandler::new(self.configuration_service, status, item_model.id as u32);
        if self.item_script_vm.contains_class(format!("itemscript{}", item_model.id).as_str()) {
            Vm::repl_on_registered_class(
                self.item_script_vm.clone(),
                format!("itemscript{}", item_model.id).as_str(),
                Box::new(&dynamic_item_script_handler),
                vec![],
            )
            .map_err(|e| error!("Failed to execute item script for item {}, due to \n{}", item_model.id, e))
            .unwrap();
        } else if let Some(script_compilation) = &item_model.script_compilation {
            let script = general_purpose::STANDARD.decode(script_compilation).unwrap();
            let maybe_class = Compiler::from_binary(&script).unwrap().pop();
            Vm::bootstrap_without_init(self.item_script_vm.clone(), vec![maybe_class.unwrap()]);
            Vm::repl_on_registered_class(
                self.item_script_vm.clone(),
                format!("itemscript{}", item_model.id).as_str(),
                Box::new(&dynamic_item_script_handler),
                vec![],
            )
            .map_err(|e| {
                error!(
                    "Failed to execute item script for item {}, due to \n{}",
                    item_model.id, e.message
                )
            })
            .unwrap();
        }
        bonuses.extend(dynamic_item_script_handler.drain());
    }

    #[inline]
    fn truncate(x: f32, decimals: u32) -> f32 {
        let y = 10i32.pow(decimals) as f32;
        (x * y).round() / y
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

    #[inline]
    pub fn attack_delay(&self, status: &StatusSnapshot) -> u32 {
        self.attack_motion(status) / 2
    }

    pub fn client_aspd(&self, aspd: f32) -> i32 {
        ((200_f32 - aspd.min(199.0)) * 10.0).round() as i32
    }

    pub fn cast_time_reduction(&self, status: &StatusSnapshot) -> f32 {
        (1.0 - status.dex() as f32 / 150.0) * (1.0 + status.cast_time())
    }

    ///  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
    /// [] - Square brackets hold the same priority as normal brackets, but
    /// indicate that the value of the contents should be rounded down to the
    /// nearest whole number (integer) once calculated. http://calc.free-ro.com/
    fn aspd(&self, status: &StatusSnapshot) -> f32 {
        let weapon_delay = self.weapon_delay(status) as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0
            - (weapon_delay
                - ((((weapon_delay * (status.agi() as f32)) / 25.0).floor() + ((weapon_delay * (status.dex() as f32)) / 100.0).floor())
                    / 10.0)
                    * (1.0 - speed_modifier))
    }

    #[inline]
    fn weapon_delay(&self, status: &StatusSnapshot) -> u32 {
        let weapon = status.right_hand_weapon_type();
        *self
            .configuration_service
            .get_job_config(status.job())
            .base_aspd()
            .get(weapon.as_str())
            .unwrap_or(&2000)
    }

    /// PRE-RE https://irowiki.org/classic/Attacks
    /// UI left side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    ///
    /// Atk stands for Attack and gives an indication of how much damage you
    /// will do when you hit something. The visible components of the Atk
    /// score are your Strength plus the Atk of the weapon you are using on the
    /// left and the damage bonus from any pluses the weapon might have on the
    /// right. The real value on the left of your Atk score includes hidden
    /// bonuses from Strength, Dexterity and Luck. For fists, the true value
    /// is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] where [] indicates you
    /// round the value inside down before continuing and ^2 indicates squaring.
    /// For weapons, the true value is equal to: STR + [STR/10]^2 + [DEX/5] +
    /// [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the
    /// value inside down before continuing and ^2 indicates squaring.
    /// For missile weapons, the true value is equal to: DEX + [DEX/10]^2 +
    /// [STR/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you
    /// round the value inside down before continuing and ^2 indicates squaring.
    /// Not counting the value of WeaponAtk and AtkBonusCards, this true value
    /// is often referred to as the base damage. This base damage is basically
    /// the your Atk with bare fists.
    #[inline]
    fn status_atk_left_side(&self, status: &StatusSnapshot) -> i32 {
        let imposito_magnus = 0;
        let _upgrade_damage = 0;
        let _atk_cards = 0;
        (status.fist_atk() + status.weapon_atk() + imposito_magnus + status.weapon_upgrade_damage() + status.base_atk()) as i32
    }

    #[inline]
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

    /// UI right side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    /// https://web.archive.org/web/20060717222819/http://rodatazone.simgaming.net/items/upgrading.php
    #[inline]
    pub fn set_status_atk_right_side(&self, status: &mut StatusSnapshot) {
        let mut atk_right = 0_i32;
        let mut overupgrade_right_hand_atk_bonus = 0;
        let mut overupgrade_left_hand_atk_bonus = 0;
        status.right_hand_weapon().map(|w| {
            if w.level() == 1 {
                atk_right = 2 * w.refine() as i32;
                if w.refine() > 7 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 7) * 3;
                }
            } else if w.level() == 2 {
                atk_right = 3 * w.refine() as i32;
                if w.refine() > 6 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 6) * 5;
                }
            } else if w.level() == 3 {
                atk_right = 5 * w.refine() as i32;
                if w.refine() > 5 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 5) * 8;
                }
            } else if w.level() == 4 {
                atk_right = 7 * w.refine() as i32;
                if w.refine() > 4 {
                    overupgrade_right_hand_atk_bonus = (w.refine() - 4) * 14;
                }
            }
        });
        status.left_hand_weapon().map(|w| {
            if w.level() == 1 {
                atk_right += 2 * w.refine() as i32;
                if w.refine() > 7 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 7) * 3;
                }
            } else if w.level() == 2 {
                atk_right += 3 * w.refine() as i32;
                if w.refine() > 6 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 6) * 5;
                }
            } else if w.level() == 3 {
                atk_right += 5 * w.refine() as i32;
                if w.refine() > 5 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 5) * 8;
                }
            } else if w.level() == 4 {
                atk_right += 7 * w.refine() as i32;
                if w.refine() > 4 {
                    overupgrade_left_hand_atk_bonus = (w.refine() - 4) * 14;
                }
            }
        });
        status.set_atk_right_side(atk_right);
        status.set_overupgrade_right_hand_atk_bonus(overupgrade_right_hand_atk_bonus);
        status.set_overupgrade_left_hand_atk_bonus(overupgrade_left_hand_atk_bonus);
    }

    #[inline]
    pub fn character_vit_def(&self, status_snapshot: &StatusSnapshot) -> u16 {
        status_snapshot.vit() // TODO angelus multiplier
    }

    pub fn character_regen_hp(&self, status_snapshot: &StatusSnapshot) -> u32 {
        // var HPR = Math.max( 1, Math.floor(MAX_HP / 200) );
        // HPR += Math.floor( VIT / 5 );
        // HPR = Math.floor( HPR * (1 + HPR_MOD * 0.01) );
        let hp_regen =
            1.0_f32.max((status_snapshot.max_hp() as f32 / 200.0).floor()) as u32 + (status_snapshot.vit() as f32 / 5.0).floor() as u32;
        // TODO hp_regen bonus
        hp_regen
    }

    pub fn character_regen_sp(&self, status_snapshot: &StatusSnapshot) -> u32 {
        // var SPR = 1;
        // SPR += Math.floor( MAX_SP / 100 );
        // SPR += Math.floor( INT / 6 );
        // if (INT >= 120) {
        //  SPR += Math.floor(INT / 2 - 56);
        // }
        // SPR = Math.floor( SPR * (1 + SPR_MOD * 0.01) );
        let mut sp_regen =
            1 + (status_snapshot.max_sp() as f32 / 100.0).floor() as u32 + (status_snapshot.int() as f32 / 6.0).floor() as u32;
        if (status_snapshot.int() > 120) {
            sp_regen += ((status_snapshot.int() as f32 / 2.0) - 56.0).floor() as u32;
        }
        // TODO sp_regen bonus
        sp_regen
    }
}

#[cfg(test)]
mod tests {
    use models::enums::bonus::BonusType;
    use models::enums::skill_enums::SkillEnum;
    use models::status::Status;
    use models::status_bonus::TemporaryStatusBonus;

    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::service::status_service::StatusService;
    use crate::tests::common;

    #[test]
    fn test_snapshot_bonuses_have_temporary_bonuses() {
        // Given
        common::before_all();
        let service = StatusService::new(GlobalConfigService::instance(), common::test_script_vm());
        let mut status = Status::default();
        status.temporary_bonuses.add(TemporaryStatusBonus::with_duration(
            BonusType::Agi(10),
            0,
            0,
            1000,
            SkillEnum::AlIncagi.id() as u16,
        ));
        // When
        let snapshot = service.to_snapshot(&status);
        // Then
        assert!(
            snapshot
                .bonuses()
                .iter()
                .any(|status_bonus| { matches!(status_bonus.bonus(), BonusType::Agi(10)) }),
            "missing agi bonus"
        );
    }
}
