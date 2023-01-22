use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::repository::model::item_model::ItemModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::events::client_notification::Notification;
use crate::server::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;

static mut SERVICE_INSTANCE: Option<BattleService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct BattleService {
    status_service: StatusService,
    configuration_service: &'static GlobalConfigService,
}

impl BattleService {
    pub fn instance() -> &'static BattleService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(status_service: StatusService, configuration_service: &'static GlobalConfigService) -> Self {
        BattleService { status_service, configuration_service }
    }

    pub fn init(status_service: StatusService, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(BattleService::new(status_service, configuration_service));
        });
    }

    /// (([((({(base_atk +
    /// + rnd(min(DEX,ATK), ATK)*SizeModifier) * SkillModifiers * (1 - DEF/100) - VitDEF + BaneSkill + UpgradeDamage}
    /// + MasterySkill + WeaponryResearchSkill + EnvenomSkill) * ElementalModifier) + Enhancements) * DamageBonusModifiers * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits
    pub fn damage_character_attack_monster_melee(&self, source: &Character, target: &MobModel) -> u32 {
        let _rng = fastrand::Rng::new();
        let upgrade_bonus: f32 = 0.0; // TODO: weapon level1 : (+1~3 ATK for every overupgrade). weapon level2 : (+1~5 ATK for every overupgrade). weapon level3 : (+1~7 ATK for every overupgrade). weapon level4 : (+1~13 ATK for every overupgrade).
        let imposito_magnus: f32 = 0.0;
        let base_atk = self.status_service.fist_atk(source) as f32 + upgrade_bonus as f32 + imposito_magnus as f32 + self.status_service.atk_cards(source) as f32;

        let size_modifier: f32 = 1.0; // TODO
        let skill_modifier: f32 = 1.0; // TODO
        let def: f32 = target.def as f32 / 100.0;
        let vitdef: f32 = self.status_service.mob_vit_def(target.vit as u32) as f32; // TODO set to 0 if critical hit
        let bane_skill: f32 = 0.0; // TODO Beast Bane, Daemon Bane, Draconology
        let mastery_skill: f32 = 0.0;
        let weaponery_research_skill: f32 = 0.0;
        let evenom_skill: f32 = 0.0;
        let elemental_modifier: f32 = 1.0;
        let enchantements: f32 = 0.0;
        let damage_bonus_modifier: f32 = 1.0;
        let damage_reduction_modifier: f32 = 1.0;
        let number_of_hits: f32 = 1.0;
        let kyrie_eleison_effect: f32 = 0.0;
        let weapon = source.right_hand_weapon().map(|(_, weapon)| self.configuration_service.get_item(weapon.item_id));
        let damage = (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (base_atk + self.weapon_atk(source, weapon) as f32 * size_modifier) * skill_modifier * (1.0 - def)
                                        )
                                            - vitdef + bane_skill + self.status_service.weapon_upgrade_damage(source) as f32
                                    )
                                        + mastery_skill + weaponery_research_skill + evenom_skill
                                )
                                    * elemental_modifier
                            ) + enchantements)
                            * damage_bonus_modifier * damage_reduction_modifier
                    ) * number_of_hits
                )
                    - kyrie_eleison_effect
            )
                / number_of_hits
        ).round() as u32;
        damage
    }

    //  rnd(min(DEX*(0.8+0.2*WeaponLevel),ATK), ATK)
    pub fn weapon_atk(&self, source: &Character, weapon: Option<&ItemModel>) -> u32 {
        let rng = fastrand::Rng::new();
        let mut weapon_level = 0;
        let mut weapon_attack = 0;
        if let Some(weapon) = weapon {
            weapon_level = if let Some(weapon_level) = weapon.weapon_level {
                weapon_level
            } else {
                warn!("Weapon {} has no level", weapon.name_aegis);
                0
            };
            weapon_attack = if let Some(weapon_attack) = weapon.attack {
                weapon_attack as u32
            } else {
                warn!("Weapon {} has no attack", weapon.name_aegis);
                0
            };
        };

        rng.u32(((source.status.dex as f32 * (0.8 + 0.2 * weapon_level as f32)).round() as u32).min(weapon_attack)..weapon_attack)
    }
}