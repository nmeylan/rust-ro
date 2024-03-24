use enum_macro::WithEq;
use crate::enums::element::Element;
use crate::enums::mob::{MobClass, MobGroup, MobRace};
use crate::enums::size::Size;
use crate::enums::status::StatusEffect;
use crate::enums::weapon::WeaponType;
use crate::status::{StatusBonus, StatusSnapshot};

// When adding an new bonus type, should update:
// 1. partialEq implementation
// 2. get_bonus_sum
// 3. get_bonus_value
#[derive(Debug, Clone, Copy)]
#[derive(WithEq)]
pub enum BonusType {
    Str(i8),
    Agi(i8),
    Vit(i8),
    Int(i8),
    Dex(i8),
    Luk(i8),
    AllStats(i8),
    Hit(i16),
    HitPercentage(i8),
    AccuracyPercentage(i8),
    Flee(i16),
    Crit(i8),
    PerfectDodge(i8),
    Aspd(i8),
    AspdPercentage(i8),
    Maxhp(i32),
    Maxsp(i32),
    MaxhpPercentage(i8),
    MaxspPercentage(i8),
    Atk(i16),
    Def(i16),
    VitDefPercentage(i8),
    DefPercentage(i8),
    Mdef(i16),
    Matk(i16),
    MatkBasedOnStaffPercentage(i8),
    MatkPercentage(i8),
    AtkPercentage(i8),
    PerfectHitPercentage(i8),
    ElementWeapon(Element),
    ElementDefense(Element),
    CriticalDamagePercentage(i8),
    CastTimePercentage(i8),
    CastTimeWhenUsingSkillIdPercentage(u32, i8),
    AfterCastDelayPercentage(i8),
    NaturalHpRecoveryPercentage(i8),
    NaturalSpRecoveryPercentage(i8),
    HpRecoveryMaxSpPercentage(i8),
    SpRecoveryMaxSpPercentage(i8),
    HpRegenFromItemPercentage(i8),
    SpRegenFromItemPercentage(i8),
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
    NormalAttackPercentage(i8),
    NullifyAttackChancePercentage(u8),

    PhysicalDamageAgainstSizePercentage(Size, i8),
    MagicalDamageAgainstSizePercentage(Size, i8),
    PhysicalDamageAgainstRacePercentage(MobRace, i8),
    MagicalDamageAgainstRacePercentage(MobRace, i8),
    PhysicalDamageAgainstElementPercentage(Element, i8),
    DamageAgainstMobGroupPercentage(MobGroup, i8),
    CriticalAgainstRacePercentage(MobRace, i8),
    ChanceToInflictStatusComaOnAttackOnClassPercentage(MobClass, f32),
    ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace, f32),
    PhysicalDamageAgainstClassPercentage(MobClass, i8),
    ResistanceDamageFromClassPercentage(MobClass, i8),
    ResistanceDamageFromElementPercentage(Element, i8),
    ResistanceDamageFromRacePercentage(MobRace, i8),
    ResistanceDamageFromSizePercentage(Size, i8),
    IgnoreDefClass(MobClass),
    IgnoreDefRace(MobRace),
    IgnoreDefRacePercentage(MobRace, i8),
    IgnoreMDefRacePercentage(MobRace, i8),
    IgnoreMDefClassPercentage(MobClass, i8),
    PhysicalDamageAgainstMobIdPercentage(u32, i8),
    DamageUsingElementPercentage(Element, i8),
    DamageUsingWeaponType(WeaponType, i8),

    // Only when attacking with ranged weapon
    IncreaseDamageAgainstClassBaseOnDef(MobClass),

    ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect, f32),
    ChanceToInflictStatusToPartyOnAttackPercentage(StatusEffect, f32),
    ChanceToInflictStatusWhenHitPercentage(StatusEffect, f32),
    ChanceToInflictStatusOnAttackPercentage(StatusEffect, f32),
    ResistanceToStatusPercentage(StatusEffect, f32),

    GainExpWhenKillingRacePercentage(MobRace, i8),
    SpDrainWhenAttackingRace(MobRace, u16),
    SpDrainWhenKillingRace(MobRace, u16),

    BreakArmorPercentage(i8),
    BreakWeaponPercentage(i8),
    BreakSelfWeaponPercentage(i8),
    ClassChangePercentageOnHit(i8),
    LongRangeCriticalChance(i8),

    SkillDelayIncDecPercentage(i8),
    DoubleAttackChancePercentage(i8),
    HealSkillPercentage(i8),
    HealSkillIdPercentage(u32, i8),
    ResistanceRangeAttackPercentage(i8),
    DamageRangedAtkPercentage(i8),
    ResistanceMagicAttackPercentage(i8),
    MagicAttackReflectChancePercentage(i8),
    PhysicalAttackReflectChancePercentage(i8),
    PhysicalAttackBlockChancePercentage(i8),
    SplashRadius(i8),
    SpeedPercentage(i8),
    VisionDistance(i8),
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
    // zeny, chance
    GainZenyWhenKillingMonster(u16, i8),
    // hp percentage, chance
    HpDrainWhenAttackingPercentage(i8, i8),
    // sp percentage, chance
    SpDrainWhenAttackingPercentage(i8, i8),
    SpDrainPerHit(i8),
    // percentage, chance
    SpBurnOnTargetWhenAttackingPercentage(i8, u16),
    //amount, every ms
    HpLossEveryMs(u16, u16),
    //amount, every ms
    HpRegenEveryMs(u16, u16),
    //amount, every ms
    SpLossEveryMs(u16, u16),
    //amount, every ms
    SpRegenEveryMs(u16, u16),
    SkillIdDamagePercentage(u32, i8),
    EnableSkillId(u32, u8),
    // Difference between SkillIdSuccessPercentage and AutospellSkillIdChancePercentage:
    // SkillIdSuccessPercentage apply for offensive(or Self) skill, determining the success of skill itself
    // AutospellSkillIdChancePercentage apply for passive skills which can trigger skill
    SkillIdSuccessPercentage(u32, i8),
    AutospellSkillIdChancePercentage(u32, i8),
    DoubleCastSkillIdChancePercentage(u32, i8),
}

