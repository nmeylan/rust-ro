use enums::skill::{SkillTargetType, UseSkillFailure};
use enums::weapon::{AmmoType};
use models::item::{NormalInventoryItem, WearWeapon};

pub mod skill_enums;
pub mod base;
pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait SkillBase {
    fn _level(&self) -> u8;
    fn _id(&self) -> u32;
    fn _validate_sp(&self, _character_sp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_hp(&self, _character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_state(&self, _state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_zeny(&self, _zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_spirit_sphere(&self, _spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn _validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }

    fn _validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_weapon(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn _validate_range(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }

    fn _skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    fn _hit_count(&self) -> i8 {
        0
    }
    fn _base_cast_time(&self) -> u32 {
        0
    }
    fn _base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    fn _update_cast_time(&mut self, _new_value: u32) {}
    fn _update_after_cast_act_delay(&mut self, _new_value: u32) {}
    fn _update_after_cast_walk_delay(&mut self, _new_value: u32) {}

    fn _cast_time(&self) -> u32 {
        0
    }
    fn _after_cast_act_delay(&self) -> u32 {
        0
    }
    fn _after_cast_walk_delay(&self) -> u32 {
        0
    }
}

pub trait Skill: SkillBase {
    fn as_base(&self) -> &dyn SkillBase where Self: Sized {
        self as &dyn SkillBase
    }
    fn as_base_mut(&mut self) -> &mut dyn SkillBase where Self: Sized {
        self as &mut dyn SkillBase
    }

    fn new(level: u8) -> Option<Self> where Self: Sized;
    fn level(&self) -> u8 {
        self._level()
    }
    fn id(&self) -> u32 {
        self._id()
    }

    fn validate_sp(&self, _character_sp: u32) -> SkillRequirementResult<u32> {
        self._validate_sp(_character_sp)
    }
    fn validate_hp(&self, _character_hp: u32) -> SkillRequirementResult<u32> {
        self._validate_hp(_character_hp)
    }
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        self._validate_ammo(_character_ammo)
    }
    fn validate_state(&self, _state: Option<u64>) -> SkillRequirementResult<()>  {
        self._validate_state(_state)
    }
    fn validate_zeny(&self, _zeny: u32) -> SkillRequirementResult<u32>  {
        self._validate_zeny(_zeny)
    }
    fn validate_spirit_sphere(&self, _spirit_sphere: u32) -> SkillRequirementResult<u32> {
        self._validate_spirit_sphere(_spirit_sphere)
    }
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        self._validate_item(_item)
    }

    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        self._validate_target(_target_type)
    }
    fn validate_weapon(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        self._validate_weapon(_character_weapon)
    }
    fn validate_range(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        self._validate_range(_character_weapon)
    }

    fn skip_item_validation(&self, _state: Option<u64>) -> bool  {
        self._skip_item_validation(_state)
    }

    fn hit_count(&self) -> i8 {
        self._hit_count()
    }
    fn base_cast_time(&self) -> u32 {
        self._base_cast_time()
    }
    fn base_after_cast_act_delay(&self) -> u32{
        self._base_after_cast_act_delay()
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        self._base_after_cast_walk_delay()
    }

    fn update_cast_time(&mut self, _new_value: u32)  {
        self._update_cast_time(_new_value)
    }
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {
        self._update_after_cast_act_delay(_new_value)
    }
    fn update_after_cast_walk_delay(&mut self, _new_value: u32)  {
        self._update_after_cast_walk_delay(_new_value)
    }

    fn cast_time(&self) -> u32 {
        self._cast_time()
    }
    fn after_cast_act_delay(&self) -> u32  {
        self._after_cast_act_delay()
    }
    fn after_cast_walk_delay(&self) -> u32 {
        self._after_cast_walk_delay()
    }
}
