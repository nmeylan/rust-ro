use enum_macro::{WithMaskValueU32, WithNumberValue, WithStringValue};

use crate::enums::{EnumWithMaskValueU32, EnumWithNumberValue, EnumWithStringValue};

#[derive(WithNumberValue, WithStringValue, WithMaskValueU32, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EffectTrigger {
    #[value_string = "ATF_SELF"]
    TargetMySelf,
    #[value_string = "ATF_TARGET"]
    TargetAttacked,
    #[value_string = "ATF_SHORT"]
    MeleeAttack,
    #[value_string = "ATF_LONG"]
    RangedAttack,
    #[value_string = "ATF_SKILL"]
    MagicOrMisckSkillAttack,
    #[value_string = "ATF_WEAPON"]
    PhysicalAttack,
    #[value_string = "ATF_MAGIC"]
    MagicSkillAttack,
    #[value_string = "ATF_MISC"]
    MiscSkillAttack,
    #[value_string = "BF_NORMAL"]
    BfNormal,
    #[value_string = "BF_SKILL"]
    BfSkill,
}
