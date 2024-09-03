use std::any::Any;
use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::EnumWithNumberValue;
use models::enums::skill::{SkillTargetType, UseSkillFailure};
use models::enums::status::StatusEffect;
use models::enums::weapon::{AmmoType};
use models::item::{NormalInventoryItem};
use models::status::{StatusSnapshot};
use models::status_bonus::TemporaryStatusBonuses;

pub mod skill_enums;
pub mod base;
pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait SkillBase {
    fn _level(&self) -> u8;
    fn _id(&self) -> u32;

    fn as_any(&self) -> &dyn Any;
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> { None }
    #[inline(always)]
    fn is_supportive_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> { None }
    #[inline(always)]
    fn is_interactive_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> { None }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> { None }
    #[inline(always)]
    fn is_performance_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_performance_skill(&self) -> Option<&dyn PerformanceSkill> { None }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool { false }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> { None }

    #[inline(always)]
    fn _validate_sp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_hp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_state(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_zeny(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_spirit_sphere(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    #[inline(always)]
    fn _validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }

    #[inline(always)]
    fn _validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_weapon(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }
    #[inline(always)]
    fn _validate_range(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        Ok(())
    }

    #[inline(always)]
    fn _skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn _update_cast_time(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, _new_value: u32) {}
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, _new_value: u32) {}

    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        0
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        0
    }

    fn _range(&self) -> i8;
    fn _max_level(&self) -> u8;
    fn _sp_cost(&self) -> u16;
    fn _target_type(&self) -> SkillTargetType;
    fn _is_ranged(&self) -> bool;

    fn _is_magic(&self) -> bool;
    fn _is_physical(&self) -> bool;
    // Support skills can provide bonus to self, like Improve concentration
    // Passive skills can provide bonus to self, like demon bane
    // Offensive skills can provide bonus to self, like spiral spear
    #[inline(always)]
    fn _bonuses_to_self(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Support skills can provide bonus to self, like Increase Agi
    #[inline(always)]
    fn _bonuses_to_target(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Support skills can provide bonus to self, like Maximize power
    #[inline(always)]
    fn _bonuses_to_party(&self, _tick: u128) -> TemporaryStatusBonuses {
        TemporaryStatusBonuses::empty()
    }

    // Type in packet to send to client. it is use by the client to determine if target is valid or if it should display specific cursor
    #[inline(always)]
    fn _client_type(&self) -> usize {
        self._target_type().value()
    }

    // Increase chance of success of skills, like FullStripe
    #[inline(always)]
    fn _success_chance(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot) -> f32 {
        100.0
    }

    #[inline(always)]
    fn _dispell_skills(&self) -> Vec<u32> {
        vec![]
    }

    #[inline(always)]
    fn _has_bonuses_to_self(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _has_bonuses_to_target(&self) -> bool {
        false
    }
    #[inline(always)]
    fn _has_bonuses_to_party(&self) -> bool {
        false
    }
}

pub trait Skill: SkillBase {
    fn as_base(&self) -> &dyn SkillBase
    where
        Self: Sized,
    {
        self as &dyn SkillBase
    }
    fn as_base_mut(&mut self) -> &mut dyn SkillBase
    where
        Self: Sized,
    {
        self as &mut dyn SkillBase
    }

    fn new(level: u8) -> Option<Self>
    where
        Self: Sized;
    fn level(&self) -> u8 {
        self._level()
    }
    fn id(&self) -> u32 {
        self._id()
    }

    #[inline(always)]
    fn range(&self) -> i8 {
        self._range()
    }

    #[inline(always)]
    fn is_ranged(&self) -> bool {
        self._is_ranged()
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        self._max_level()
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        self._sp_cost()
    }
    #[inline(always)]
    fn target_type(&self) -> SkillTargetType {
        self._target_type()
    }

    #[inline(always)]
    fn validate_sp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        self._validate_sp(_status)
    }
    #[inline(always)]
    fn validate_hp(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        self._validate_hp(_status)
    }
    #[inline(always)]
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        self._validate_ammo(_character_ammo)
    }
    #[inline(always)]
    fn validate_state(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        self._validate_state(_status)
    }
    #[inline(always)]
    fn validate_zeny(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        self._validate_zeny(_status)
    }
    #[inline(always)]
    fn validate_spirit_sphere(&self, _status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        self._validate_spirit_sphere(_status)
    }
    #[inline(always)]
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        self._validate_item(_item)
    }

    #[inline(always)]
    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        self._validate_target(_target_type)
    }
    #[inline(always)]
    fn validate_weapon(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        self._validate_weapon(_status)
    }
    #[inline(always)]
    fn validate_range(&self, _status: &StatusSnapshot) -> SkillRequirementResult<()> {
        self._validate_range(_status)
    }

    #[inline(always)]
    fn skip_item_validation(&self, _state: Option<u64>) -> bool {
        self._skip_item_validation(_state)
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        self._base_cast_time()
    }
    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        self._base_after_cast_act_delay()
    }
    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        self._base_after_cast_walk_delay()
    }

    #[inline(always)]
    fn update_cast_time(&mut self, _new_value: u32) {
        self._update_cast_time(_new_value)
    }
    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {
        self._update_after_cast_act_delay(_new_value)
    }
    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, _new_value: u32) {
        self._update_after_cast_walk_delay(_new_value)
    }

    #[inline(always)]
    fn cast_time(&self) -> u32 {
        self._cast_time()
    }
    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32 {
        self._after_cast_act_delay()
    }
    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        self._after_cast_walk_delay()
    }

    #[inline(always)]
    fn is_magic(&self) -> bool {
        self._is_magic()
    }
    #[inline(always)]
    fn is_physical(&self) -> bool {
        self._is_physical()
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        self._bonuses_to_self(tick)
    }

    #[inline(always)]
    fn bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        self._bonuses_to_target(tick)
    }

    #[inline(always)]
    fn bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        self._bonuses_to_party(tick)
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        self._client_type()
    }

    #[inline(always)]
    fn success_chance(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot) -> f32 {
        100.0
    }

    #[inline(always)]
    fn dispell_skills(&self) -> Vec<u32> {
        self._dispell_skills()
    }
    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        self._has_bonuses_to_self()
    }
    #[inline(always)]
    fn has_bonuses_to_target(&self) -> bool {
        self._has_bonuses_to_target()
    }
    #[inline(always)]
    fn has_bonuses_to_party(&self) -> bool {
        self._has_bonuses_to_party()
    }
}

