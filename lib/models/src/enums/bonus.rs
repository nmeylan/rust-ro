use crate::enums::element::Element;
use crate::status::StatusSnapshot;

#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Str(i8),
    Agi(i8),
    Vit(i8),
    Int(i8),
    Dex(i8),
    Luk(i8),
    AllStats(i8),
    Hit(i8),
    HitPercentage(i8),
    Flee(i8),
    Crit(i8),
    PerfectDodge(i8),
    Aspd(i8),
    AspdPercentage(i8),
    Maxhp(i8),
    Maxsp(i8),
    MaxhpPercentage(i8),
    MaxspPercentage(i8),
    Atk(i8),
    Def(i8),
    VitDefPercentage(i8),
    DefPercentage(i8),
    Mdef(i8),
    Matk(i8),
    MatkBasedOnStaffPercentage(i8),
    MatkPercentage(i8),
    AtkPercentage(i8),
    PerfectHitPercentage(i8),
    ElementWeapon(Element),
    ElementDefense(Element),
    BypassDefenseOnRace,
    WeaponAtkIncreaseOnTargetDefense,
    ReduceDefense(i8),
    ReduceDefensePercentage(i8),
    CriticalDamagePercentage(i8),
    CastTimePercentage(i8),
    CastTimeWhenUsingSkillIdPercentage(u32, i8),
    AfterCastDelayPercentage(i8),
    NaturalHpRecoveryPercentage(i8),
    NaturalSpRecoveryPercentage(i8),
    HpRegenPercentage(i8),
    SpRegenPercentage(i8),
    HpRegenFromItemPercentage(i8),
    HpRegenFromItemIDPercentage(u32, i8),
    HpRegenFromHerbPercentage(i8),
    HpRegenFromFruitPercentage(i8),
    HpRegenFromMeatPercentage(i8),
    HpRegenFromCandyPercentage(i8),
    HpRegenFromJuicePercentage(i8),
    HpRegenFromFishPercentage(i8),
    HpRegenFromFoodPercentage(i8),
    HpRegenFromPotionPercentage(i8),
    GainHpWhenKillingEnemy(i8),
    GainHpWhenKillingEnemyWithMagicAttack(i8),
    GainSpWhenKillingEnemyWithMagicAttack(i8),
    HpRegenFromSkillPercentage(i8),
    DisableHpRegen,
    DisableSpRegen,
    GainSpWhenHittingEnemy(i8),
    GainSpWhenKillingEnemy(i8),
    SpConsumption(i8),
    ResistanceRangeAttackPercentage(i8),
    NormalAttackPercentage(i8),
    PhysicalDamageAgainstSizeSmallPercentage(i8),
    PhysicalDamageAgainstSizeMediumPercentage(i8),
    PhysicalDamageAgainstSizeLargePercentage(i8),
    MagicalDamageAgainstSizeSmallPercentage(i8),
    MagicalDamageAgainstSizeMediumPercentage(i8),
    MagicalDamageAgainstSizeLargePercentage(i8),
    PhysicalDamageAgainstRaceFormlessPercentage(i8),
    PhysicalDamageAgainstRaceUndeadPercentage(i8),
    PhysicalDamageAgainstRaceBrutePercentage(i8),
    PhysicalDamageAgainstRacePlantPercentage(i8),
    PhysicalDamageAgainstRaceInsectPercentage(i8),
    PhysicalDamageAgainstRaceFishPercentage(i8),
    PhysicalDamageAgainstRaceDemonPercentage(i8),
    PhysicalDamageAgainstRaceDemiHumanPercentage(i8),
    PhysicalDamageAgainstRaceAngelPercentage(i8),
    PhysicalDamageAgainstRaceDragonPercentage(i8),
    MagicalDamageAgainstRacePercentage(i8),
    MagicalDamageAgainstRaceFormlessPercentage(i8),
    MagicalDamageAgainstRaceUndeadPercentage(i8),
    MagicalDamageAgainstRaceBrutePercentage(i8),
    MagicalDamageAgainstRacePlantPercentage(i8),
    MagicalDamageAgainstRaceInsectPercentage(i8),
    MagicalDamageAgainstRaceFishPercentage(i8),
    MagicalDamageAgainstRaceDemonPercentage(i8),
    MagicalDamageAgainstRaceDemiHumanPercentage(i8),
    MagicalDamageAgainstRaceAngelPercentage(i8),
    MagicalDamageAgainstRaceDragonPercentage(i8),
    PhysicalDamageAgainstElementNeutralPercentage(i8),
    PhysicalDamageAgainstElementWaterPercentage(i8),
    PhysicalDamageAgainstElementEarthPercentage(i8),
    PhysicalDamageAgainstElementFirePercentage(i8),
    PhysicalDamageAgainstElementWindPercentage(i8),
    PhysicalDamageAgainstElementPoisonPercentage(i8),
    PhysicalDamageAgainstElementHolyPercentage(i8),
    PhysicalDamageAgainstElementDarkPercentage(i8),
    PhysicalDamageAgainstElementGhostPercentage(i8),
    PhysicalDamageAgainstElementUndeadPercentage(i8),
    LowerDefencePercentage(i8),
    CriticalAgainstRaceFormlessPercentage(i8),
    CriticalAgainstRaceUndeadPercentage(i8),
    CriticalAgainstRaceBrutePercentage(i8),
    CriticalAgainstRacePlantPercentage(i8),
    CriticalAgainstRaceInsectPercentage(i8),
    CriticalAgainstRaceFishPercentage(i8),
    CriticalAgainstRaceDemonPercentage(i8),
    CriticalAgainstRaceDemiHumanPercentage(i8),
    CriticalAgainstRaceAngelPercentage(i8),
    CriticalAgainstRaceDragonPercentage(i8),
    ChanceToInflictStatusPoisonOnAttackPercentage(i8),
    ChanceToInflictStatusStunOnAttackPercentage(i8),
    ChanceToInflictStatusFreezeOnAttackPercentage(i8),
    ChanceToInflictStatusCurseOnAttackPercentage(i8),
    ChanceToInflictStatusBlindOnAttackPercentage(i8),
    ChanceToInflictStatusSleepOnAttackPercentage(i8),
    ChanceToInflictStatusSilenceOnAttackPercentage(i8),
    ChanceToInflictStatusBurningOnAttackPercentage(i8),
    ChanceToInflictStatusChaosOnAttackPercentage(i8),
    ChanceToInflictStatusBleedingOnAttackPercentage(i8),
    ChanceToInflictStatusStoneOnAttackPercentage(i8),
    ChanceToInflictStatusConfuseOnAttackPercentage(i8),
    ChanceToInflictStatusWeaponBreakOnAttackPercentage(i8),
    ChanceToInflictStatusArmorBreakOnAttackPercentage(i8),
    ChanceToInflictStatusComaOnAttackPercentage(i8),
    ChanceToInflictStatusComaOnAttackOnBossClassPercentage(i8),
    ChanceToInflictStatusComaOnAttackOnNormalClassPercentage(i8),
    ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceFormlessPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceUndeadPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceBrutePercentage(i8),
    ChanceToInflictStatusComaOnAttackRacePlantPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceInsectPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceFishPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceDemonPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceAngelPercentage(i8),
    ChanceToInflictStatusComaOnAttackRaceDragonPercentage(i8),
    ChanceToInflictStatusPoisonToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusStunToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusFreezeToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusCurseToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusBlindToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusSleepToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusSilenceToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusBurningToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusChaosToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusBleedingToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusStoneToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusConfuseToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusComaToSelfOnAttackPercentage(i8),
    ChanceToInflictStatusPoisonWhenHitPercentage(i8),
    ChanceToInflictStatusStunWhenHitPercentage(i8),
    ChanceToInflictStatusFreezeWhenHitPercentage(i8),
    ChanceToInflictStatusCurseWhenHitPercentage(i8),
    ChanceToInflictStatusBlindWhenHitPercentage(i8),
    ChanceToInflictStatusSleepWhenHitPercentage(i8),
    ChanceToInflictStatusSilenceWhenHitPercentage(i8),
    ChanceToInflictStatusBurningWhenHitPercentage(i8),
    ChanceToInflictStatusChaosWhenHitPercentage(i8),
    ChanceToInflictStatusBleedingWhenHitPercentage(i8),
    ChanceToInflictStatusStoneWhenHitPercentage(i8),
    ChanceToInflictStatusConfuseWhenHitPercentage(i8),
    ChanceToInflictStatusWeaponBreakWhenHitPercentage(i8),
    ChanceToInflictStatusArmorBreakWhenHitPercentage(i8),
    ChanceToInflictStatusComaWhenHitPercentage(i8),
    ResistanceToStatusPoisonPercentage(i8),
    ResistanceToStatusStunPercentage(i8),
    ResistanceToStatusFreezePercentage(i8),
    ResistanceToStatusCursePercentage(i8),
    ResistanceToStatusConfusePercentage(i8),
    ResistanceToStatusBurningPercentage(i8),
    ResistanceToStatusBlindPercentage(i8),
    ResistanceToStatusSleepPercentage(i8),
    ResistanceToStatusSilencePercentage(i8),
    ResistanceToStatusChaosPercentage(i8),
    ResistanceToStatusBleedingPercentage(i8),
    ResistanceToStatusStonePercentage(i8),
    ResistanceToStatusWeaponBreakPercentage(i8),
    ResistanceToStatusArmorBreakPercentage(i8),
    BreakArmorPercentage(i8),
    BreakWeaponPercentage(i8),
    ClassChangePercentageOnHit(i8),
    CriticalChance(i8),
    LongRangeCriticalChance(i8),
    IncreaseDamageAgainstClassBossBaseOnDef,
    IncreaseDamageAgainstClassNormalBaseOnDef,
    IncreaseDamageAgainstClassGuardianBaseOnDef,
    PhysicalDamageAgainstClassBossPercentage(i8),
    PhysicalDamageAgainstClassNormalPercentage(i8),
    PhysicalDamageAgainstClassGuardianPercentage(i8),
    PhysicalDamageAgainstMobIdPercentage(u32, i8),
    ResistanceDamageFromClassBossPercentage(i8),
    ResistanceDamageFromClassNormalPercentage(i8),
    ResistanceDamageFromClassGuardianPercentage(i8),
    ResistanceDamageFromElementNeutralPercentage(i8),
    ResistanceDamageFromElementWaterPercentage(i8),
    ResistanceDamageFromElementEarthPercentage(i8),
    ResistanceDamageFromElementFirePercentage(i8),
    ResistanceDamageFromElementWindPercentage(i8),
    ResistanceDamageFromElementPoisonPercentage(i8),
    ResistanceDamageFromElementHolyPercentage(i8),
    ResistanceDamageFromElementDarkPercentage(i8),
    ResistanceDamageFromElementGhostPercentage(i8),
    ResistanceDamageFromElementUndeadPercentage(i8),
    ResistanceDamageFromRaceFormlessPercentage(i8),
    ResistanceDamageFromRaceUndeadPercentage(i8),
    ResistanceDamageFromRaceBrutePercentage(i8),
    ResistanceDamageFromRacePlantPercentage(i8),
    ResistanceDamageFromRaceInsectPercentage(i8),
    ResistanceDamageFromRaceFishPercentage(i8),
    ResistanceDamageFromRaceDemonPercentage(i8),
    ResistanceDamageFromRaceDemiHumanPercentage(i8),
    ResistanceDamageFromRaceAngelPercentage(i8),
    ResistanceDamageFromRaceDragonPercentage(i8),
    ResistanceDamageFromSizeSmallPercentage(i8),
    ResistanceDamageFromSizeMediumPercentage(i8),
    ResistanceDamageFromSizeLargePercentage(i8),

    SkillDelayIncDecPercentage(i8),
    DoubleAttackChancePercentage(i8),
    HealSkillPercentage(i8),
    HealSkillIdPercentage(u32, i8),
    IgnoreDefClassNormal,
    IgnoreDefClassBoss,
    IgnoreDefClassGuardian,
    IgnoreDefRaceAngel,
    IgnoreDefRaceBrute,
    IgnoreDefRaceDemiHuman,
    IgnoreDefRaceDemon,
    IgnoreDefRaceDragon,
    IgnoreDefRaceFish,
    IgnoreDefRaceFormless,
    IgnoreDefRaceInsect,
    IgnoreDefRacePlant,
    IgnoreDefRacePlayerHuman,
    IgnoreDefRacePlayerDoram,
    IgnoreDefRaceUndead,
    IgnoreDefRaceFormlessPercentage(i8),
    IgnoreDefRaceUndeadPercentage(i8),
    IgnoreDefRaceBrutePercentage(i8),
    IgnoreDefRacePlantPercentage(i8),
    IgnoreDefRaceInsectPercentage(i8),
    IgnoreDefRaceFishPercentage(i8),
    IgnoreDefRaceDemonPercentage(i8),
    IgnoreDefRaceDemiHumanPercentage(i8),
    IgnoreDefRaceAngelPercentage(i8),
    IgnoreDefRaceDragonPercentage(i8),
    IgnoreMDefRaceFormlessPercentage(i8),
    IgnoreMDefRaceUndeadPercentage(i8),
    IgnoreMDefRaceBrutePercentage(i8),
    IgnoreMDefRacePlantPercentage(i8),
    IgnoreMDefRaceInsectPercentage(i8),
    IgnoreMDefRaceFishPercentage(i8),
    IgnoreMDefRaceDemonPercentage(i8),
    IgnoreMDefRaceDemiHumanPercentage(i8),
    IgnoreMDefRaceAngelPercentage(i8),
    IgnoreMDefRaceDragonPercentage(i8),
    IgnoreMDefClassNormalPercentage(i8),
    IgnoreMDefClassBossPercentage(i8),
    IgnoreMDefClassGuardianPercentage(i8),

    ResistanceRangeAttack(i8),
    DamageRangedAtkPercentage(i8),
    ResistanceMagicAttackPercentage(i8),
    MagicAttackReflectChancePercentage(i8),
    MeleeAttackReflectChancePercentage(i8),
    SplashRadius(i8),
    SpeedPercentage(i8),
    EnableFullHpSpRecoverOnResurrect,
    EnableSeeHidden,
    EnableNoCancelCast,
    EnableNoGemstoneRequired,
    EnableIgnoreSizeModifier,
    EnableNoKnockback,
    EnableNoWalkDelay,
    UnbreakableArmor,
    UnbreakableShoulder,
    UnbreakableHelm,
    UnbreakableShield,
    UnbreakableShoes,
    UnbreakableWeapon,
    ResistancePhysicalAttackFromMobIdPercentage(u32, i8),
    DropChanceItemIdPercentage(u32, i8),
    DropChanceJewelPercentage(i8),
    DropChanceOrePercentage(i8),
    DropChanceRecoveryPercentage(i8),
    DropChanceFoodPercentage(i8),
    KnockbackWhenUsingSkillId(u32, i8),
    GainExpWhenKillingRaceFormlessPercentage(i8),
    GainExpWhenKillingRaceUndeadPercentage(i8),
    GainExpWhenKillingRaceBrutePercentage(i8),
    GainExpWhenKillingRacePlantPercentage(i8),
    GainExpWhenKillingRaceInsectPercentage(i8),
    GainExpWhenKillingRaceFishPercentage(i8),
    GainExpWhenKillingRaceDemonPercentage(i8),
    GainExpWhenKillingRaceDemiHumanPercentage(i8),
    GainExpWhenKillingRaceAngelPercentage(i8),
    GainExpWhenKillingRaceDragonPercentage(i8),
    GainZenyWhenKillingMonster(u16, i8),
    // zeny, chance
    HpDrainWhenAttackingPercentage(i8, i8),
    // hp percentage, chance
    SpDrainWhenAttackingPercentage(i8, i8),
    // sp percentage, chance
    SpDrainWhenAttackingRaceFormless(u16),
    SpDrainWhenAttackingRaceUndead(u16),
    SpDrainWhenAttackingRaceBrute(u16),
    SpDrainWhenAttackingRacePlant(u16),
    SpDrainWhenAttackingRaceInsect(u16),
    SpDrainWhenAttackingRaceFish(u16),
    SpDrainWhenAttackingRaceDemon(u16),
    SpDrainWhenAttackingRaceDemiHuman(u16),
    SpDrainWhenAttackingRaceAngel(u16),
    SpDrainWhenAttackingRaceDragon(u16),
    SpDrainWhenKillingRaceFormless(u16),
    SpDrainWhenKillingRaceUndead(u16),
    SpDrainWhenKillingRaceBrute(u16),
    SpDrainWhenKillingRacePlant(u16),
    SpDrainWhenKillingRaceInsect(u16),
    SpDrainWhenKillingRaceFish(u16),
    SpDrainWhenKillingRaceDemon(u16),
    SpDrainWhenKillingRaceDemiHuman(u16),
    SpDrainWhenKillingRaceAngel(u16),
    SpDrainWhenKillingRaceDragon(u16),
    SpBurnOnTargetWhenAttackingPercentage(i8, u16),
    // percentage, chance
    HpLossEveryMs(u16, u16),
    //amount, every ms
    HpRegenEveryMs(u16, u16),
    //amount, every ms
    SpLossEveryMs(u16, u16),
    //amount, every ms
    SpRegenEveryMs(u16, u16),
    //amount, every ms
    SkillIdDamagePercentage(u32, i8),
}

