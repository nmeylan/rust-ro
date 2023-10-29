use enums::skill::{SkillState, SkillTargetType};
use enums::weapon::{AmmoType};
use models::item::NormalInventoryItem;
use models::weapon::Weapon;

pub mod skill_enums;
pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait Skill {
    fn new(level: u8) -> Option<Self> where Self: Sized;
    fn delegate(&self) -> &Option<Box<dyn DelegateSkill>>;
    fn level(&self) -> u8;

    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32>;
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32>;
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32>;
    fn validate_state(&self, state: Option<SkillState>) -> SkillRequirementResult<()>;
    fn validate_zeny(&self, zeny: u32) -> SkillRequirementResult<u32>;
    fn validate_spirit_sphere(&self, spirit_sphere: u32) -> SkillRequirementResult<u32>;
    fn validate_item(&self, item: &Vec<NormalInventoryItem>) -> SkillRequirementResult<Option<NormalInventoryItem>>;

    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()>;
    fn validate_weapon(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()>;
    fn validate_range(&self, character_weapon: Option<Weapon>) -> SkillRequirementResult<()>;

    fn validate_delegate(&self) -> SkillRequirementResult<()> {
        if self.delegate().is_none() {
            return Ok(());
        }
        self.delegate().as_ref().unwrap().validate_extra()
    }
    fn validate_override(&self) -> SkillRequirementResult<()> {
        Ok(())
    }

    fn hit_count(&self) -> i8;
    fn cast_delay(&self) -> u32;
    fn after_cast_act_delay(&self) -> u32;
    fn after_cast_walk_delay(&self) -> u32;

    fn to_skill(&self) -> Box<&dyn Skill>  where Self: Sized {
        Box::new(self)
    }
}

pub trait DelegateSkill {
    fn validate_extra(&self) -> SkillRequirementResult<()>;
}

