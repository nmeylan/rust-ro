#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Str,
    Agi,
    Vit,
    Int,
    Dex,
    Luk,
    AllStats,
    Hit,
    Flee,
    Crit,
    PerfectDodge,
    AspdPercentage,
    Maxhp,
    Maxsp,
    MaxhpPercentage,
    MaxspPercentage,
    Atk,
    Def,
    Mdef,
    ElementWeapon,
    BypassDefenseOnRace,
    WeaponAtkIncreaseOnTargetDefense,
    ReduceDefense,
    ReduceDefensePercentage,
    IncreaseDamageAgainstBossPercentage,
    IncreaseDamageAgainstSizeSmallPercentage,
    IncreaseDamageAgainstSizeMediumPercentage,
    IncreaseDamageAgainstSizeLargePercentage,
    IncreaseDamageRaceFormlessPercentage,
    IncreaseDamageRaceUndeadPercentage,
    IncreaseDamageRaceBrutePercentage,
    IncreaseDamageRacePlantPercentage,
    IncreaseDamageRaceInsectPercentage,
    IncreaseDamageRaceFishPercentage,
    IncreaseDamageRaceDemonPercentage,
    IncreaseDamageRaceDemihumanPercentage,
    IncreaseDamageRaceAngelPercentage,
    IncreaseDamageRaceDragonPercentage,
    IncreaseDamageElementNeutralPercentage,
    IncreaseDamageElementWaterPercentage,
    IncreaseDamageElementEarthPercentage,
    IncreaseDamageElementFirePercentage,
    IncreaseDamageElementWindPercentage,
    IncreaseDamageElementPoisonPercentage,
    IncreaseDamageElementHolyPercentage,
    IncreaseDamageElementDarkPercentage,
    IncreaseDamageElementGhostPercentage,
    IncreaseDamageElementUndeadPercentage,
    DamageIncDecRacePercentage,
    DamageIncDecRaceFormlessPercentage,
    DamageIncDecRaceUndeadPercentage,
    DamageIncDecRaceBrutePercentage,
    DamageIncDecRacePlantPercentage,
    DamageIncDecRaceInsectPercentage,
    DamageIncDecRaceFishPercentage,
    DamageIncDecRaceDemonPercentage,
    DamageIncDecRaceDemihumanPercentage,
    DamageIncDecRaceAngelPercentage,
    DamageIncDecRaceDragonPercentage,
    DamageIncDecElementNeutralPercentage,
    DamageIncDecElementWaterPercentage,
    DamageIncDecElementEarthPercentage,
    DamageIncDecElementFirePercentage,
    DamageIncDecElementWindPercentage,
    DamageIncDecElementPoisonPercentage,
    DamageIncDecElementHolyPercentage,
    DamageIncDecElementDarkPercentage,
    DamageIncDecElementGhostPercentage,
    DamageIncDecElementUndeadPercentage,
    CriticalDamagePercentage,
    CastTimePercentage,
    AcdPercentage,
    HpRegenPercentage,
    SpRegenPercentage,
    ResistanceRangeAttackPercentage,
    NormalAttackPercentage,
    IncreaseDamageGoblinPercentage,
    IncreaseDamageKoboldPercentage,
    IncreaseDamageOrcPercentage,
    IncreaseDamageGolemPercentage,
    LowerDefencePercentage,
    IncreaseHitPercentage,
    AtkPercentage,
    MatkBasedOnStaffPercentage,
    MatkPercentage,
    CriticalAgainstRaceFormlessPercentage,
    CriticalAgainstRaceUndeadPercentage,
    CriticalAgainstRaceBrutePercentage,
    CriticalAgainstRacePlantPercentage,
    CriticalAgainstRaceInsectPercentage,
    CriticalAgainstRaceFishPercentage,
    CriticalAgainstRaceDemonPercentage,
    CriticalAgainstRaceDemihumanPercentage,
    CriticalAgainstRaceAngelPercentage,
    CriticalAgainstRaceDragonPercentage,
    ChanceToInflictStatusOnAttack,
    ChanceToInflictStatusPoisonOnAttack,
    ChanceToInflictStatusStunOnAttack,
    ChanceToInflictStatusFreezeOnAttack,
    ChanceToInflictStatusCurseOnAttack,
    ChanceToInflictStatusBlindOnAttack,
    ChanceToInflictStatusSleepOnAttack,
    ChanceToInflictStatusSilenceOnAttack,
    ChanceToInflictStatusChaosOnAttack,
    ChanceToInflictStatusBleedingOnAttack,
    ChanceToInflictStatusStoneOnAttack,
    ChanceToInflictStatusWeaponBreakOnAttack,
    ChanceToInflictStatusArmorBreakOnAttack,
    ResistanceToStatusPoisonPercentage,
    ResistanceToStatusStunPercentage,
    ResistanceToStatusFreezePercentage,
    ResistanceToStatusCursePercentage,
    ResistanceToStatusBlindPercentage,
    ResistanceToStatusSleepPercentage,
    ResistanceToStatusSilencePercentage,
    ResistanceToStatusChaosPercentage,
    ResistanceToStatusBleedingPercentage,
    ResistanceToStatusStonePercentage,
    ResistanceToStatusWeaponBreakPercentage,
    ResistanceToStatusArmorBreakPercentage,
    DamageIncDecSizePercentage,
    DamageIncDecSizeSmallPercentage,
    DamageIncDecSizeMediumPercentage,
    DamageIncDecSizeLargePercentage
}