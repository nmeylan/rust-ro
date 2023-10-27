#![allow(dead_code)]
use crate::{EnumWithNumberValue, EnumWithStringValue, EnumWithMaskValueU64};
use crate::{WithNumberValue, WithStringValue, WithMaskValueU64};
#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillRequirement {
    #[value = 0]
    None,
    Hidding,
    Riding,
    Falcon,
    Cart,
    Shield,
    RecoverWeightRate,
    MoveEnable,
    Water,
    RidingDragon,
    Wug,
    RidingWug,
    Mado,
    ElementalSpirit,
    ElementalSpirit2,
    Peco,
    SunStance,
    MoonStance,
    StarStance,
    UniverseStance,
    Sight,
    WeaponBlockOn,
    QdShotReady,
    ExplosionSpirits,
    CartBoost,
    PoisoningWeapon,
    RollingCutter
}

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillType {
    Magic,
    Weapon,
    Misc
}

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Default)]
pub enum SkillTargetType {
    #[value = 0]
    #[default]
    Passive,
    Attack,
    Ground,
    #[value_string = "Self"]
    MySelf,
    #[value = 10]
    Support,
    #[value = 11]
    Trap,
}




#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillDamageFlags {
    #[mask_value = 0]
    Nodamage,
    Splash,
    Splashsplit,
    Ignoreatkcard,
    Ignoreelement,
    Ignoredefense,
    Ignoreflee,
    Ignoredefcard,
    Critical,
    Ignorelongcard,
    Max,
}

#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillFlags {
    #[mask_value = 0]
    Permanent,
    //NPC skills are those that players can't have in their skill tree.
    Isnpc,
    Iswedding,
    Isspirit,
    Isguild,
    Issong,
    Isensemble,
    Istrap,
    //Refers to ground placed skills that will target the caster as well (like Grandcross)
    Targetself,
    Notargetself,
    Partyonly,
    Guildonly,
    Notargetenemy,
    // Skill that available for SCAUTOSHADOWSPELL
    Isautoshadowspell,
    // Chorus skill
    Ischorus,
    // Skill that ignore bg reduction
    Ignorebgreduction,
    // Skill that ignore gvg reduction
    Ignoregvgreduction,
    // disable to cast skill if near with NPC
    Disablenearnpc,
    // can hit trap-type skill (ISTRAP)
    Targettrap,
    // Skill that can ignore Land Protector
    Ignorelandprotector,
    // Skill that can be use in hiding
    Allowwhenhidden,
    // Skill that can be use while in dancing state
    Allowwhenperforming,
    // Skill that could hit emperium
    Targetemperium,
    // Skill blocked by kagehumi
    Ignorekagehumi,
    // Skill shows AoE area while casting
    Alterrangevulture,
    Max,
    Isquest,
    AllowOnMado,
    AllowOnWarg,
    IgnoreHovering,
    IgnoreWugBite,
    TargetHidden,
    AlterRangeResearchTrap,
    AlterRangeSnakeEye,
    AlterRangeShadowJump,
    AlterRangeRadius,
    IgnoreAutoGuard,
    IgnoreCicada,
    ShowScale,
    IncreaseDanceWithWugDamage,
    TargetManHole
}

impl SkillFlags {
    pub fn is_permanent(&self) -> bool {
        *self == Self::Permanent
    }
}

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillDamageType {
    // damage [ damage: total damage, div: amount of hits, damage2: assassin dual-wield damage ]
    Normal,
    // pick up item
    #[value_string = "Pickup_Item"]
    PickupItem,
    #[value_string = "Sit_Down"]
    SitDown,
    #[value_string = "Stand_up"]
    StandUp,
    Endure,
    Splash,
    Single,
    Repeat,
    // multi-hit damage
    MultiHit,
    // multi-hit damage (endure)
    MultiHitEndure,
    Critical,
    LucyDodge,
    Touch,
    MultiHitCritical,
}

#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillCopyType {
    Plagiarism,
    Reproduce,
}

#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillCastTimeDelayType {
    IgnoreDex,
    IgnoreStatus,
    IgnoreItemBonus,
}

#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillUnitType {
    None,
    // If 'defunit_not_enemy' is set, the target is changed to 'friend'
    NoEnemy,
    // Spell cannot be stacked
    NorEiteration,
    // Spell cannot be cast near/on targets
    NoFootset,
    // Spell effects do not overlap
    NoOverlap,
    // Only cells with a shootable path will be placed
    PathCheck,
    // May not target players
    NotPlayer,
    // May not target mobs
    NotMob,
    // May target skills
    Skill,
    // Dance
    Dance,
    // Duet
    Ensemble,
    // Song
    Song,
    // Spells should trigger both ontimer and onplace/onout/onleft effects.
    Dualmode,
    // Skill unit cannot be knocked back
    NoKnockback,
    // hack for ranged layout, only display center
    RangedSingleunit,
    // Immune to Crazy Weed removal
    CrazyWeedImmune,
    // removed by Fire Rain
    RemovedByFireRain,
    // knockback skill unit with its group instead of single unit
    KnockbackGroup,
    // Hidden trap
    HiddenTrap,
    Max,
}