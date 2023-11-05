#![allow(dead_code)]

use crate::{EnumWithNumberValue, EnumWithStringValue, EnumWithMaskValueU64};
use crate::{WithNumberValue, WithStringValue, WithMaskValueU64};

#[derive(WithNumberValue, WithStringValue, WithMaskValueU64, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillState {
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
    RollingCutter,
    SoulLinked,
}

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkillType {
    Magic,
    Weapon,
    Misc,
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
    TargetManHole,
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

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum UseSkillFailure {
    #[value = 0]
    Level,
    SpInsufficient,
    HpInsufficient ,
    StuffInsufficient,
    Skillinterval,
    Money,
    ThisWeapon,
    RedGemstone,
    BlueGemmstone,
    Weightover,
    Fail,
    Totarget,
    AncillaNumover,
    Holywater,
    Ancilla,
    DuplicateRangein,
    NeedOtherSkill,
    NeedHelper,
    InvalidDir,
    Summon,
    SummonNone,
    ImitationSkillNone,
    Duplicate,
    Condition,
    Paintbrush,
    Dragon,
    Pos,
    HelperSpInsufficient,
    NeerWall,
    NeedExp1percent,
    ChorusSpInsufficient,
    GcWeaponblocking,
    GcPoisoningweapon,
    Madogear,
    NeedEquipmentKunai,
    TotargetPlayer,
    Size,
    Canonball,
    //xxxIiMadogear_acceleration,
    //xxxIiMadogear_hovering_booster,
    #[value = 40]
    MadogearHovering,
    //xxxIiMadogear_selfdestruction_device,
    //xxxIiMadogear_shapeshifter,
    #[value = 43]
    GuillontinePoison,
    //xxxIiMadogear_cooling_device,
    //xxxIiMadogear_magneticfield_generator,
    //xxxIiMadogear_barrier_generator,
    //xxxIiMadogear_opticalcamouflage_generator,
    //xxxIiMadogear_repairkit,
    //xxxIiMonkey_spanner,
    #[value = 50]
    MadogearRide,
    Spellbook,
    SpellbookDifficultSleep,
    SpellbookPreservationPoint,
    SpellbookReading,
    //xxxIiFace_paints,
    //xxxIiMakeup_brush,
    #[value = 57]
    Cart,
    //xxxIiThorns_seed,
    //xxxIiBlood_sucker_seed,
    #[value = 60]
    NoMoreSpell,
    //xxxIiBomb_mushroom_spore,
    //xxxIiGasoline_boomb,
    //xxxIiOil_bottle,
    //xxxIiExplosion_powder,
    //xxxIiSmoke_powder,
    //xxxIiTear_gas,
    //xxxIiHydrochloric_acid_bottle,
    //xxxIiHells_plant_bottle,
    //xxxIiMandragora_flowerpot,
    #[value = 70]
    ManualNotify,
    //Caution: client uses unidentified display name for displaying the required item. still broken on 2017-05-31 [lemongrass]
    NeedItem,
    NeedEquipment,
    Comboskill,
    Spirits,
    Explosionspirits,
    HpToomany,
    NeedRoyalGuardBanding,
    NeedEquippedWeaponClass,
    ElSummon,
    Relationgrade,
    StyleChangeFighter,
    StyleChangeGrappler,
    ThereAreNpcAround,
    NeedMoreBullet,
    Coins,
    // 86-99Unknown
    #[value = 100]
    ApInsufficient,
    #[value = 255]
    Max,
}

#[derive(WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum UseSkillFailureClientSideType {
    SkillFailed,
    NoEmotion,
    NoSit,
    NoChat,
    NoParty,
    NoShout,
    NoPking,
    NoAlligning
}