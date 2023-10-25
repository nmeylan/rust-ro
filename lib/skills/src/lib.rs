use enums::skill::{SkillDamageType, SkillTargetType, SkillType};
use enums::weapon::WeaponType;
use models::weapon::Weapon;

pub mod skill_enums;
// mod skills;

type SkillRequirementResult<T> = std::result::Result<T, String>;

pub trait Skill {
    fn skill_type(&self) -> SkillTargetType;

    fn validate_sp(&self, level: u8, character_sp: u32) -> SkillRequirementResult<u32>;
    fn validate_hp(&self, level: u8, character_hp: u32) -> SkillRequirementResult<u32>;
    // fn validate_ammo(&self, level: u8, character_hp: u32) -> SkillRequirementResult<u32>;
    fn validate_level(&self, level: u8, character_level: u32) -> SkillRequirementResult<()>;
    fn validate_weapon(&self, level: u8, character_weapon: Weapon) -> SkillRequirementResult<()>;
    fn validate_range(&self, level: u8, character_weapon: Weapon) -> SkillRequirementResult<()>;

}

pub trait OffensiveSkill : Skill {
    fn hit_count(&self, level: u8) -> u8;
    fn cast_delay(&self, level: u8) -> u32;
    fn after_cast_delay(&self, level: u8) -> u32;

    // fn damage(&self, level: u8) -> u32;
    // fn skill_damage_type(&self) -> SkillDamageType;

}

pub trait SupportiveSkill {

}
