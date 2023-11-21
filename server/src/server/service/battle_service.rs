use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::Once;
use enums::action::ActionType;
use enums::EnumWithMaskValueU64;
use enums::size::Size;
use enums::skill::SkillState;
use enums::weapon::WeaponType;
use models::status::Status;
use packets::packets::PacketZcNotifyAct;
use crate::repository::model::item_model::ItemModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::model::map_item::{MapItem, MapItemType};
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};


use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::enums::EnumWithNumberValue;
use crate::packets::packets::Packet;
use crate::server::model::action::Damage;
use crate::server::model::status::StatusFromDb;

static mut SERVICE_INSTANCE: Option<BattleService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct BattleService {
    client_notification_sender: SyncSender<Notification>,
    status_service: StatusService,
    configuration_service: &'static GlobalConfigService,
}

impl BattleService {
    pub fn instance() -> &'static BattleService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, status_service: StatusService, configuration_service: &'static GlobalConfigService) -> Self {
        BattleService { client_notification_sender, status_service, configuration_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, status_service: StatusService, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(BattleService::new(client_notification_sender, status_service, configuration_service));
        });
    }

    /// (([((({(base_atk +
    /// + rnd(min(DEX,ATK), ATK)*SizeModifier) * SkillModifiers * (1 - DEF/100) - VitDEF + BaneSkill + UpgradeDamage}
    /// + MasterySkill + WeaponryResearchSkill + EnvenomSkill) * ElementalModifier) + Enhancements) * DamageBonusModifiers * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits
    pub fn damage_character_attack_monster(&self, source_status: &Status, target_status: &Status, skill_modifier: f32) -> u32 {
        let _rng = fastrand::Rng::new();
        let upgrade_bonus: f32 = 0.0; // TODO: weapon level1 : (+1~3 ATK for every overupgrade). weapon level2 : (+1~5 ATK for every overupgrade). weapon level3 : (+1~7 ATK for every overupgrade). weapon level4 : (+1~13 ATK for every overupgrade).
        let imposito_magnus: u32 = 0;
        let base_atk = self.status_service.fist_atk(&source_status) as f32 + upgrade_bonus + self.status_service.atk_cards(&source_status) as f32;

        let size_modifier: f32 = self.size_modifier(source_status, target_status);
        let def: f32 = target_status.def as f32 / 100.0;
        let vitdef: f32 = self.status_service.mob_vit_def(target_status.vit as u32) as f32; // TODO set to 0 if critical hit
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

        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (base_atk + self.weapon_atk(source_status, &target_status) as f32) * skill_modifier * (1.0 - def)
                                        )
                                            - vitdef + bane_skill + self.status_service.weapon_upgrade_damage(&source_status) as f32
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
        ).round() as u32
    }

    //  rnd(min(DEX*(0.8+0.2*WeaponLevel),ATK), ATK)
    pub fn weapon_atk(&self, source_status: &Status, target_status: &Status) -> u32 {
        let mut rng = fastrand::Rng::new();
        let weapon = source_status.right_hand_weapon().map(|weapon| self.configuration_service.get_item(weapon.item_id));
        let imposito_magnus: u32 = 0; // TODO get from status
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

        let work_dex = ((source_status.dex as f32 * (0.8 + 0.2 * weapon_level as f32)).round() as u32);
        let mut weapon_max_attack: u32 = 0;
        let mut weapon_over_upgrade_max: u32 = 0;
        let mut weapon_over_upgrade_min: u32 = 0;
        let mut weapon_min_attack: u32 = 0;
        let size_modifier = self.size_modifier(source_status, target_status);
        if work_dex >= weapon_attack { // || maximize power skill
            weapon_max_attack = weapon_over_upgrade_max + ((weapon_attack + imposito_magnus) as f32 * size_modifier).floor() as u32;
            weapon_min_attack = weapon_over_upgrade_min + ((weapon_attack + imposito_magnus) as f32 * size_modifier).floor() as u32;
        } else {
            weapon_max_attack = weapon_over_upgrade_max + ((weapon_attack - 1 + imposito_magnus) as f32 * size_modifier).floor() as u32;
            weapon_min_attack = weapon_over_upgrade_min + ((work_dex + imposito_magnus) as f32 * size_modifier).floor() as u32;
        }
        // if weapon.is_some() && weapon.as_ref().unwrap().weapon_type.unwrap().is_ranged() {
        //     if let Some(ammo) = source_status.equipped_ammo() {
        //         let ammo_atk = self.configuration_service.get_item(ammo.item_id).attack.unwrap_or(1);
        //         weapon_max_attack += ((ammo_atk - 1) as f32 * size_modifier).floor();
        //     }
        // }
        rng.u32(weapon_min_attack..=weapon_max_attack)
    }

    #[inline]
    pub fn size_modifier(&self, source_status: &Status, target_status: &Status) -> f32 {
        // Size Modifiers for Weapons
        // Size 	Fist 	Dagger 	1H Sword 	2H Sword 	Spear 	Spear+Peco 	Axe 	Mace 	Rod 	Bow 	Katar 	Book 	Claw 	Instrument 	Whip 	Gun 	Huuma Shuriken
        // Small 	100 	100 	75       	75       	75  	75 	        50  	75  	100 	100 	75  	100 	100 	75 	        75 	    100 	100
        // Medium 	100 	75  	100      	75       	75  	100 	    75  	100 	100 	100 	100 	100 	75 	    100 	    100 	100 	100
        // Large 	100 	50  	75      	100     	100 	100 	    100 	100 	100 	75  	75 	    50 	    50 	    75 	        50 	    100 	100
        let weapon_type = self.status_service.right_hand_weapon_type(source_status);
        match weapon_type {
            WeaponType::Fist => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0
                }
            }
            WeaponType::Dagger => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 0.75,
                    Size::Large => 0.5
                }
            }
            WeaponType::Sword1H => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large =>  0.75,
                }
            }
            WeaponType::Sword2H => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 0.75,
                    Size::Large => 1.0
                }
            }
            WeaponType::Spear1H | WeaponType::Spear2H => {
                if source_status.state & SkillState::Riding.as_flag() > 0 {
                    match target_status.size {
                        Size::Small => 0.75,
                        Size::Medium => 1.0,
                        Size::Large => 1.0
                    }
                } else {
                    match target_status.size {
                        Size::Small => 0.75,
                        Size::Medium => 0.75,
                        Size::Large => 1.0
                    }
                }
            }
            WeaponType::Axe1H | WeaponType::Axe2H => {
                match target_status.size {
                    Size::Small => 0.5,
                    Size::Medium => 0.75,
                    Size::Large => 1.0
                }
            }
            WeaponType::Mace | WeaponType::Mace2H => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 1.0
                }
            }
            WeaponType::Staff | WeaponType::Staff2H => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0
                }
            }
            WeaponType::Bow => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 0.75
                }
            }
            WeaponType::Knuckle => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 0.75,
                    Size::Large => 0.5
                }
            }
            WeaponType::Musical => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.75
                }
            }
            WeaponType::Whip => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.5
                }
            }
            WeaponType::Book => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 0.5
                }
            }
            WeaponType::Katar => {
                match target_status.size {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.75
                }
            }
            WeaponType::Revolver | WeaponType::Rifle | WeaponType::Gatling | WeaponType::Shotgun | WeaponType::Grenade => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0
                }
            }
            WeaponType::Huuma | WeaponType::Shuriken => {
                match target_status.size {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0
                }
            }
            _ => 1.0
        }
    }

    pub fn basic_attack(&self, character: &mut Character, target: MapItem, tick: u128) -> Option<Damage> {
        character.attack?;
        let attack = character.attack();

        let attack_motion = self.status_service.attack_motion(&character.status);

        if tick < attack.last_attack_tick + attack_motion as u128 {
            return None;
        }
        if !attack.repeat { // one shot attack
            character.clear_attack();
        } else {
            character.update_last_attack_tick(tick);
            character.update_last_attack_motion(attack_motion);
        }
        let mut packet_zc_notify_act3 = PacketZcNotifyAct::new(self.configuration_service.packetver());
        packet_zc_notify_act3.set_target_gid(attack.target);
        packet_zc_notify_act3.set_action(ActionType::Attack.value() as u8);
        packet_zc_notify_act3.set_gid(character.char_id);
        packet_zc_notify_act3.set_attack_mt(attack_motion as i32);
        packet_zc_notify_act3.set_attacked_mt(attack_motion as i32);
        let damage = if matches!(target.object_type(), MapItemType::Mob) {
            let mob = self.configuration_service.get_mob(target.client_item_class() as i32);
            packet_zc_notify_act3.set_attacked_mt(mob.damage_motion);
            self.damage_character_attack_monster(&character.status, &StatusFromDb::from_mob_model(&mob), 1.0)
        } else {
            0
        };
        packet_zc_notify_act3.set_damage(damage as i16);
        packet_zc_notify_act3.set_count(1);
        packet_zc_notify_act3.fill_raw();
        self.client_notification_sender.send(Notification::Area(AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_notify_act3.raw_mut())))).expect("Failed to send notification to client");
        Some(Damage {
            target_id: attack.target,
            attacker_id: character.char_id,
            damage,
            attacked_at: tick + attack_motion as u128,
        })
    }
}