impl BonusType {
    pub fn add_bonus_to_status(&self, status_snapshot: &mut StatusSnapshot) {
        match self {
            BonusType::Str(str) => { status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *str as u16) }
            BonusType::Agi(agi) => { status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *agi as u16) }
            BonusType::Vit(vit) => { status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *vit as u16) }
            BonusType::Int(int) => { status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *int as u16) }
            BonusType::Dex(dex) => { status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *dex as u16) }
            BonusType::Luk(luk) => { status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *luk as u16) }
            BonusType::AllStats(all) => {
                status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *all as u16);
                status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *all as u16);
                status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *all as u16);
                status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *all as u16);
                status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *all as u16);
                status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *all as u16);
            }
            BonusType::Hit(hit) => { status_snapshot.set_hit(status_snapshot.hit() + *hit as u16) }
            BonusType::HitPercentage(_) => {}
            BonusType::Flee(flee) => { status_snapshot.set_flee(status_snapshot.flee() + *flee as u16) }
            BonusType::Crit(crit) => { status_snapshot.set_crit(status_snapshot.crit() + *crit as f32) }
            BonusType::PerfectDodge(_) => {}
            BonusType::Aspd(aspd) => { status_snapshot.set_aspd(status_snapshot.aspd() + *aspd as f32) }
            BonusType::AspdPercentage(_) => {}
            BonusType::Maxhp(hp) => { status_snapshot.set_hp(status_snapshot.hp() + *hp as u32) }
            BonusType::Maxsp(sp) => { status_snapshot.set_sp(status_snapshot.sp() + *sp as u32) }
            BonusType::MaxhpPercentage(_) => {}
            BonusType::MaxspPercentage(_) => {}
            BonusType::Atk(_) => {}
            BonusType::Def(_) => {}
            BonusType::VitDefPercentage(_) => {}
            BonusType::DefPercentage(_) => {}
            BonusType::Mdef(mdef) => { status_snapshot.set_mdef(status_snapshot.mdef() + *mdef as u16) }
            BonusType::Matk(matk) => {
                status_snapshot.set_matk_min(status_snapshot.matk_min() + *matk as u16);
                status_snapshot.set_matk_max(status_snapshot.matk_max() + *matk as u16);
            }
            BonusType::MatkBasedOnStaffPercentage(_) => {}
            BonusType::MatkPercentage(matk_percentage) => {status_snapshot.set_matk_item_modifier(status_snapshot.matk_item_modifier() + (*matk_percentage as f32/100.0))}
            BonusType::AtkPercentage(_) => {}
            BonusType::PerfectHitPercentage(_) => {}
            BonusType::ElementWeapon(_) => {}
            BonusType::ElementDefense(_) => {}
            BonusType::BypassDefenseOnRace => {}
            BonusType::WeaponAtkIncreaseOnTargetDefense => {}
            BonusType::ReduceDefense(_) => {}
            BonusType::ReduceDefensePercentage(_) => {}
            BonusType::CriticalDamagePercentage(_) => {}
            BonusType::CastTimePercentage(_) => {}
            BonusType::CastTimeWhenUsingSkillIdPercentage(_, _) => {}
            BonusType::AfterCastDelayPercentage(_) => {}
            BonusType::NaturalHpRecoveryPercentage(_) => {}
            BonusType::NaturalSpRecoveryPercentage(_) => {}
            BonusType::HpRegenPercentage(_) => {}
            BonusType::SpRegenPercentage(_) => {}
            BonusType::HpRegenFromItemPercentage(_) => {}
            BonusType::HpRegenFromItemIDPercentage(_, _) => {}
            BonusType::HpRegenFromHerbPercentage(_) => {}
            BonusType::HpRegenFromFruitPercentage(_) => {}
            BonusType::HpRegenFromMeatPercentage(_) => {}
            BonusType::HpRegenFromCandyPercentage(_) => {}
            BonusType::HpRegenFromJuicePercentage(_) => {}
            BonusType::HpRegenFromFishPercentage(_) => {}
            BonusType::HpRegenFromFoodPercentage(_) => {}
            BonusType::HpRegenFromPotionPercentage(_) => {}
            BonusType::GainHpWhenKillingEnemy(_) => {}
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(_) => {}
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(_) => {}
            BonusType::HpRegenFromSkillPercentage(_) => {}
            BonusType::DisableHpRegen => {}
            BonusType::DisableSpRegen => {}
            BonusType::GainSpWhenHittingEnemy(_) => {}
            BonusType::GainSpWhenKillingEnemy(_) => {}
            BonusType::SpConsumption(_) => {}
            BonusType::ResistanceRangeAttackPercentage(_) => {}
            BonusType::NormalAttackPercentage(_) => {}
            BonusType::PhysicalDamageAgainstSizeSmallPercentage(_) => {}
            BonusType::PhysicalDamageAgainstSizeMediumPercentage(_) => {}
            BonusType::PhysicalDamageAgainstSizeLargePercentage(_) => {}
            BonusType::MagicalDamageAgainstSizeSmallPercentage(_) => {}
            BonusType::MagicalDamageAgainstSizeMediumPercentage(_) => {}
            BonusType::MagicalDamageAgainstSizeLargePercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceFormlessPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceUndeadPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceBrutePercentage(_) => {}
            BonusType::PhysicalDamageAgainstRacePlantPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceInsectPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceFishPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceDemonPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceDemiHumanPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceAngelPercentage(_) => {}
            BonusType::PhysicalDamageAgainstRaceDragonPercentage(_) => {}
            BonusType::MagicalDamageAgainstRacePercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceFormlessPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceUndeadPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceBrutePercentage(_) => {}
            BonusType::MagicalDamageAgainstRacePlantPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceInsectPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceFishPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceDemonPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceDemiHumanPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceAngelPercentage(_) => {}
            BonusType::MagicalDamageAgainstRaceDragonPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementNeutralPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementWaterPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementEarthPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementFirePercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementWindPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementPoisonPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementHolyPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementDarkPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementGhostPercentage(_) => {}
            BonusType::PhysicalDamageAgainstElementUndeadPercentage(_) => {}
            BonusType::LowerDefencePercentage(_) => {}
            BonusType::CriticalAgainstRaceFormlessPercentage(_) => {}
            BonusType::CriticalAgainstRaceUndeadPercentage(_) => {}
            BonusType::CriticalAgainstRaceBrutePercentage(_) => {}
            BonusType::CriticalAgainstRacePlantPercentage(_) => {}
            BonusType::CriticalAgainstRaceInsectPercentage(_) => {}
            BonusType::CriticalAgainstRaceFishPercentage(_) => {}
            BonusType::CriticalAgainstRaceDemonPercentage(_) => {}
            BonusType::CriticalAgainstRaceDemiHumanPercentage(_) => {}
            BonusType::CriticalAgainstRaceAngelPercentage(_) => {}
            BonusType::CriticalAgainstRaceDragonPercentage(_) => {}
            BonusType::ChanceToInflictStatusPoisonOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusStunOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusFreezeOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusCurseOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBlindOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusSleepOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusSilenceOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBurningOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusChaosOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBleedingOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusStoneOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusConfuseOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusWeaponBreakOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusArmorBreakOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackOnBossClassPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackOnNormalClassPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceFormlessPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceUndeadPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceBrutePercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRacePlantPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceInsectPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceFishPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceDemonPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceAngelPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaOnAttackRaceDragonPercentage(_) => {}
            BonusType::ChanceToInflictStatusPoisonToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusStunToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusFreezeToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusCurseToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBlindToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusSleepToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusSilenceToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBurningToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusChaosToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusBleedingToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusStoneToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusConfuseToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaToSelfOnAttackPercentage(_) => {}
            BonusType::ChanceToInflictStatusPoisonWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusStunWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusFreezeWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusCurseWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusBlindWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusSleepWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusSilenceWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusBurningWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusChaosWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusBleedingWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusStoneWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusConfuseWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusWeaponBreakWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusArmorBreakWhenHitPercentage(_) => {}
            BonusType::ChanceToInflictStatusComaWhenHitPercentage(_) => {}
            BonusType::ResistanceToStatusPoisonPercentage(_) => {}
            BonusType::ResistanceToStatusStunPercentage(_) => {}
            BonusType::ResistanceToStatusFreezePercentage(_) => {}
            BonusType::ResistanceToStatusCursePercentage(_) => {}
            BonusType::ResistanceToStatusConfusePercentage(_) => {}
            BonusType::ResistanceToStatusBurningPercentage(_) => {}
            BonusType::ResistanceToStatusBlindPercentage(_) => {}
            BonusType::ResistanceToStatusSleepPercentage(_) => {}
            BonusType::ResistanceToStatusSilencePercentage(_) => {}
            BonusType::ResistanceToStatusChaosPercentage(_) => {}
            BonusType::ResistanceToStatusBleedingPercentage(_) => {}
            BonusType::ResistanceToStatusStonePercentage(_) => {}
            BonusType::ResistanceToStatusWeaponBreakPercentage(_) => {}
            BonusType::ResistanceToStatusArmorBreakPercentage(_) => {}
            BonusType::BreakArmorPercentage(_) => {}
            BonusType::BreakWeaponPercentage(_) => {}
            BonusType::ClassChangePercentageOnHit(_) => {}
            BonusType::CriticalChance(_) => {}
            BonusType::LongRangeCriticalChance(_) => {}
            BonusType::IncreaseDamageAgainstClassBossBaseOnDef => {}
            BonusType::IncreaseDamageAgainstClassNormalBaseOnDef => {}
            BonusType::IncreaseDamageAgainstClassGuardianBaseOnDef => {}
            BonusType::PhysicalDamageAgainstClassBossPercentage(_) => {}
            BonusType::PhysicalDamageAgainstClassNormalPercentage(_) => {}
            BonusType::PhysicalDamageAgainstClassGuardianPercentage(_) => {}
            BonusType::PhysicalDamageAgainstMobIdPercentage(_, _) => {}
            BonusType::ResistanceDamageFromClassBossPercentage(_) => {}
            BonusType::ResistanceDamageFromClassNormalPercentage(_) => {}
            BonusType::ResistanceDamageFromClassGuardianPercentage(_) => {}
            BonusType::ResistanceDamageFromElementNeutralPercentage(_) => {}
            BonusType::ResistanceDamageFromElementWaterPercentage(_) => {}
            BonusType::ResistanceDamageFromElementEarthPercentage(_) => {}
            BonusType::ResistanceDamageFromElementFirePercentage(_) => {}
            BonusType::ResistanceDamageFromElementWindPercentage(_) => {}
            BonusType::ResistanceDamageFromElementPoisonPercentage(_) => {}
            BonusType::ResistanceDamageFromElementHolyPercentage(_) => {}
            BonusType::ResistanceDamageFromElementDarkPercentage(_) => {}
            BonusType::ResistanceDamageFromElementGhostPercentage(_) => {}
            BonusType::ResistanceDamageFromElementUndeadPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceFormlessPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceUndeadPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceBrutePercentage(_) => {}
            BonusType::ResistanceDamageFromRacePlantPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceInsectPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceFishPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceDemonPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceDemiHumanPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceAngelPercentage(_) => {}
            BonusType::ResistanceDamageFromRaceDragonPercentage(_) => {}
            BonusType::ResistanceDamageFromSizeSmallPercentage(_) => {}
            BonusType::ResistanceDamageFromSizeMediumPercentage(_) => {}
            BonusType::ResistanceDamageFromSizeLargePercentage(_) => {}
            BonusType::SkillDelayIncDecPercentage(_) => {}
            BonusType::DoubleAttackChancePercentage(_) => {}
            BonusType::HealSkillPercentage(_) => {}
            BonusType::HealSkillIdPercentage(_, _) => {}
            BonusType::IgnoreDefClassNormal => {}
            BonusType::IgnoreDefClassBoss => {}
            BonusType::IgnoreDefClassGuardian => {}
            BonusType::IgnoreDefRaceAngel => {}
            BonusType::IgnoreDefRaceBrute => {}
            BonusType::IgnoreDefRaceDemiHuman => {}
            BonusType::IgnoreDefRaceDemon => {}
            BonusType::IgnoreDefRaceDragon => {}
            BonusType::IgnoreDefRaceFish => {}
            BonusType::IgnoreDefRaceFormless => {}
            BonusType::IgnoreDefRaceInsect => {}
            BonusType::IgnoreDefRacePlant => {}
            BonusType::IgnoreDefRacePlayerHuman => {}
            BonusType::IgnoreDefRacePlayerDoram => {}
            BonusType::IgnoreDefRaceUndead => {}
            BonusType::IgnoreDefRaceFormlessPercentage(_) => {}
            BonusType::IgnoreDefRaceUndeadPercentage(_) => {}
            BonusType::IgnoreDefRaceBrutePercentage(_) => {}
            BonusType::IgnoreDefRacePlantPercentage(_) => {}
            BonusType::IgnoreDefRaceInsectPercentage(_) => {}
            BonusType::IgnoreDefRaceFishPercentage(_) => {}
            BonusType::IgnoreDefRaceDemonPercentage(_) => {}
            BonusType::IgnoreDefRaceDemiHumanPercentage(_) => {}
            BonusType::IgnoreDefRaceAngelPercentage(_) => {}
            BonusType::IgnoreDefRaceDragonPercentage(_) => {}
            BonusType::IgnoreMDefRaceFormlessPercentage(_) => {}
            BonusType::IgnoreMDefRaceUndeadPercentage(_) => {}
            BonusType::IgnoreMDefRaceBrutePercentage(_) => {}
            BonusType::IgnoreMDefRacePlantPercentage(_) => {}
            BonusType::IgnoreMDefRaceInsectPercentage(_) => {}
            BonusType::IgnoreMDefRaceFishPercentage(_) => {}
            BonusType::IgnoreMDefRaceDemonPercentage(_) => {}
            BonusType::IgnoreMDefRaceDemiHumanPercentage(_) => {}
            BonusType::IgnoreMDefRaceAngelPercentage(_) => {}
            BonusType::IgnoreMDefRaceDragonPercentage(_) => {}
            BonusType::IgnoreMDefClassNormalPercentage(_) => {}
            BonusType::IgnoreMDefClassBossPercentage(_) => {}
            BonusType::IgnoreMDefClassGuardianPercentage(_) => {}
            BonusType::ResistanceRangeAttack(_) => {}
            BonusType::DamageRangedAtkPercentage(_) => {}
            BonusType::ResistanceMagicAttackPercentage(_) => {}
            BonusType::MagicAttackReflectChancePercentage(_) => {}
            BonusType::MeleeAttackReflectChancePercentage(_) => {}
            BonusType::SplashRadius(_) => {}
            BonusType::SpeedPercentage(_) => {}
            BonusType::EnableFullHpSpRecoverOnResurrect => {}
            BonusType::EnableSeeHidden => {}
            BonusType::EnableNoCancelCast => {}
            BonusType::EnableNoGemstoneRequired => {}
            BonusType::EnableIgnoreSizeModifier => {}
            BonusType::EnableNoKnockback => {}
            BonusType::EnableNoWalkDelay => {}
            BonusType::UnbreakableArmor => {}
            BonusType::UnbreakableShoulder => {}
            BonusType::UnbreakableHelm => {}
            BonusType::UnbreakableShield => {}
            BonusType::UnbreakableShoes => {}
            BonusType::UnbreakableWeapon => {}
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(_, _) => {}
            BonusType::DropChanceItemIdPercentage(_, _) => {}
            BonusType::DropChanceJewelPercentage(_) => {}
            BonusType::DropChanceOrePercentage(_) => {}
            BonusType::DropChanceRecoveryPercentage(_) => {}
            BonusType::DropChanceFoodPercentage(_) => {}
            BonusType::KnockbackWhenUsingSkillId(_, _) => {}
            BonusType::GainExpWhenKillingRaceFormlessPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceUndeadPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceBrutePercentage(_) => {}
            BonusType::GainExpWhenKillingRacePlantPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceInsectPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceFishPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceDemonPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceDemiHumanPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceAngelPercentage(_) => {}
            BonusType::GainExpWhenKillingRaceDragonPercentage(_) => {}
            BonusType::GainZenyWhenKillingMonster(_, _) => {}
            BonusType::HpDrainWhenAttackingPercentage(_, _) => {}
            BonusType::SpDrainWhenAttackingPercentage(_, _) => {}
            BonusType::SpDrainWhenAttackingRaceFormless(_) => {}
            BonusType::SpDrainWhenAttackingRaceUndead(_) => {}
            BonusType::SpDrainWhenAttackingRaceBrute(_) => {}
            BonusType::SpDrainWhenAttackingRacePlant(_) => {}
            BonusType::SpDrainWhenAttackingRaceInsect(_) => {}
            BonusType::SpDrainWhenAttackingRaceFish(_) => {}
            BonusType::SpDrainWhenAttackingRaceDemon(_) => {}
            BonusType::SpDrainWhenAttackingRaceDemiHuman(_) => {}
            BonusType::SpDrainWhenAttackingRaceAngel(_) => {}
            BonusType::SpDrainWhenAttackingRaceDragon(_) => {}
            BonusType::SpDrainWhenKillingRaceFormless(_) => {}
            BonusType::SpDrainWhenKillingRaceUndead(_) => {}
            BonusType::SpDrainWhenKillingRaceBrute(_) => {}
            BonusType::SpDrainWhenKillingRacePlant(_) => {}
            BonusType::SpDrainWhenKillingRaceInsect(_) => {}
            BonusType::SpDrainWhenKillingRaceFish(_) => {}
            BonusType::SpDrainWhenKillingRaceDemon(_) => {}
            BonusType::SpDrainWhenKillingRaceDemiHuman(_) => {}
            BonusType::SpDrainWhenKillingRaceAngel(_) => {}
            BonusType::SpDrainWhenKillingRaceDragon(_) => {}
            BonusType::SpBurnOnTargetWhenAttackingPercentage(_, _) => {}
            BonusType::HpLossEveryMs(_, _) => {}
            BonusType::HpRegenEveryMs(_, _) => {}
            BonusType::SpLossEveryMs(_, _) => {}
            BonusType::SpRegenEveryMs(_, _) => {}
            BonusType::SkillIdDamagePercentage(_, _) => {}
        }
    }
}