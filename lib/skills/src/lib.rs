use enums::skill::{SkillTargetType, UseSkillFailure};
use enums::weapon::{AmmoType};
use models::item::{NormalInventoryItem, WearWeapon};

pub mod skill_enums;
pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait Skill {
    fn new(level: u8) -> Option<Self> where Self: Sized;
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>> {
        &None
    }
    fn level(&self) -> u8;
    fn id(&self) -> u32;

    fn validate_sp(&self, _character_sp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_hp(&self, _character_hp: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_ammo(&self, _character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_state(&self, _state: Option<u64>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_zeny(&self, _zeny: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_spirit_sphere(&self, _spirit_sphere: u32) -> SkillRequirementResult<u32> {
        Ok(0)
    }
    fn validate_item(&self, _item: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        Ok(None)
    }

    fn validate_target(&self, _target_type: SkillTargetType) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_weapon(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }
    fn validate_range(&self, _character_weapon: Option<&WearWeapon>) -> SkillRequirementResult<()> {
        Ok(())
    }

    fn validate_delegate(&self) -> SkillRequirementResult<()> {
        if self.delegate().is_none() {
            return Ok(());
        }
        self.delegate().as_ref().unwrap().extra_validation()
    }
    fn validate_override(&self) -> SkillRequirementResult<()> {
        Ok(())
    }

    fn skip_item_validation(&self, _state: Option<u64>) -> bool {
        false
    }

    fn hit_count(&self) -> i8 {
        0
    }
    fn base_cast_time(&self) -> u32 {
        0
    }
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }
    fn base_after_cast_walk_delay(&self) -> u32 {
        0
    }

    fn update_cast_time(&mut self, _new_value: u32) {

    }
    fn update_after_cast_act_delay(&mut self, _new_value: u32) {

    }
    fn update_after_cast_walk_delay(&mut self, _new_value: u32) {

    }

    fn cast_time(&self) -> u32 {
        0
    }
    fn after_cast_act_delay(&self) -> u32 {
        0
    }
    fn after_cast_walk_delay(&self) -> u32 {
        0
    }
}

pub trait DelegateSkill {
    fn extra_validation(&self) -> SkillRequirementResult<()>;

    // fn damage_calculation(&self, attacker_status: Status, target_status: Option<Status>)
}