pub trait OffensiveSkillBase: Skill {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        0
    }
    #[inline(always)]
    fn _dmg_atk(&self) -> Option<f32> {
        None
    }
    #[inline(always)]
    fn _dmg_matk(&self) -> Option<f32> {
        None
    }
    fn _element(&self) -> Element;
    #[inline(always)]
    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
    #[inline(always)]
    fn _inflict_status_effect_to_self(&self, _status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Option<StatusEffect> {
        None
    }
    fn _damage_if_failed(&self) -> f64 {
        0.0
    }
}
pub trait OffensiveSkill: OffensiveSkillBase {
    #[inline(always)]
    fn hit_count(&self) -> i8 {
        self._hit_count()
    }
    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        self._dmg_atk()
    }
    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        self._dmg_matk()
    }
    #[inline(always)]
    fn element(&self) -> Element {
        self._element()
    }
    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        self._inflict_status_effect_to_target(_status, _target_status, _rng)
    }
    #[inline(always)]
    fn inflict_status_effect_to_self(&self, _status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Option<StatusEffect> {
        self._inflict_status_effect_to_self(_status, _rng)
    }
    fn damage_if_failed(&self) -> f64 {
        self._damage_if_failed()
    }
}

pub trait SupportiveSkillBase: Skill {
}
pub trait SupportiveSkill: SupportiveSkillBase {}

pub trait PerformanceSkillBase: Skill {}
pub trait PerformanceSkill: PerformanceSkillBase {}

pub trait PassiveSkillBase: Skill {}
pub trait PassiveSkill: PassiveSkillBase {}

pub trait GroundSkillBase: Skill {
    fn _mitigate_skills(&self) -> Vec<u32> {
        vec![]
    }
}
pub trait GroundSkill: GroundSkillBase {
    fn mitigate_skills(&self) -> Vec<u32> {
       self._mitigate_skills()
    }
}
pub trait InteractiveSkillBase: Skill {}
pub trait InteractiveSkill: InteractiveSkillBase {}