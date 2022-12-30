#![allow(dead_code)]

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillRequirement {
    #[value = 0]
    None,
    Hidden,
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
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillType {
    Magic,
    Weapon,
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillTargetType {
    #[value = 0]
    PassiveSkill,
    AttackSkill,
    GroundSkill,
    SelfSkill,
    #[value = 10]
    SupportSkill,
    #[value = 11]
    TrapSkill,
}


#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillDamageFlags {
    #[value = 0]
    Nodamage = 0,
    #[value = 1]
    Splash,
    #[value = 2]
    Splashsplit,
    #[value = 4]
    Ignoreatkcard,
    #[value = 8]
    Ignoreelement,
    #[value = 16]
    Ignoredefense,
    #[value = 32]
    Ignoreflee,
    #[value = 64]
    Ignoredefcard,
    #[value = 128]
    Critical,
    #[value = 512]
    Ignorelongcard,
    #[value = 1024]
    Max,
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillFlags {
    #[value = 1]
    //NPC skills are those that players can't have in their skill tree.
    Isnpc,
    #[value = 2]
    Iswedding,
    #[value = 4]
    Isspirit,
    #[value = 8]
    Isguild,
    #[value = 16]
    Issong,
    #[value = 32]
    Isensemble,
    #[value = 64]
    Istrap,
    #[value = 128]
    //Refers to ground placed skills that will target the caster as well (like Grandcross)
    Targetself,
    #[value = 256]
    Notargetself,
    #[value = 512]
    Partyonly,
    #[value = 1024]
    Guildonly,
    #[value = 2048]
    Notargetenemy,
    #[value = 4096]
    // Skill that available for SCAUTOSHADOWSPELL
    Isautoshadowspell,
    #[value = 8192]
    // Chorus skill
    Ischorus,
    #[value = 16384]
    // Skill that ignore bg reduction
    Ignorebgreduction,
    #[value = 32768]
    // Skill that ignore gvg reduction
    Ignoregvgreduction,
    #[value = 65536]
    // disable to cast skill if near with NPC
    Disablenearnpc,
    #[value = 131072]
    // can hit trap-type skill (ISTRAP)
    Targettrap,
    #[value = 262144]
    // Skill that can ignore Land Protector
    Ignorelandprotector,
    #[value = 524288]
    // Skill that can be use in hiding
    Allowwhenhidden,
    #[value = 1048576]
    // Skill that can be use while in dancing state
    Allowwhenperforming,
    #[value = 2097152]
    // Skill that could hit emperium
    Targetemperium,
    #[value = 4194304]
    // Skill blocked by kagehumi
    Ignorekagehumi,
    #[value = 8388608]
    // Skill shows AoE area while casting
    Alterrangevulture,
    #[value = 16777216]
    Max,
    #[value = 33554432]
    Isquest,
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillDamageType {
    // damage [ damage: total damage, div: amount of hits, damage2: assassin dual-wield damage ]
    Normal,
    // pick up item
    PickupItem,
    SitDown,
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

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillCopyFlags {
    #[value = 1]
    Plagiarism,
    #[value = 2]
    Reproduce,
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillCastTimeDelayFlags {
    #[value = 1]
    IgnoreDex,
    #[value = 2]
    IgnoreStatus,
    #[value = 4]
    IgnoreItemBonus,
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum SkillUnitFlag {
    #[value = 0]
    None,
    #[value = 1]
    // If 'defunit_not_enemy' is set, the target is changed to 'friend'
    NoEnemy,
    #[value = 2]
    // Spell cannot be stacked
    NorEiteration,
    #[value = 4]
    // Spell cannot be cast near/on targets
    NoFootset,
    #[value = 8]
    // Spell effects do not overlap
    NoOverlap,
    #[value = 16]
    // Only cells with a shootable path will be placed
    PathCheck,
    #[value = 32]
    // May not target players
    NotPlayer,
    #[value = 64]
    // May not target mobs
    NotMob,
    #[value = 128]
    // May target skills
    Skill,
    #[value = 256]
    // Dance
    Dance,
    #[value = 512]
    // Duet
    Ensemble,
    #[value = 1024]
    // Song
    Song,
    #[value = 2048]
    // Spells should trigger both ontimer and onplace/onout/onleft effects.
    Dualmode,
    #[value = 4096]
    // Skill unit cannot be knocked back
    NoKnockback,
    #[value = 8192]
    // hack for ranged layout, only display center
    RangedSingleunit,
    #[value = 16384]
    // Immune to Crazy Weed removal
    CrazyWeedImmune,
    #[value = 32768]
    // removed by Fire Rain
    RemovedByFireRain,
    #[value = 65536]
    // knockback skill unit with its group instead of single unit
    KnockbackGroup,
    #[value = 131072]
    // Hidden trap
    HiddenTrap,
    #[value = 262144]
    Max,
}