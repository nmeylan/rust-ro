use enums::skill::{SkillDamageType, SkillTargetType, SkillType};
use enums::weapon::WeaponType;
use models::weapon::Weapon;

pub mod skill_enums;
// pub mod skills;

type SkillRequirementResult<T> = std::result::Result<T, ()>;

pub trait Skill {
    fn new(level: u8) -> Self;

    fn validate_sp(&self, character_sp: u32) -> SkillRequirementResult<u32>;
    fn validate_hp(&self, character_hp: u32) -> SkillRequirementResult<u32>;
    // fn validate_ammo(&self, level: u8, character_hp: u32) -> SkillRequirementResult<u32>;
    fn validate_level(&self, character_level: u32) -> SkillRequirementResult<()>;
    fn validate_target(&self, target_type: SkillTargetType) -> SkillRequirementResult<()>;
    fn validate_weapon(&self, character_weapon: Weapon) -> SkillRequirementResult<()>;
    fn validate_range(&self, character_weapon: Weapon) -> SkillRequirementResult<()>;

    fn hit_count(&self) -> u8;
    fn cast_delay(&self) -> u32;
    fn after_cast_delay(&self) -> u32;

}