impl BonusType {
    pub fn add_bonus_to_status(&self, status_snapshot: &mut StatusSnapshot) {
        match self {
            BonusType::Str(str) => { status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *str as i16) }
            BonusType::Agi(agi) => { status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *agi as i16) }
            BonusType::Vit(vit) => { status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *vit as i16) }
            BonusType::Int(int) => { status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *int as i16) }
            BonusType::Dex(dex) => { status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *dex as i16) }
            BonusType::Luk(luk) => { status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *luk as i16) }
            BonusType::AllStats(all) => {
                status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *all as i16);
                status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *all as i16);
                status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *all as i16);
                status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *all as i16);
                status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *all as i16);
                status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *all as i16);
            }
            BonusType::Hit(hit) => { status_snapshot.set_hit(status_snapshot.hit() + *hit as i16) }
            BonusType::Flee(flee) => { status_snapshot.set_flee(status_snapshot.flee() + *flee as i16) }
            BonusType::Crit(crit) => { status_snapshot.set_crit(status_snapshot.crit() + *crit as f32) }
            BonusType::Aspd(aspd) => { status_snapshot.set_aspd(status_snapshot.aspd() + *aspd as f32) }
            BonusType::Maxhp(hp) => { status_snapshot.set_hp(status_snapshot.hp() + *hp as u32) }
            BonusType::Maxsp(sp) => { status_snapshot.set_sp(status_snapshot.sp() + *sp as u32) }
            BonusType::MatkPercentage(matk_percentage) => { status_snapshot.set_matk_item_modifier(status_snapshot.matk_item_modifier() + (*matk_percentage as f32 / 100.0)) }
            BonusType::Atk(atk) => { status_snapshot.set_bonus_atk(status_snapshot.bonus_atk() + *atk as u16) }
            BonusType::Def(def) => { status_snapshot.set_def(status_snapshot.def() + *def as i16) }
            BonusType::Mdef(mdef) => { status_snapshot.set_mdef(status_snapshot.mdef() + *mdef as i16) }
            BonusType::Matk(matk) => {
                status_snapshot.set_matk_min(status_snapshot.matk_min() + *matk as u16);
                status_snapshot.set_matk_max(status_snapshot.matk_max() + *matk as u16);
            }
            BonusType::ElementDefense(element) => { status_snapshot.set_element(element.clone()) }
            _ => {}
        }
    }

    pub fn add_percentage_bonus_to_status(&self, status_snapshot: &mut StatusSnapshot) {
        match self {
            BonusType::HitPercentage(value) => { status_snapshot.set_hit((status_snapshot.hit() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16); }
            BonusType::AspdPercentage(value) => { status_snapshot.set_aspd((status_snapshot.aspd() + ((200.0 - status_snapshot.aspd()) * (*value as f32 / 100.0)))); }
            BonusType::MaxhpPercentage(value) => { status_snapshot.set_max_hp((status_snapshot.max_hp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32); }
            BonusType::MaxspPercentage(value) => { status_snapshot.set_max_sp((status_snapshot.max_sp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32); }
            BonusType::VitDefPercentage(_) => {}
            BonusType::DefPercentage(value) => { status_snapshot.set_def((status_snapshot.def() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16); }
            BonusType::MatkBasedOnStaffPercentage(_) => {}
            BonusType::AtkPercentage(value) => { status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16); }
            BonusType::PerfectHitPercentage(_) => {}
            BonusType::CriticalDamagePercentage(value) => { status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16); }
            BonusType::CastTimePercentage(_) => {}
            BonusType::CastTimeWhenUsingSkillIdPercentage(_, _) => {}
            BonusType::AfterCastDelayPercentage(_) => {}
            BonusType::NaturalHpRecoveryPercentage(_) => {}
            BonusType::NaturalSpRecoveryPercentage(_) => {}
            BonusType::SpRecoveryMaxSpPercentage(_) => {}
            BonusType::HpRecoveryMaxSpPercentage(_) => {}
            _ => {}
        };
    }

    pub fn merge_bonuses(bonuses: &Vec<BonusType>) -> Vec<BonusType> {
        let mut merged_bonuses: Vec<BonusType> = Vec::with_capacity(bonuses.len());
        for bonus in bonuses.iter() {
            if merged_bonuses.contains(bonus) {
                continue
            }
            merged_bonuses.push(Self::get_bonus_sum(bonus, &bonuses));
        }
        merged_bonuses
    }
    pub fn get_bonus_sum(bonus: &BonusType, bonuses: &Vec<BonusType>) -> BonusType {
        let val: i32 = bonuses.into_iter().filter_map(|b|
            if b == bonus {
                match b {
                    BonusType::Str(val) => Some(*val as i32),
                    BonusType::Agi(val) => Some(*val as i32),
                    BonusType::Vit(val) => Some(*val as i32),
                    BonusType::Int(val) => Some(*val as i32),
                    BonusType::Dex(val) => Some(*val as i32),
                    BonusType::Luk(val) => Some(*val as i32),
                    BonusType::AllStats(val) => Some(*val as i32),
                    BonusType::Hit(val) => Some(*val as i32),
                    BonusType::HitPercentage(val) => Some(*val as i32),
                    BonusType::AccuracyPercentage(val) => Some(*val as i32),
                    BonusType::Flee(val) => Some(*val as i32),
                    BonusType::Crit(val) => Some(*val as i32),
                    BonusType::PerfectDodge(val) => Some(*val as i32),
                    BonusType::Aspd(val) => Some(*val as i32),
                    BonusType::AspdPercentage(val) => Some(*val as i32),
                    BonusType::Maxhp(val) => Some(*val as i32),
                    BonusType::Maxsp(val) => Some(*val as i32),
                    BonusType::MaxhpPercentage(val) => Some(*val as i32),
                    BonusType::MaxspPercentage(val) => Some(*val as i32),
                    BonusType::Atk(val) => Some(*val as i32),
                    BonusType::Def(val) => Some(*val as i32),
                    BonusType::VitDefPercentage(val) => Some(*val as i32),
                    BonusType::DefPercentage(val) => Some(*val as i32),
                    BonusType::Mdef(val) => Some(*val as i32),
                    BonusType::Matk(val) => Some(*val as i32),
                    BonusType::MatkBasedOnStaffPercentage(val) => Some(*val as i32),
                    BonusType::MatkPercentage(val) => Some(*val as i32),
                    BonusType::AtkPercentage(val) => Some(*val as i32),
                    BonusType::PerfectHitPercentage(val) => Some(*val as i32),
                    BonusType::ElementWeapon(val) => Some(*val as i32),
                    BonusType::ElementDefense(val) => Some(*val as i32),
                    BonusType::CriticalDamagePercentage(val) => Some(*val as i32),
                    BonusType::CastTimePercentage(val) => Some(*val as i32),
                    BonusType::AfterCastDelayPercentage(val) => Some(*val as i32),
                    BonusType::NaturalHpRecoveryPercentage(val) => Some(*val as i32),
                    BonusType::NaturalSpRecoveryPercentage(val) => Some(*val as i32),
                    BonusType::HpRecoveryMaxSpPercentage(val) => Some(*val as i32),
                    BonusType::SpRecoveryMaxSpPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromItemPercentage(val) => Some(*val as i32),
                    BonusType::SpRegenFromItemPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromHerbPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromFruitPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromMeatPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromCandyPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromJuicePercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromFishPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromFoodPercentage(val) => Some(*val as i32),
                    BonusType::HpRegenFromPotionPercentage(val) => Some(*val as i32),
                    BonusType::GainHpWhenKillingEnemy(val) => Some(*val as i32),
                    BonusType::GainHpWhenKillingEnemyWithMagicAttack(val) => Some(*val as i32),
                    BonusType::GainSpWhenKillingEnemyWithMagicAttack(val) => Some(*val as i32),
                    BonusType::HpRegenFromSkillPercentage(val) => Some(*val as i32),
                    BonusType::GainSpWhenHittingEnemy(val) => Some(*val as i32),
                    BonusType::GainSpWhenKillingEnemy(val) => Some(*val as i32),
                    BonusType::SpConsumption(val) => Some(*val as i32),
                    BonusType::NormalAttackPercentage(val) => Some(*val as i32),
                    BonusType::NullifyAttackChancePercentage(val) => Some(*val as i32),
                    BonusType::BreakArmorPercentage(val) => Some(*val as i32),
                    BonusType::BreakWeaponPercentage(val) => Some(*val as i32),
                    BonusType::BreakSelfWeaponPercentage(val) => Some(*val as i32),
                    BonusType::ClassChangePercentageOnHit(val) => Some(*val as i32),
                    BonusType::LongRangeCriticalChance(val) => Some(*val as i32),
                    BonusType::SkillDelayIncDecPercentage(val) => Some(*val as i32),
                    BonusType::DoubleAttackChancePercentage(val) => Some(*val as i32),
                    BonusType::HealSkillPercentage(val) => Some(*val as i32),
                    BonusType::ResistanceRangeAttackPercentage(val) => Some(*val as i32),
                    BonusType::DamageRangedAtkPercentage(val) => Some(*val as i32),
                    BonusType::ResistanceMagicAttackPercentage(val) => Some(*val as i32),
                    BonusType::MagicAttackReflectChancePercentage(val) => Some(*val as i32),
                    BonusType::PhysicalAttackReflectChancePercentage(val) => Some(*val as i32),
                    BonusType::PhysicalAttackBlockChancePercentage(val) => Some(*val as i32),
                    BonusType::SplashRadius(val) => Some(*val as i32),
                    BonusType::VisionDistance(val) => Some(*val as i32),
                    BonusType::SpeedPercentage(val) => Some(*val as i32),
                    BonusType::DropChanceJewelPercentage(val) => Some(*val as i32),
                    BonusType::DropChanceOrePercentage(val) => Some(*val as i32),
                    BonusType::DropChanceRecoveryPercentage(val) => Some(*val as i32),
                    BonusType::DropChanceFoodPercentage(val) => Some(*val as i32),
                    BonusType::CastTimeWhenUsingSkillIdPercentage(_, val) => Some(*val as i32),
                    BonusType::HpRegenFromItemIDPercentage(_, val) => Some(*val as i32),
                    BonusType::PhysicalDamageAgainstSizePercentage(_, val) => Some(*val as i32),
                    BonusType::MagicalDamageAgainstSizePercentage(_, val) => Some(*val as i32),
                    BonusType::PhysicalDamageAgainstRacePercentage(_, val) => Some(*val as i32),
                    BonusType::PhysicalDamageAgainstMobIdPercentage(_, val) => Some(*val as i32),
                    BonusType::DamageUsingElementPercentage(_, val) => Some(*val as i32),
                    BonusType::DamageUsingWeaponType(_, val) => Some(*val as i32),
                    BonusType::MagicalDamageAgainstRacePercentage(_, val) => Some(*val as i32),
                    BonusType::PhysicalDamageAgainstElementPercentage(_, val) => Some(*val as i32),
                    BonusType::DamageAgainstMobGroupPercentage(_, val) => Some(*val as i32),
                    BonusType::CriticalAgainstRacePercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(_, val) => Some(*val as i32),
                    BonusType::PhysicalDamageAgainstClassPercentage(_, val) => Some(*val as i32),
                    BonusType::ResistanceDamageFromClassPercentage(_, val) => Some(*val as i32),
                    BonusType::ResistanceDamageFromElementPercentage(_, val) => Some(*val as i32),
                    BonusType::ResistanceDamageFromRacePercentage(_, val) => Some(*val as i32),
                    BonusType::ResistanceDamageFromSizePercentage(_, val) => Some(*val as i32),
                    BonusType::IgnoreDefRacePercentage(_, val) => Some(*val as i32),
                    BonusType::IgnoreMDefRacePercentage(_, val) => Some(*val as i32),
                    BonusType::IgnoreMDefClassPercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusWhenHitPercentage(_, val) => Some(*val as i32),
                    BonusType::ChanceToInflictStatusOnAttackPercentage(_, val) => Some(*val as i32),
                    BonusType::ResistanceToStatusPercentage(_, val) => Some(*val as i32),
                    BonusType::GainExpWhenKillingRacePercentage(_, val) => Some(*val as i32),
                    BonusType::SpDrainWhenAttackingRace(_, val) => Some(*val as i32),
                    BonusType::SpDrainWhenKillingRace(_, val) => Some(*val as i32),
                    BonusType::HealSkillIdPercentage(_, val) => Some(*val as i32),
                    BonusType::ResistancePhysicalAttackFromMobIdPercentage(_, val) => Some(*val as i32),
                    BonusType::DropChanceItemIdPercentage(_, val) => Some(*val as i32),
                    BonusType::KnockbackWhenUsingSkillId(_, val) => Some(*val as i32),
                    BonusType::GainZenyWhenKillingMonster(_, val) => Some(*val as i32),
                    BonusType::HpDrainWhenAttackingPercentage(_, val) => Some(*val as i32),
                    BonusType::SpDrainWhenAttackingPercentage(_, val) => Some(*val as i32),
                    BonusType::SpDrainPerHit(val) => Some(*val as i32),
                    BonusType::SpBurnOnTargetWhenAttackingPercentage(_, val) => Some(*val as i32),
                    BonusType::HpLossEveryMs(_, val) => Some(*val as i32),
                    BonusType::HpRegenEveryMs(_, val) => Some(*val as i32),
                    BonusType::SpLossEveryMs(_, val) => Some(*val as i32),
                    BonusType::SpRegenEveryMs(_, val) => Some(*val as i32),
                    BonusType::SkillIdDamagePercentage(_, val) => Some(*val as i32),
                    BonusType::EnableSkillId(_, val) => Some(*val as i32),
                    BonusType::SkillIdSuccessPercentage(_, val) => Some(*val as i32),
                    BonusType::AutospellSkillIdChancePercentage(_, val) => Some(*val as i32),
                    BonusType::DoubleCastSkillIdChancePercentage(_, val) => Some(*val as i32),
                    _ => None,
                }
            } else {
                None
            }
        ).sum();

        match bonus {
            BonusType::Str(_) => BonusType::Str(val as i8),
            BonusType::Agi(_) => BonusType::Agi(val as i8),
            BonusType::Vit(_) => BonusType::Vit(val as i8),
            BonusType::Int(_) => BonusType::Int(val as i8),
            BonusType::Dex(_) => BonusType::Dex(val as i8),
            BonusType::Luk(_) => BonusType::Luk(val as i8),
            BonusType::AllStats(_) => BonusType::AllStats(val as i8),
            BonusType::Hit(_) => BonusType::Hit(val as i16),
            BonusType::HitPercentage(_) => BonusType::HitPercentage(val as i8),
            BonusType::Flee(_) => BonusType::Flee(val as i16),
            BonusType::Crit(_) => BonusType::Crit(val as i8),
            BonusType::PerfectDodge(_) => BonusType::PerfectDodge(val as i8),
            BonusType::Aspd(_) => BonusType::Aspd(val as i8),
            BonusType::AspdPercentage(_) => BonusType::AspdPercentage(val as i8),
            BonusType::Maxhp(_) => BonusType::Maxhp(val as i32),
            BonusType::Maxsp(_) => BonusType::Maxsp(val as i32),
            BonusType::MaxhpPercentage(_) => BonusType::MaxhpPercentage(val as i8),
            BonusType::MaxspPercentage(_) => BonusType::MaxspPercentage(val as i8),
            BonusType::Atk(_) => BonusType::Atk(val as i16),
            BonusType::Def(_) => BonusType::Def(val as i16),
            BonusType::AccuracyPercentage(_) => BonusType::AccuracyPercentage(val as i8),
            BonusType::VitDefPercentage(_) => BonusType::VitDefPercentage(val as i8),
            BonusType::DefPercentage(_) => BonusType::DefPercentage(val as i8),
            BonusType::Mdef(_) => BonusType::Mdef(val as i16),
            BonusType::Matk(_) => BonusType::Matk(val as i16),
            BonusType::MatkBasedOnStaffPercentage(_) => BonusType::MatkBasedOnStaffPercentage(val as i8),
            BonusType::MatkPercentage(_) => BonusType::MatkPercentage(val as i8),
            BonusType::AtkPercentage(_) => BonusType::AtkPercentage(val as i8),
            BonusType::PerfectHitPercentage(_) => BonusType::PerfectHitPercentage(val as i8),
            BonusType::ElementWeapon(e) => BonusType::ElementWeapon(*e),
            BonusType::ElementDefense(e) => BonusType::ElementDefense(*e),
            BonusType::CriticalDamagePercentage(_) => BonusType::CriticalDamagePercentage(val as i8),
            BonusType::CastTimePercentage(_) => BonusType::CastTimePercentage(val as i8),
            BonusType::CastTimeWhenUsingSkillIdPercentage(variant, _) => BonusType::CastTimeWhenUsingSkillIdPercentage(*variant, val as i8),
            BonusType::AfterCastDelayPercentage(_) => BonusType::AfterCastDelayPercentage(val as i8),
            BonusType::NaturalHpRecoveryPercentage(_) => BonusType::NaturalHpRecoveryPercentage(val as i8),
            BonusType::NaturalSpRecoveryPercentage(_) => BonusType::NaturalSpRecoveryPercentage(val as i8),
            BonusType::HpRecoveryMaxSpPercentage(_) => BonusType::HpRecoveryMaxSpPercentage(val as i8),
            BonusType::SpRecoveryMaxSpPercentage(_) => BonusType::SpRecoveryMaxSpPercentage(val as i8),
            BonusType::HpRegenFromItemPercentage(_) => BonusType::HpRegenFromItemPercentage(val as i8),
            BonusType::SpRegenFromItemPercentage(_) => BonusType::SpRegenFromItemPercentage(val as i8),
            BonusType::HpRegenFromItemIDPercentage(variant, _) => BonusType::HpRegenFromItemIDPercentage(*variant, val as i8),
            BonusType::HpRegenFromHerbPercentage(_) => BonusType::HpRegenFromHerbPercentage(val as i8),
            BonusType::HpRegenFromFruitPercentage(_) => BonusType::HpRegenFromFruitPercentage(val as i8),
            BonusType::HpRegenFromMeatPercentage(_) => BonusType::HpRegenFromMeatPercentage(val as i8),
            BonusType::HpRegenFromCandyPercentage(_) => BonusType::HpRegenFromCandyPercentage(val as i8),
            BonusType::HpRegenFromJuicePercentage(_) => BonusType::HpRegenFromJuicePercentage(val as i8),
            BonusType::HpRegenFromFishPercentage(_) => BonusType::HpRegenFromFishPercentage(val as i8),
            BonusType::HpRegenFromFoodPercentage(_) => BonusType::HpRegenFromFoodPercentage(val as i8),
            BonusType::HpRegenFromPotionPercentage(_) => BonusType::HpRegenFromPotionPercentage(val as i8),
            BonusType::GainHpWhenKillingEnemy(_) => BonusType::GainHpWhenKillingEnemy(val as i8),
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(_) => BonusType::GainHpWhenKillingEnemyWithMagicAttack(val as i8),
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(_) => BonusType::GainSpWhenKillingEnemyWithMagicAttack(val as i8),
            BonusType::HpRegenFromSkillPercentage(_) => BonusType::HpRegenFromSkillPercentage(val as i8),
            BonusType::DisableHpRegen => BonusType::DisableHpRegen,
            BonusType::DisableSpRegen => BonusType::DisableSpRegen,
            BonusType::GainSpWhenHittingEnemy(_) => BonusType::GainSpWhenHittingEnemy(val as i8),
            BonusType::GainSpWhenKillingEnemy(_) => BonusType::GainSpWhenKillingEnemy(val as i8),
            BonusType::SpConsumption(_) => BonusType::SpConsumption(val as i8),
            BonusType::NormalAttackPercentage(_) => BonusType::NormalAttackPercentage(val as i8),
            BonusType::NullifyAttackChancePercentage(_) => BonusType::NullifyAttackChancePercentage(val as u8),
            BonusType::PhysicalDamageAgainstSizePercentage(variant, _) => BonusType::PhysicalDamageAgainstSizePercentage(*variant, val as i8),
            BonusType::MagicalDamageAgainstSizePercentage(variant, _) => BonusType::MagicalDamageAgainstSizePercentage(*variant, val as i8),
            BonusType::PhysicalDamageAgainstRacePercentage(variant, _) => BonusType::PhysicalDamageAgainstRacePercentage(*variant, val as i8),
            BonusType::MagicalDamageAgainstRacePercentage(variant, _) => BonusType::MagicalDamageAgainstRacePercentage(*variant, val as i8),
            BonusType::PhysicalDamageAgainstElementPercentage(variant, _) => BonusType::PhysicalDamageAgainstElementPercentage(*variant, val as i8),
            BonusType::DamageAgainstMobGroupPercentage(variant, _) => BonusType::DamageAgainstMobGroupPercentage(*variant, val as i8),
            BonusType::CriticalAgainstRacePercentage(variant, _) => BonusType::CriticalAgainstRacePercentage(*variant, val as i8),
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(variant, _) => BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(*variant, val as f32),
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(variant, _) => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(*variant, val as f32),
            BonusType::PhysicalDamageAgainstClassPercentage(variant, _) => BonusType::PhysicalDamageAgainstClassPercentage(*variant, val as i8),
            BonusType::PhysicalDamageAgainstMobIdPercentage(variant, _) => BonusType::PhysicalDamageAgainstMobIdPercentage(*variant, val as i8),
            BonusType::DamageUsingElementPercentage(variant, _) => BonusType::DamageUsingElementPercentage(*variant, val as i8),
            BonusType::DamageUsingWeaponType(variant, _) => BonusType::DamageUsingWeaponType(*variant, val as i8),
            BonusType::ResistanceDamageFromClassPercentage(variant, _) => BonusType::ResistanceDamageFromClassPercentage(*variant, val as i8),
            BonusType::ResistanceDamageFromElementPercentage(variant, _) => BonusType::ResistanceDamageFromElementPercentage(*variant, val as i8),
            BonusType::ResistanceDamageFromRacePercentage(variant, _) => BonusType::ResistanceDamageFromRacePercentage(*variant, val as i8),
            BonusType::ResistanceDamageFromSizePercentage(variant, _) => BonusType::ResistanceDamageFromSizePercentage(*variant, val as i8),
            BonusType::IgnoreDefClass(variant) => BonusType::IgnoreDefClass(*variant),
            BonusType::IgnoreDefRace(variant) => BonusType::IgnoreDefRace(*variant),
            BonusType::IgnoreDefRacePercentage(variant, _) => BonusType::IgnoreDefRacePercentage(*variant, val as i8),
            BonusType::IgnoreMDefRacePercentage(variant, _) => BonusType::IgnoreMDefRacePercentage(*variant, val as i8),
            BonusType::IgnoreMDefClassPercentage(variant, _) => BonusType::IgnoreMDefClassPercentage(*variant, val as i8),
            BonusType::IncreaseDamageAgainstClassBaseOnDef(variant) => BonusType::IncreaseDamageAgainstClassBaseOnDef(*variant),
            BonusType::ChanceToInflictStatusOnAttackPercentage(variant, _) => BonusType::ChanceToInflictStatusOnAttackPercentage(*variant, val as f32),
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(variant, _) => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(*variant, val as f32),
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(variant, _) => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(*variant, val as f32),
            BonusType::ChanceToInflictStatusWhenHitPercentage(variant, _) => BonusType::ChanceToInflictStatusWhenHitPercentage(*variant, val as f32),
            BonusType::ResistanceToStatusPercentage(variant, _) => BonusType::ResistanceToStatusPercentage(*variant, val as f32),
            BonusType::GainExpWhenKillingRacePercentage(variant, _) => BonusType::GainExpWhenKillingRacePercentage(*variant, val as i8),
            BonusType::SpDrainWhenAttackingRace(variant, _) => BonusType::SpDrainWhenAttackingRace(*variant, val as u16),
            BonusType::BreakArmorPercentage(_) => BonusType::BreakArmorPercentage(val as i8),
            BonusType::BreakWeaponPercentage(_) => BonusType::BreakWeaponPercentage(val as i8),
            BonusType::BreakSelfWeaponPercentage(_) => BonusType::BreakSelfWeaponPercentage(val as i8),
            BonusType::ClassChangePercentageOnHit(_) => BonusType::ClassChangePercentageOnHit(val as i8),
            BonusType::LongRangeCriticalChance(_) => BonusType::LongRangeCriticalChance(val as i8),
            BonusType::SkillDelayIncDecPercentage(_) => BonusType::SkillDelayIncDecPercentage(val as i8),
            BonusType::DoubleAttackChancePercentage(_) => BonusType::DoubleAttackChancePercentage(val as i8),
            BonusType::HealSkillPercentage(_) => BonusType::HealSkillPercentage(val as i8),
            BonusType::HealSkillIdPercentage(variant, _) => BonusType::HealSkillIdPercentage(*variant, val as i8),
            BonusType::ResistanceRangeAttackPercentage(_) => BonusType::ResistanceRangeAttackPercentage(val as i8),
            BonusType::DamageRangedAtkPercentage(_) => BonusType::DamageRangedAtkPercentage(val as i8),
            BonusType::ResistanceMagicAttackPercentage(_) => BonusType::ResistanceMagicAttackPercentage(val as i8),
            BonusType::MagicAttackReflectChancePercentage(_) => BonusType::MagicAttackReflectChancePercentage(val as i8),
            BonusType::PhysicalAttackReflectChancePercentage(_) => BonusType::PhysicalAttackReflectChancePercentage(val as i8),
            BonusType::PhysicalAttackBlockChancePercentage(_) => BonusType::PhysicalAttackBlockChancePercentage(val as i8),
            BonusType::SplashRadius(_) => BonusType::SplashRadius(val as i8),
            BonusType::VisionDistance(_) => BonusType::VisionDistance(val as i8),
            BonusType::SpeedPercentage(_) => BonusType::SpeedPercentage(val as i8),
            BonusType::EnableFullHpSpRecoverOnResurrect => BonusType::EnableFullHpSpRecoverOnResurrect,
            BonusType::EnableSeeHidden => BonusType::EnableSeeHidden,
            BonusType::EnableNoCancelCast => BonusType::EnableNoCancelCast,
            BonusType::EnableNoGemstoneRequired => BonusType::EnableNoGemstoneRequired,
            BonusType::EnableIgnoreSizeModifier => BonusType::EnableIgnoreSizeModifier,
            BonusType::EnableNoKnockback => BonusType::EnableNoKnockback,
            BonusType::EnableNoWalkDelay => BonusType::EnableNoWalkDelay,
            BonusType::UnbreakableArmor => BonusType::UnbreakableArmor,
            BonusType::UnbreakableShoulder => BonusType::UnbreakableShoulder,
            BonusType::UnbreakableHelm => BonusType::UnbreakableHelm,
            BonusType::UnbreakableShield => BonusType::UnbreakableShield,
            BonusType::UnbreakableShoes => BonusType::UnbreakableShoes,
            BonusType::UnbreakableWeapon => BonusType::UnbreakableWeapon,
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(variant, _) => BonusType::ResistancePhysicalAttackFromMobIdPercentage(*variant, val as i8),
            BonusType::DropChanceItemIdPercentage(variant, _) => BonusType::DropChanceItemIdPercentage(*variant, val as i8),
            BonusType::DropChanceJewelPercentage(_) => BonusType::DropChanceJewelPercentage(val as i8),
            BonusType::DropChanceOrePercentage(_) => BonusType::DropChanceOrePercentage(val as i8),
            BonusType::DropChanceRecoveryPercentage(_) => BonusType::DropChanceRecoveryPercentage(val as i8),
            BonusType::DropChanceFoodPercentage(_) => BonusType::DropChanceFoodPercentage(val as i8),
            BonusType::KnockbackWhenUsingSkillId(variant, _) => BonusType::KnockbackWhenUsingSkillId(*variant, val as i8),
            BonusType::GainZenyWhenKillingMonster(variant, _) => BonusType::GainZenyWhenKillingMonster(*variant, val as i8),
            BonusType::HpDrainWhenAttackingPercentage(variant, _) => BonusType::HpDrainWhenAttackingPercentage(*variant, val as i8),
            BonusType::SpDrainWhenAttackingPercentage(variant, _) => BonusType::SpDrainWhenAttackingPercentage(*variant, val as i8),
            BonusType::SpDrainWhenKillingRace(variant, _) => BonusType::SpDrainWhenKillingRace(*variant, val as u16),
            BonusType::SpBurnOnTargetWhenAttackingPercentage(variant, _) => BonusType::SpBurnOnTargetWhenAttackingPercentage(*variant, val as u16),
            BonusType::SpDrainPerHit(_) => BonusType::SpDrainPerHit(val as i8),
            BonusType::HpLossEveryMs(variant, _) => BonusType::HpLossEveryMs(*variant, val as u16),
            BonusType::HpRegenEveryMs(variant, _) => BonusType::HpRegenEveryMs(*variant, val as u16),
            BonusType::SpLossEveryMs(variant, _) => BonusType::SpLossEveryMs(*variant, val as u16),
            BonusType::SpRegenEveryMs(variant, _) => BonusType::SpRegenEveryMs(*variant, val as u16),
            BonusType::SkillIdDamagePercentage(variant, _) => BonusType::SkillIdDamagePercentage(*variant, val as i8),
            BonusType::EnableSkillId(variant, _) => BonusType::EnableSkillId(*variant, val as u8),
            BonusType::AutospellSkillIdChancePercentage(variant, _) => BonusType::AutospellSkillIdChancePercentage(*variant, val as i8),
            BonusType::DoubleCastSkillIdChancePercentage(variant, _) => BonusType::DoubleCastSkillIdChancePercentage(*variant, val as i8),
            BonusType::SkillIdSuccessPercentage(variant, _) => BonusType::SkillIdSuccessPercentage(*variant, val as i8),
        }
    }

    #[inline]
    pub fn get_bonus<'a>(bonus: &BonusType, bonuses: &'a Vec<&BonusType>) -> Option<&'a BonusType> {
        bonuses.iter().find(|b| **b == bonus).map(|b| *b)
    }
    #[inline]
    pub fn get_bonus_value<'a>(bonus: &BonusType, bonuses: &'a Vec<&BonusType>) -> Option<f32> {
        Self::get_bonus(bonus, bonuses).map(|b| match b {
            BonusType::Str(val) => *val as f32,
            BonusType::Agi(val) => *val as f32,
            BonusType::Vit(val) => *val as f32,
            BonusType::Int(val) => *val as f32,
            BonusType::Dex(val) => *val as f32,
            BonusType::Luk(val) => *val as f32,
            BonusType::AllStats(val) => *val as f32,
            BonusType::Hit(val) => *val as f32,
            BonusType::HitPercentage(val) => *val as f32,
            BonusType::AccuracyPercentage(val) => *val as f32,
            BonusType::Flee(val) => *val as f32,
            BonusType::Crit(val) => *val as f32,
            BonusType::PerfectDodge(val) => *val as f32,
            BonusType::Aspd(val) => *val as f32,
            BonusType::AspdPercentage(val) => *val as f32,
            BonusType::Maxhp(val) => *val as f32,
            BonusType::Maxsp(val) => *val as f32,
            BonusType::MaxhpPercentage(val) => *val as f32,
            BonusType::MaxspPercentage(val) => *val as f32,
            BonusType::Atk(val) => *val as f32,
            BonusType::Def(val) => *val as f32,
            BonusType::VitDefPercentage(val) => *val as f32,
            BonusType::DefPercentage(val) => *val as f32,
            BonusType::Mdef(val) => *val as f32,
            BonusType::Matk(val) => *val as f32,
            BonusType::MatkBasedOnStaffPercentage(val) => *val as f32,
            BonusType::MatkPercentage(val) => *val as f32,
            BonusType::AtkPercentage(val) => *val as f32,
            BonusType::PerfectHitPercentage(val) => *val as f32,
            BonusType::CriticalDamagePercentage(val) => *val as f32,
            BonusType::CastTimePercentage(val) => *val as f32,
            BonusType::CastTimeWhenUsingSkillIdPercentage(_, val) => *val as f32,
            BonusType::AfterCastDelayPercentage(val) => *val as f32,
            BonusType::NaturalHpRecoveryPercentage(val) => *val as f32,
            BonusType::NaturalSpRecoveryPercentage(val) => *val as f32,
            BonusType::HpRecoveryMaxSpPercentage(val) => *val as f32,
            BonusType::SpRecoveryMaxSpPercentage(val) => *val as f32,
            BonusType::HpRegenFromItemPercentage(val) => *val as f32,
            BonusType::SpRegenFromItemPercentage(val) => *val as f32,
            BonusType::HpRegenFromItemIDPercentage(_, val) => *val as f32,
            BonusType::HpRegenFromHerbPercentage(val) => *val as f32,
            BonusType::HpRegenFromFruitPercentage(val) => *val as f32,
            BonusType::HpRegenFromMeatPercentage(val) => *val as f32,
            BonusType::HpRegenFromCandyPercentage(val) => *val as f32,
            BonusType::HpRegenFromJuicePercentage(val) => *val as f32,
            BonusType::HpRegenFromFishPercentage(val) => *val as f32,
            BonusType::HpRegenFromFoodPercentage(val) => *val as f32,
            BonusType::HpRegenFromPotionPercentage(val) => *val as f32,
            BonusType::GainHpWhenKillingEnemy(val) => *val as f32,
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(val) => *val as f32,
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(val) => *val as f32,
            BonusType::HpRegenFromSkillPercentage(val) => *val as f32,
            BonusType::GainSpWhenHittingEnemy(val) => *val as f32,
            BonusType::GainSpWhenKillingEnemy(val) => *val as f32,
            BonusType::SpConsumption(val) => *val as f32,
            BonusType::NormalAttackPercentage(val) => *val as f32,
            BonusType::NullifyAttackChancePercentage(val) => *val as f32,
            BonusType::PhysicalDamageAgainstSizePercentage(_, val) => *val as f32,
            BonusType::MagicalDamageAgainstSizePercentage(_, val) => *val as f32,
            BonusType::PhysicalDamageAgainstRacePercentage(_, val) => *val as f32,
            BonusType::MagicalDamageAgainstRacePercentage(_, val) => *val as f32,
            BonusType::PhysicalDamageAgainstElementPercentage(_, val) => *val as f32,
            BonusType::DamageAgainstMobGroupPercentage(_, val) => *val as f32,
            BonusType::CriticalAgainstRacePercentage(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(_, val) => *val as f32,
            BonusType::PhysicalDamageAgainstClassPercentage(_, val) => *val as f32,
            BonusType::ResistanceDamageFromClassPercentage(_, val) => *val as f32,
            BonusType::ResistanceDamageFromElementPercentage(_, val) => *val as f32,
            BonusType::ResistanceDamageFromRacePercentage(_, val) => *val as f32,
            BonusType::ResistanceDamageFromSizePercentage(_, val) => *val as f32,
            BonusType::IgnoreDefRacePercentage(_, val) => *val as f32,
            BonusType::IgnoreMDefRacePercentage(_, val) => *val as f32,
            BonusType::IgnoreMDefClassPercentage(_, val) => *val as f32,
            BonusType::PhysicalDamageAgainstMobIdPercentage(_, val) => *val as f32,
            BonusType::DamageUsingElementPercentage(_, val) => *val as f32,
            BonusType::DamageUsingWeaponType(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusWhenHitPercentage(_, val) => *val as f32,
            BonusType::ChanceToInflictStatusOnAttackPercentage(_, val) => *val as f32,
            BonusType::ResistanceToStatusPercentage(_, val) => *val as f32,
            BonusType::GainExpWhenKillingRacePercentage(_, val) => *val as f32,
            BonusType::SpDrainWhenAttackingRace(_, val) => *val as f32,
            BonusType::SpDrainWhenKillingRace(_, val) => *val as f32,
            BonusType::BreakArmorPercentage(val) => *val as f32,
            BonusType::BreakWeaponPercentage(val) => *val as f32,
            BonusType::BreakSelfWeaponPercentage(val) => *val as f32,
            BonusType::ClassChangePercentageOnHit(val) => *val as f32,
            BonusType::LongRangeCriticalChance(val) => *val as f32,
            BonusType::SkillDelayIncDecPercentage(val) => *val as f32,
            BonusType::DoubleAttackChancePercentage(val) => *val as f32,
            BonusType::HealSkillPercentage(val) => *val as f32,
            BonusType::HealSkillIdPercentage(_, val) => *val as f32,
            BonusType::ResistanceRangeAttackPercentage(val) => *val as f32,
            BonusType::DamageRangedAtkPercentage(val) => *val as f32,
            BonusType::ResistanceMagicAttackPercentage(val) => *val as f32,
            BonusType::MagicAttackReflectChancePercentage(val) => *val as f32,
            BonusType::PhysicalAttackReflectChancePercentage(val) => *val as f32,
            BonusType::PhysicalAttackBlockChancePercentage(val) => *val as f32,
            BonusType::SplashRadius(val) => *val as f32,
            BonusType::VisionDistance(val) => *val as f32,
            BonusType::SpeedPercentage(val) => *val as f32,
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(_, val) => *val as f32,
            BonusType::DropChanceItemIdPercentage(_, val) => *val as f32,
            BonusType::DropChanceJewelPercentage(val) => *val as f32,
            BonusType::DropChanceOrePercentage(val) => *val as f32,
            BonusType::DropChanceRecoveryPercentage(val) => *val as f32,
            BonusType::DropChanceFoodPercentage(val) => *val as f32,
            BonusType::KnockbackWhenUsingSkillId(_, val) => *val as f32,
            BonusType::GainZenyWhenKillingMonster(_, val) => *val as f32,
            BonusType::HpDrainWhenAttackingPercentage(_, val) => *val as f32,
            BonusType::SpDrainWhenAttackingPercentage(_, val) => *val as f32,
            BonusType::SpBurnOnTargetWhenAttackingPercentage(_, val) => *val as f32,
            BonusType::SpDrainPerHit(val) => *val as f32,
            BonusType::HpLossEveryMs(_, val) => *val as f32,
            BonusType::HpRegenEveryMs(_, val) => *val as f32,
            BonusType::SpLossEveryMs(_, val) => *val as f32,
            BonusType::SpRegenEveryMs(_, val) => *val as f32,
            BonusType::SkillIdDamagePercentage(_, val) => *val as f32,
            BonusType::EnableSkillId(_, val) => *val as f32,
            BonusType::SkillIdSuccessPercentage(_, val) => *val as f32,
            BonusType::AutospellSkillIdChancePercentage(_, val) => *val as f32,
            BonusType::DoubleCastSkillIdChancePercentage(_, val) => *val as f32,
            _ => 0.0
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::bonus::BonusType;
    use crate::enums::element::Element;

    #[test]
    fn test_merge_bonus() {
        let str1 = BonusType::Str(10);
        let str2 = BonusType::Str(3);
        let damage_water1 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 10);
        let damage_water2 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 5);
        let damage_fire = BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, 5);
        let bonuses = vec![str1, str2, damage_water1, damage_water2, damage_fire];

        let merged_bonus = BonusType::merge_bonuses(&bonuses);
        println!("{:?}", merged_bonus);
        assert_eq!(merged_bonus.len(), 3);
        assert_eq!(merged_bonus[0], BonusType::Str(13));
        assert_eq!(merged_bonus[1], BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 15));
        assert_eq!(merged_bonus[2], BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, 5));
    }
    #[test]
    fn test_get_bonus() {
        let str1 = BonusType::Str(10);
        let str2 = BonusType::Str(3);
        let damage_water1 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 10);
        let damage_water2 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 5);
        let damage_fire = BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, 5);
        let bonuses = vec![str1, str2, damage_water1, damage_water2, damage_fire];

        let merged_bonus = BonusType::merge_bonuses(&bonuses);
        let bonuses_ref: Vec<&BonusType> = bonuses.iter().map(|b| b).collect();
        assert_eq!(*BonusType::get_bonus(&BonusType::Str(0), &bonuses_ref).unwrap(), BonusType::Str(13));
        assert_eq!(*BonusType::get_bonus(&BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 0), &bonuses_ref).unwrap(), BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 15));
        assert_eq!(BonusType::get_bonus(&BonusType::PhysicalDamageAgainstElementPercentage(Element::Earth, 0), &bonuses_ref), None);
    }
}
