use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::Once;

use models::enums::action::ActionType;
use models::enums::element::Element;
use models::enums::EnumWithMaskValueU64;
use models::enums::size::Size;
use models::enums::skill::SkillState;
use models::enums::weapon::WeaponType;
use models::status::{StatusSnapshot};
use packets::packets::PacketZcNotifyAct;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType};
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use skills::OffensiveSkill;

use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use models::enums::EnumWithNumberValue;
use crate::packets::packets::Packet;
use crate::server::model::action::Damage;

static mut SERVICE_INSTANCE: Option<BattleService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct BattleService {
    client_notification_sender: SyncSender<Notification>,
    status_service: &'static StatusService,
    configuration_service: &'static GlobalConfigService,
    battle_result_mode: BattleResultMode,
}

pub enum BattleResultMode {
    TestMin,
    TestMax,
    Normal,
}

impl BattleService {
    pub fn instance() -> &'static BattleService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, status_service: &'static StatusService, configuration_service: &'static GlobalConfigService, battle_result_mode: BattleResultMode) -> Self {
        BattleService { client_notification_sender, status_service, configuration_service, battle_result_mode }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, status_service: &'static StatusService, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(BattleService::new(client_notification_sender, status_service, configuration_service, BattleResultMode::Normal));
        });
    }

    pub fn calculate_damage(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, skill: Option<&dyn OffensiveSkill>) -> i32 {
        let mut damage = 0;
        if let Some(skill) = skill {
            if skill.is_physical() {
                let mut skill_modifier = skill.dmg_atk().unwrap_or(1.0);
                if skill.hit_count() > 1 {
                    skill_modifier /= skill.hit_count() as f32;
                }
                damage = self.physical_damage_character_attack_monster(source_status, target_status, skill_modifier, skill.is_ranged());
                if skill.hit_count() > 1 {
                    damage *= skill.hit_count() as i32;
                } else {
                    damage = ((damage as f32 / skill.hit_count().abs() as f32).floor() * skill.hit_count().abs() as f32) as i32;
                }
            } else if skill.is_magic() {
                let mut skill_modifier = skill.dmg_matk().unwrap_or(1.0);
                if skill.hit_count() > 1 {
                    skill_modifier /= skill.hit_count() as f32;
                }
                damage = self.magic_damage_character_attack_monster(source_status, target_status, skill_modifier, &skill.element());
                if skill.hit_count() > 1 {
                    damage *= skill.hit_count() as i32;
                }
            }
        } else {
            let is_ranged = source_status.right_hand_weapon().map(|w| w.weapon_type().is_ranged()).unwrap_or(false);
            damage = self.physical_damage_character_attack_monster(source_status, target_status, 1.0, is_ranged);
        }

        damage
    }

    /// (([((({(base_atk +
    /// + rnd(min(DEX,ATK), ATK)*SizeModifier) * SkillModifiers * (1 - DEF/100) - VitDEF + BaneSkill + UpgradeDamage}
    /// + MasterySkill + WeaponryResearchSkill + EnvenomSkill) * ElementalModifier) + Enhancements) * DamageBonusModifiers * DamageReductionModifiers] * NumberOfMultiHits) - KyrieEleisonEffect) / NumberOfMultiHits
    fn physical_damage_character_attack_monster(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, skill_modifier: f32, is_ranged: bool) -> i32 {
        let upgrade_bonus: f32 = 0.0; // TODO: weapon level1 : (+1~3 ATK for every overupgrade). weapon level2 : (+1~5 ATK for every overupgrade). weapon level3 : (+1~7 ATK for every overupgrade). weapon level4 : (+1~13 ATK for every overupgrade).
        let _imposito_magnus: u32 = 0;
        let base_atk = self.status_service.fist_atk(source_status, is_ranged) as f32 + upgrade_bonus + source_status.base_atk() as f32;

        let def: f32 = target_status.def() as f32 / 100.0;

        /// MOB vit def: VIT + rnd(0,[VIT/20]^2-1).
        let vitdef: f32 = self.mob_vitdef(target_status); // TODO set to 0 if critical hit
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
                                            (base_atk + self.weapon_atk(source_status, target_status, is_ranged) as f32).floor() * skill_modifier * (1.0 - def)
                                        )
                                            - vitdef + bane_skill + source_status.weapon_upgrade_damage() as f32
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
        ).floor() as i32
    }

    pub fn mob_vitdef(&self, target_status: &StatusSnapshot) -> f32 {
        let mut rng = fastrand::Rng::new();
        target_status.vit() as f32 + rng.u16(0..1.max(1.max(((target_status.vit() as f32 / 20.0).ceil() as u16).pow(2)) - 1)) as f32
    }


    ///  [VIT*0.5] + rnd([VIT*0.3], max([VIT*0.3],[VIT^2/150]-1))
    pub fn player_vitdef(&self, target_status: &StatusSnapshot) -> u16 {
        let mut rng = fastrand::Rng::new();
        let vitdef = (target_status.vit() as f32 * 0.5).floor()  as u16;
        let vitdef_lower_part= (target_status.vit() as f32 * 0.3).floor() as u16;
        let vitdef_higher_part= vitdef_lower_part.max(((target_status.vit().pow(2) as f32 / 150.0).floor() - 1.0 ) as u16);
        // TODO handle bDef2Rate, bDef2 (and also from angelus and divine protection)
        match self.battle_result_mode {
            BattleResultMode::TestMin => {vitdef + vitdef_lower_part}
            BattleResultMode::TestMax => {vitdef + vitdef_higher_part}
            BattleResultMode::Normal => {vitdef + rng.u16(vitdef_lower_part..=vitdef_higher_part)}
        }
    }

    //  rnd(min(DEX*(0.8+0.2*WeaponLevel),ATK), ATK)
    pub fn weapon_atk(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, _is_ranged: bool) -> u32 {
        let mut rng = fastrand::Rng::new();
        let weapon = source_status.right_hand_weapon().map(|weapon| self.configuration_service.get_item(weapon.item_id()));
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

        let work_dex = (source_status.dex() as f32 * (0.8 + 0.2 * weapon_level as f32)).round() as u32;
        let mut weapon_max_attack: u32 = 0;
        let weapon_over_upgrade_max: u32 = 0;
        let weapon_over_upgrade_min: u32 = 0;
        let mut weapon_min_attack: u32 = 0;
        let size_modifier = Self::size_modifier(source_status, target_status);
        if work_dex >= weapon_attack { // todo || maximize power skill
            weapon_max_attack = weapon_over_upgrade_max + ((weapon_attack + imposito_magnus) as f32 * size_modifier).floor() as u32;
            weapon_min_attack = weapon_over_upgrade_min + ((weapon_attack + imposito_magnus) as f32 * size_modifier).floor() as u32;
        } else {
            weapon_max_attack = weapon_over_upgrade_max + ((weapon_attack - 1 + imposito_magnus) as f32 * size_modifier).floor() as u32;
            weapon_min_attack = weapon_over_upgrade_min + ((work_dex + imposito_magnus) as f32 * size_modifier).floor() as u32;
        }
        if source_status.right_hand_weapon_type().is_ranged() {
            let ammo_dmg = (source_status.ammo().map_or_else(|| 1.0, |ammo| (ammo.attack() - 1) as f32) * size_modifier).floor() as u32;
            weapon_max_attack += ammo_dmg;
            let mut w1 = weapon_over_upgrade_max + ((weapon_attack * weapon_attack) as f32 / 100.0 * size_modifier).floor() as u32 + (imposito_magnus as f32 * size_modifier).floor() as u32 + ammo_dmg;
            let w2 = weapon_over_upgrade_max + ((weapon_attack * work_dex) as f32 / 100.0 * size_modifier).floor() as u32 + (imposito_magnus as f32 * size_modifier).floor() as u32 + ammo_dmg;
            if w1 > w2 { w1 = w2 }
            if weapon_max_attack < w1 { weapon_max_attack = w1 }
            weapon_min_attack = weapon_over_upgrade_min + (((weapon_attack * weapon_attack) as f32 / 100.0 + imposito_magnus as f32) * size_modifier).floor() as u32;
            let w = weapon_over_upgrade_min + (((weapon_attack * work_dex) as f32 / 100.0 + imposito_magnus as f32) * size_modifier).floor() as u32;
            if weapon_min_attack > w { weapon_min_attack = w }
        }

        match self.battle_result_mode {
            BattleResultMode::TestMin => { weapon_min_attack }
            BattleResultMode::TestMax => { weapon_max_attack }
            BattleResultMode::Normal => { rng.u32(weapon_min_attack..=weapon_max_attack) }
        }
    }


    /// {rnd(minMATK,maxMATK) * ItemModifier * SkillModifier * (1-MDEF/100) - INT - VIT/2} * Elemental Modifier
    pub fn magic_damage_character_attack_monster(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, skill_modifier: f32, element: &Element) -> i32 {
        let mut rng = fastrand::Rng::new();
        let matk = match self.battle_result_mode {
            BattleResultMode::TestMin => { source_status.matk_min() }
            BattleResultMode::TestMax => { source_status.matk_max() }
            BattleResultMode::Normal => { rng.u16(source_status.matk_min()..=source_status.matk_max()) }
        } as f32;
        let elemental_modifier: f32 = Self::element_modifier(element, target_status);
        let mdef = target_status.mdef() as f32 / 100.0;
        // println!("({} * {} * {} * {} - {} - {}) * {}", matk, item_modifier, skill_modifier, (1.0 - mdef), target_status.int(), target_status.vit() as f32 / 2.0, elemental_modifier);
        ((matk * skill_modifier * (1.0 - mdef)).floor() * elemental_modifier).floor() as i32
    }

    pub fn attack_element(&self, source_status: &StatusSnapshot, skill: Option<&dyn OffensiveSkill>) -> Element {
        if let Some(skill) = skill {
            if matches!(skill.element(), Element::Ammo) {
                source_status.ammo().map(|ammo| *ammo.element()).unwrap_or(Element::Neutral)
            } else if matches!(skill.element(), Element::Weapon) {
                source_status.right_hand_weapon().map(|weapon| *weapon.element()).unwrap_or(Element::Neutral)
            } else {
                skill.element()
            }
        } else if let Some(weapon) = source_status.right_hand_weapon() {
            if weapon.weapon_type().is_ranged() {
                source_status.ammo().map(|ammo| *ammo.element()).unwrap_or(Element::Neutral)
            } else {
                source_status.right_hand_weapon().map(|weapon| *weapon.element()).unwrap_or(Element::Neutral)
            }
        } else {
            Element::Neutral
        }
    }

    pub fn basic_attack(&self, character: &mut Character, target: MapItemSnapshot, source_status: &StatusSnapshot, target_status: &StatusSnapshot, tick: u128) -> Option<Damage> {
        character.attack?;
        let attack = character.attack();

        let attack_motion = self.status_service.attack_motion(source_status);

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
        let damage = if matches!(target.map_item.object_type(), MapItemType::Mob) {
            let mob = self.configuration_service.get_mob(target.map_item.client_item_class() as i32);
            packet_zc_notify_act3.set_attacked_mt(mob.damage_motion);
            self.physical_damage_character_attack_monster(source_status, target_status, 1.0, source_status.right_hand_weapon_type().is_ranged())
        } else {
            0
        };
        packet_zc_notify_act3.set_damage(damage as i16);
        packet_zc_notify_act3.set_count(1);
        packet_zc_notify_act3.fill_raw();
        self.client_notification_sender.send(
            Notification::Area(AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(),
                                                     AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_notify_act3.raw_mut())))).expect("Failed to send notification to client");
        if damage >= 0 {
            Some(Damage {
                target_id: attack.target,
                attacker_id: character.char_id,
                damage: damage as u16 as u32,
                attacked_at: tick + attack_motion as u128,
            })
        } else {
            // TODO handle heal if damage < 0
            None
        }
    }

    #[inline]
    pub fn size_modifier(source_status: &StatusSnapshot, target_status: &StatusSnapshot) -> f32 {
        // Size Modifiers for Weapons
        // Size 	Fist 	Dagger 	1H Sword 	2H Sword 	Spear 	Spear+Peco 	Axe 	Mace 	Rod 	Bow 	Katar 	Book 	Claw 	Instrument 	Whip 	Gun 	Huuma Shuriken
        // Small 	100 	100 	75       	75       	75  	75 	        50  	75  	100 	100 	75  	100 	100 	75 	        75 	    100 	100
        // Medium 	100 	75  	100      	75       	75  	100 	    75  	100 	100 	100 	100 	100 	75 	    100 	    100 	100 	100
        // Large 	100 	50  	75      	100     	100 	100 	    100 	100 	100 	75  	75 	    50 	    50 	    75 	        50 	    100 	100
        let weapon_type = source_status.right_hand_weapon_type();
        match weapon_type {
            WeaponType::Fist => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Dagger => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 0.75,
                    Size::Large => 0.5,
                    _ => 1.0
                }
            }
            WeaponType::Sword1H => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.75,
                    _ => 1.0
                }
            }
            WeaponType::Sword2H => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 0.75,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Spear1H | WeaponType::Spear2H => {
                if source_status.state() & SkillState::Riding.as_flag() > 0 {
                    match target_status.size() {
                        Size::Small => 0.75,
                        Size::Medium => 1.0,
                        Size::Large => 1.0,
                        _ => 1.0
                    }
                } else {
                    match target_status.size() {
                        Size::Small => 0.75,
                        Size::Medium => 0.75,
                        Size::Large => 1.0,
                        _ => 1.0
                    }
                }
            }
            WeaponType::Axe1H | WeaponType::Axe2H => {
                match target_status.size() {
                    Size::Small => 0.5,
                    Size::Medium => 0.75,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Mace | WeaponType::Mace2H => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Staff | WeaponType::Staff2H => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Bow => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 0.75,
                    _ => 1.0
                }
            }
            WeaponType::Knuckle => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 0.75,
                    Size::Large => 0.5,
                    _ => 1.0
                }
            }
            WeaponType::Musical => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.75,
                    _ => 1.0
                }
            }
            WeaponType::Whip => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.5,
                    _ => 1.0
                }
            }
            WeaponType::Book => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 0.5,
                    _ => 1.0
                }
            }
            WeaponType::Katar => {
                match target_status.size() {
                    Size::Small => 0.75,
                    Size::Medium => 1.0,
                    Size::Large => 0.75,
                    _ => 1.0
                }
            }
            WeaponType::Revolver | WeaponType::Rifle | WeaponType::Gatling | WeaponType::Shotgun | WeaponType::Grenade => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            WeaponType::Huuma | WeaponType::Shuriken => {
                match target_status.size() {
                    Size::Small => 1.0,
                    Size::Medium => 1.0,
                    Size::Large => 1.0,
                    _ => 1.0
                }
            }
            _ => 1.0
        }
    }

    #[inline]
    pub fn element_modifier(element: &Element, target_status: &StatusSnapshot) -> f32 {
        if target_status.element_level() == 1 || target_status.element_level() == 0 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.25
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        0.25
                    } else if matches!(element, Element::Fire) {
                        0.5
                    } else if matches!(element, Element::Wind) {
                        1.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        1.5
                    } else if matches!(element, Element::Wind) {
                        0.5
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        1.5
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Fire) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.5
                    } else if matches!(element, Element::Earth) {
                        1.5
                    } else if matches!(element, Element::Wind) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Dark) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.25
                    } else if matches!(element, Element::Holy) {
                        0.0
                    } else if matches!(element, Element::Undead) || matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.75
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        0.0
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else if matches!(element, Element::Poison) {
                        0.5
                    } else if matches!(element, Element::Ghost) {
                        0.75
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.25
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        1.0
                    }
                }
                _ => 1.0
            }
        } else if target_status.element_level() == 2 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.25
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        0.25
                    } else if matches!(element, Element::Wind) {
                        1.75
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        1.75
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Wind) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        1.75
                    } else if matches!(element, Element::Earth) {
                        0.25
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.25
                    } else if matches!(element, Element::Earth) {
                        1.75
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        1.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Dark) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.5
                    } else if matches!(element, Element::Holy) {
                        -0.25
                    } else if matches!(element, Element::Undead) {
                        1.25
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.5
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        0.5
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else {
                        0.75
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.25
                    } else if matches!(element, Element::Poison) {
                        0.75
                    } else if matches!(element, Element::Ghost) {
                        1.5
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.5
                    } else if matches!(element, Element::Holy) {
                        1.75
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.5
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.25
                    } else {
                        1.0
                    }
                }
                _ => 1.0
            }
        } else if target_status.element_level() == 3 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        -0.25
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        -0.25
                    } else if matches!(element, Element::Poison) {
                        1.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Earth) {
                        2.0
                    } else if matches!(element, Element::Wind) {
                        -0.25
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.5
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Undead) || matches!(element, Element::Dark) || matches!(element, Element::Poison) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.5
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else {
                        1.0
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        1.75
                    } else if matches!(element, Element::Holy) {
                        -0.5
                    } else if matches!(element, Element::Undead) {
                        1.5
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.25
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        -0.5
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        0.25
                    } else {
                        0.5
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        0.5
                    } else if matches!(element, Element::Ghost) {
                        1.75
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) {
                        1.75
                    } else if matches!(element, Element::Water) {
                        1.25
                    } else if matches!(element, Element::Earth) {
                        0.75
                    } else if matches!(element, Element::Holy) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -0.75
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.5
                    } else {
                        1.0
                    }
                }
                _ => 1.0
            }
        } else if target_status.element_level() == 4 {
            match target_status.element() {
                Element::Neutral => {
                    if matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        1.0
                    }
                }
                Element::Water => {
                    if matches!(element, Element::Water) {
                        -0.5
                    } else if matches!(element, Element::Fire) {
                        0.0
                    } else if matches!(element, Element::Wind) {
                        2.0
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Earth => {
                    if matches!(element, Element::Fire) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.25
                    } else if matches!(element, Element::Wind) {
                        0.0
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Fire => {
                    if matches!(element, Element::Water) {
                        2.0
                    } else if matches!(element, Element::Earth) {
                        0.0
                    } else if matches!(element, Element::Fire) {
                        -0.5
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Wind => {
                    if matches!(element, Element::Water) {
                        0.0
                    } else if matches!(element, Element::Earth) {
                        2.0
                    } else if matches!(element, Element::Wind) {
                        -0.5
                    } else if matches!(element, Element::Ghost) || matches!(element, Element::Undead) {
                        0.25
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Holy) || matches!(element, Element::Dark) {
                        0.75
                    } else {
                        1.0
                    }
                }
                Element::Poison => {
                    if matches!(element, Element::Undead) || matches!(element, Element::Dark) {
                        -0.25
                    } else if matches!(element, Element::Ghost) {
                        0.5
                    } else if matches!(element, Element::Holy) {
                        1.25
                    } else if matches!(element, Element::Poison) {
                        0.0
                    } else {
                        0.75
                    }
                }
                Element::Holy => {
                    if matches!(element, Element::Dark) {
                        2.0
                    } else if matches!(element, Element::Holy) {
                        -1.0
                    } else if matches!(element, Element::Undead) {
                        1.75
                    } else if matches!(element, Element::Neutral) {
                        1.0
                    } else {
                        0.0
                    }
                }
                Element::Dark => {
                    if matches!(element, Element::Dark) {
                        -0.5
                    } else if matches!(element, Element::Holy) {
                        1.5
                    } else if matches!(element, Element::Poison) {
                        0.25
                    } else if matches!(element, Element::Undead) || matches!(element, Element::Ghost) {
                        0.0
                    } else {
                        0.25
                    }
                }
                Element::Ghost => {
                    if matches!(element, Element::Neutral) {
                        0.0
                    } else if matches!(element, Element::Poison) {
                        0.25
                    } else if matches!(element, Element::Ghost) {
                        2.0
                    } else {
                        1.0
                    }
                }
                Element::Undead => {
                    if matches!(element, Element::Fire) || matches!(element, Element::Holy){
                        2.0
                    } else if matches!(element, Element::Water) {
                        1.5
                    } else if matches!(element, Element::Earth) {
                        0.5
                    } else if matches!(element, Element::Poison) || matches!(element, Element::Dark) {
                        -1.0
                    } else if matches!(element, Element::Undead) {
                        0.0
                    } else if matches!(element, Element::Ghost) {
                        1.75
                    } else {
                        1.0
                    }
                }
                _ => 1.0
            }
        } else {
            1.0
        }

    }
}