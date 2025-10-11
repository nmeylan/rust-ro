use std::fmt::Debug;

use enum_macro::{WithEq, WithStackable};

use crate::enums::element::Element;
use crate::enums::mob::{MobClass, MobGroup, MobRace};
use crate::enums::size::Size;
use crate::enums::status::StatusEffect;
use crate::enums::weapon::WeaponType;
use crate::enums::{EnumStackable, EnumWithNumberValue};
use crate::status::StatusSnapshot;

#[derive(Debug, Clone, Copy, WithEq, WithStackable)]
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
    Crit(f32),
    PerfectDodge(i8),
    Aspd(i8),
    AspdPercentage(f32),
    Maxhp(i32),
    Maxsp(i32),
    MaxhpPercentage(i8),
    MaxspPercentage(i8),
    Atk(i16),
    AtkBaneAgainstRace(MobRace, i16),
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
    HpRecoveryMaxSpPercentage(f32),
    SpRecoveryMaxSpPercentage(f32),
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
    MasteryDamageUsingWeaponType(WeaponType, i8),

    // Only when attacking with ranged weapon
    IncreaseDamageAgainstClassBaseOnDef(MobClass),

    ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect, f32),
    ChanceToInflictStatusToPartyOnAttackPercentage(StatusEffect, f32),
    ChanceToInflictStatusOnAttackPercentage(StatusEffect, f32),
    ResistanceToStatusPercentage(StatusEffect, f32),

    GainExpWhenKillingRacePercentage(MobRace, i8),
    SpDrainWhenAttackingRace(MobRace, u16),
    SpDrainWhenKillingRace(MobRace, u16),

    BreakArmorPercentage(f32),
    BreakWeaponPercentage(i8),
    BreakSelfWeaponPercentage(f32),
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
    SkillIdSuccessPercentage(u32, f32),
    AutospellSkillIdChancePercentage(u32, f32),
    DoubleCastSkillIdChancePercentage(u32, i8),
}

impl Eq for BonusType {}
impl BonusType {
    pub fn add_bonus_to_status(&self, status_snapshot: &mut StatusSnapshot) {
        match self {
            BonusType::Str(str) => status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *str as i16),
            BonusType::Agi(agi) => status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *agi as i16),
            BonusType::Vit(vit) => status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *vit as i16),
            BonusType::Int(int) => status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *int as i16),
            BonusType::Dex(dex) => status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *dex as i16),
            BonusType::Luk(luk) => status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *luk as i16),
            BonusType::AllStats(all) => {
                status_snapshot.set_bonus_str(status_snapshot.bonus_str() + *all as i16);
                status_snapshot.set_bonus_agi(status_snapshot.bonus_agi() + *all as i16);
                status_snapshot.set_bonus_vit(status_snapshot.bonus_vit() + *all as i16);
                status_snapshot.set_bonus_int(status_snapshot.bonus_int() + *all as i16);
                status_snapshot.set_bonus_dex(status_snapshot.bonus_dex() + *all as i16);
                status_snapshot.set_bonus_luk(status_snapshot.bonus_luk() + *all as i16);
            }
            BonusType::Hit(hit) => status_snapshot.set_hit(status_snapshot.hit() + { *hit }),
            BonusType::Flee(flee) => status_snapshot.set_flee(status_snapshot.flee() + { *flee }),
            BonusType::Crit(crit) => status_snapshot.set_crit(status_snapshot.crit() + { *crit }),
            BonusType::Aspd(aspd) => status_snapshot.set_aspd(status_snapshot.aspd() + *aspd as f32),
            BonusType::Maxhp(hp) => status_snapshot.set_hp(status_snapshot.hp() + *hp as u32),
            BonusType::Maxsp(sp) => status_snapshot.set_sp(status_snapshot.sp() + *sp as u32),
            BonusType::MatkPercentage(matk_percentage) => {
                status_snapshot.set_matk_item_modifier(status_snapshot.matk_item_modifier() + (*matk_percentage as f32 / 100.0))
            }
            BonusType::Atk(atk) => status_snapshot.set_bonus_atk(status_snapshot.bonus_atk() + *atk as u16),
            BonusType::Def(def) => status_snapshot.set_def(status_snapshot.def() + { *def }),
            BonusType::Mdef(mdef) => status_snapshot.set_mdef(status_snapshot.mdef() + { *mdef }),
            BonusType::Matk(matk) => {
                status_snapshot.set_matk_min(status_snapshot.matk_min() + *matk as u16);
                status_snapshot.set_matk_max(status_snapshot.matk_max() + *matk as u16);
            }
            BonusType::ElementDefense(element) => status_snapshot.set_element(*element),
            BonusType::SpeedPercentage(speed_percentage) => {
                let speed = (status_snapshot.base_speed() as f32 * (*speed_percentage as f32 / 100.0)).ceil() as u16;
                if status_snapshot.speed() > speed {
                    status_snapshot.set_speed(status_snapshot.speed() - speed)
                }
            }
            _ => { /* TODO */ }
        }
    }

    pub fn add_percentage_bonus_to_status(&self, status_snapshot: &mut StatusSnapshot) {
        match self {
            BonusType::HitPercentage(value) => {
                status_snapshot.set_hit((status_snapshot.hit() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16);
            }
            BonusType::AspdPercentage(value) => {
                status_snapshot.set_aspd(status_snapshot.aspd() + ((200.0 - status_snapshot.aspd()) * ({ *value } / 100.0)));
            }
            BonusType::MaxhpPercentage(value) => {
                status_snapshot.set_max_hp((status_snapshot.max_hp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32);
            }
            BonusType::MaxspPercentage(value) => {
                status_snapshot.set_max_sp((status_snapshot.max_sp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32);
            }
            BonusType::VitDefPercentage(_) => {}
            BonusType::DefPercentage(value) => {
                status_snapshot.set_def((status_snapshot.def() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16);
            }
            BonusType::MatkBasedOnStaffPercentage(_) => {}
            BonusType::AtkPercentage(value) => {
                status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16);
            }
            BonusType::PerfectHitPercentage(_) => {}
            BonusType::CriticalDamagePercentage(value) => {
                status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16);
            }
            BonusType::CastTimePercentage(value) => status_snapshot.set_cast_time(status_snapshot.cast_time() + *value as f32 / 100.0),
            BonusType::CastTimeWhenUsingSkillIdPercentage(..) => {}
            BonusType::AfterCastDelayPercentage(_) => {}
            BonusType::NaturalHpRecoveryPercentage(_) => {}
            BonusType::NaturalSpRecoveryPercentage(_) => {}
            BonusType::SpRecoveryMaxSpPercentage(_) => {}
            BonusType::HpRecoveryMaxSpPercentage(_) => {}
            _ => {}
        };
    }

    fn id(&self) -> usize {
        match self {
            BonusType::Str(_) => 1usize,
            BonusType::Agi(_) => 2usize,
            BonusType::Vit(_) => 3usize,
            BonusType::Int(_) => 4usize,
            BonusType::Dex(_) => 5usize,
            BonusType::Luk(_) => 6usize,
            BonusType::AllStats(_) => 7usize,
            BonusType::Hit(_) => 8usize,
            BonusType::HitPercentage(_) => 9usize,
            BonusType::AccuracyPercentage(_) => 10usize,
            BonusType::Flee(_) => 11usize,
            BonusType::Crit(_) => 12usize,
            BonusType::PerfectDodge(_) => 13usize,
            BonusType::Aspd(_) => 14usize,
            BonusType::AspdPercentage(_) => 15usize,
            BonusType::Maxhp(_) => 16usize,
            BonusType::Maxsp(_) => 17usize,
            BonusType::MaxhpPercentage(_) => 18usize,
            BonusType::MaxspPercentage(_) => 19usize,
            BonusType::Atk(_) => 20usize,
            BonusType::AtkBaneAgainstRace(..) => 21usize,
            BonusType::Def(_) => 22usize,
            BonusType::VitDefPercentage(_) => 23usize,
            BonusType::DefPercentage(_) => 24usize,
            BonusType::Mdef(_) => 25usize,
            BonusType::Matk(_) => 26usize,
            BonusType::MatkBasedOnStaffPercentage(_) => 27usize,
            BonusType::MatkPercentage(_) => 28usize,
            BonusType::AtkPercentage(_) => 29usize,
            BonusType::PerfectHitPercentage(_) => 30usize,
            BonusType::ElementWeapon(_) => 31usize,
            BonusType::ElementDefense(_) => 32usize,
            BonusType::CriticalDamagePercentage(_) => 33usize,
            BonusType::CastTimePercentage(_) => 34usize,
            BonusType::CastTimeWhenUsingSkillIdPercentage(..) => 35usize,
            BonusType::AfterCastDelayPercentage(_) => 36usize,
            BonusType::NaturalHpRecoveryPercentage(_) => 37usize,
            BonusType::NaturalSpRecoveryPercentage(_) => 38usize,
            BonusType::HpRecoveryMaxSpPercentage(_) => 39usize,
            BonusType::SpRecoveryMaxSpPercentage(_) => 40usize,
            BonusType::HpRegenFromItemPercentage(_) => 41usize,
            BonusType::SpRegenFromItemPercentage(_) => 42usize,
            BonusType::HpRegenFromItemIDPercentage(..) => 43usize,
            BonusType::HpRegenFromHerbPercentage(_) => 44usize,
            BonusType::HpRegenFromFruitPercentage(_) => 45usize,
            BonusType::HpRegenFromMeatPercentage(_) => 46usize,
            BonusType::HpRegenFromCandyPercentage(_) => 47usize,
            BonusType::HpRegenFromJuicePercentage(_) => 48usize,
            BonusType::HpRegenFromFishPercentage(_) => 49usize,
            BonusType::HpRegenFromFoodPercentage(_) => 50usize,
            BonusType::HpRegenFromPotionPercentage(_) => 51usize,
            BonusType::GainHpWhenKillingEnemy(_) => 52usize,
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(_) => 53usize,
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(_) => 54usize,
            BonusType::HpRegenFromSkillPercentage(_) => 55usize,
            BonusType::DisableHpRegen => 56usize,
            BonusType::DisableSpRegen => 57usize,
            BonusType::GainSpWhenHittingEnemy(_) => 58usize,
            BonusType::GainSpWhenKillingEnemy(_) => 59usize,
            BonusType::SpConsumption(_) => 60usize,
            BonusType::NormalAttackPercentage(_) => 61usize,
            BonusType::NullifyAttackChancePercentage(_) => 62usize,
            BonusType::PhysicalDamageAgainstSizePercentage(..) => 63usize,
            BonusType::MagicalDamageAgainstSizePercentage(..) => 64usize,
            BonusType::PhysicalDamageAgainstRacePercentage(..) => 65usize,
            BonusType::MagicalDamageAgainstRacePercentage(..) => 66usize,
            BonusType::PhysicalDamageAgainstElementPercentage(..) => 67usize,
            BonusType::DamageAgainstMobGroupPercentage(..) => 68usize,
            BonusType::CriticalAgainstRacePercentage(..) => 69usize,
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(..) => 70usize,
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(..) => 71usize,
            BonusType::PhysicalDamageAgainstClassPercentage(..) => 72usize,
            BonusType::ResistanceDamageFromClassPercentage(..) => 73usize,
            BonusType::ResistanceDamageFromElementPercentage(..) => 74usize,
            BonusType::ResistanceDamageFromRacePercentage(..) => 75usize,
            BonusType::ResistanceDamageFromSizePercentage(..) => 76usize,
            BonusType::IgnoreDefClass(_) => 77usize,
            BonusType::IgnoreDefRace(_) => 78usize,
            BonusType::IgnoreDefRacePercentage(..) => 79usize,
            BonusType::IgnoreMDefRacePercentage(..) => 80usize,
            BonusType::IgnoreMDefClassPercentage(..) => 81usize,
            BonusType::PhysicalDamageAgainstMobIdPercentage(..) => 82usize,
            BonusType::DamageUsingElementPercentage(..) => 83usize,
            BonusType::MasteryDamageUsingWeaponType(..) => 84usize,
            BonusType::IncreaseDamageAgainstClassBaseOnDef(_) => 85usize,
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(..) => 86usize,
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(..) => 87usize,
            BonusType::ChanceToInflictStatusOnAttackPercentage(..) => 88usize,
            BonusType::ResistanceToStatusPercentage(..) => 89usize,
            BonusType::GainExpWhenKillingRacePercentage(..) => 90usize,
            BonusType::SpDrainWhenAttackingRace(..) => 91usize,
            BonusType::SpDrainWhenKillingRace(..) => 92usize,
            BonusType::BreakArmorPercentage(_) => 93usize,
            BonusType::BreakWeaponPercentage(_) => 94usize,
            BonusType::BreakSelfWeaponPercentage(_) => 95usize,
            BonusType::ClassChangePercentageOnHit(_) => 96usize,
            BonusType::LongRangeCriticalChance(_) => 97usize,
            BonusType::SkillDelayIncDecPercentage(_) => 98usize,
            BonusType::DoubleAttackChancePercentage(_) => 99usize,
            BonusType::HealSkillPercentage(_) => 100usize,
            BonusType::HealSkillIdPercentage(..) => 101usize,
            BonusType::ResistanceRangeAttackPercentage(_) => 102usize,
            BonusType::DamageRangedAtkPercentage(_) => 103usize,
            BonusType::ResistanceMagicAttackPercentage(_) => 104usize,
            BonusType::MagicAttackReflectChancePercentage(_) => 105usize,
            BonusType::PhysicalAttackReflectChancePercentage(_) => 106usize,
            BonusType::PhysicalAttackBlockChancePercentage(_) => 107usize,
            BonusType::SplashRadius(_) => 108usize,
            BonusType::SpeedPercentage(_) => 109usize,
            BonusType::VisionDistance(_) => 110usize,
            BonusType::EnableFullHpSpRecoverOnResurrect => 111usize,
            BonusType::EnableSeeHidden => 112usize,
            BonusType::EnableNoCancelCast => 113usize,
            BonusType::EnableNoGemstoneRequired => 114usize,
            BonusType::EnableIgnoreSizeModifier => 115usize,
            BonusType::EnableNoKnockback => 116usize,
            BonusType::EnableNoWalkDelay => 117usize,
            BonusType::UnbreakableArmor => 118usize,
            BonusType::UnbreakableShoulder => 119usize,
            BonusType::UnbreakableHelm => 120usize,
            BonusType::UnbreakableShield => 121usize,
            BonusType::UnbreakableShoes => 122usize,
            BonusType::UnbreakableWeapon => 123usize,
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(..) => 124usize,
            BonusType::DropChanceItemIdPercentage(..) => 125usize,
            BonusType::DropChanceJewelPercentage(_) => 126usize,
            BonusType::DropChanceOrePercentage(_) => 127usize,
            BonusType::DropChanceRecoveryPercentage(_) => 128usize,
            BonusType::DropChanceFoodPercentage(_) => 129usize,
            BonusType::KnockbackWhenUsingSkillId(..) => 130usize,
            BonusType::GainZenyWhenKillingMonster(..) => 131usize,
            BonusType::HpDrainWhenAttackingPercentage(..) => 132usize,
            BonusType::SpDrainWhenAttackingPercentage(..) => 133usize,
            BonusType::SpDrainPerHit(_) => 134usize,
            BonusType::SpBurnOnTargetWhenAttackingPercentage(..) => 135usize,
            BonusType::HpLossEveryMs(..) => 136usize,
            BonusType::HpRegenEveryMs(..) => 137usize,
            BonusType::SpLossEveryMs(..) => 138usize,
            BonusType::SpRegenEveryMs(..) => 139usize,
            BonusType::SkillIdDamagePercentage(..) => 140usize,
            BonusType::EnableSkillId(..) => 141usize,
            BonusType::SkillIdSuccessPercentage(..) => 142usize,
            BonusType::AutospellSkillIdChancePercentage(..) => 143usize,
            BonusType::DoubleCastSkillIdChancePercentage(..) => 144usize,
            _ => {
                panic!("Value can\'t be found for enum {:?}", self);
            }
        }
    }

    pub fn serialize_to_sc_data(&self) -> (i32, i32, i32) {
        match self {
            BonusType::Str(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Agi(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Vit(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Int(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Dex(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Luk(val) => (self.id() as i32, *val as i32, 0),
            BonusType::AllStats(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Hit(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HitPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::AccuracyPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Flee(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Crit(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::PerfectDodge(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Aspd(val) => (self.id() as i32, *val as i32, 0),
            BonusType::AspdPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::Maxhp(val) => (self.id() as i32, *val, 0),
            BonusType::Maxsp(val) => (self.id() as i32, *val, 0),
            BonusType::MaxhpPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::MaxspPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Atk(val) => (self.id() as i32, *val as i32, 0),
            BonusType::AtkBaneAgainstRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::Def(val) => (self.id() as i32, *val as i32, 0),
            BonusType::VitDefPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DefPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Mdef(val) => (self.id() as i32, *val as i32, 0),
            BonusType::Matk(val) => (self.id() as i32, *val as i32, 0),
            BonusType::MatkBasedOnStaffPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::MatkPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::AtkPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::PerfectHitPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::ElementWeapon(element) => (self.id() as i32, element.value() as i32, 0),
            BonusType::ElementDefense(element) => (self.id() as i32, element.value() as i32, 0),
            BonusType::CriticalDamagePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::CastTimePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::CastTimeWhenUsingSkillIdPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32),
            BonusType::AfterCastDelayPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::NaturalHpRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::NaturalSpRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRecoveryMaxSpPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::SpRecoveryMaxSpPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::HpRegenFromItemPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SpRegenFromItemPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromItemIDPercentage(item_id, val) => (self.id() as i32, *item_id as i32, *val as i32),
            BonusType::HpRegenFromHerbPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromFruitPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromMeatPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromCandyPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromJuicePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromFishPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromFoodPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromPotionPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::GainHpWhenKillingEnemy(val) => (self.id() as i32, *val as i32, 0),
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(val) => (self.id() as i32, *val as i32, 0),
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HpRegenFromSkillPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DisableHpRegen => (self.id() as i32, 0, 0),
            BonusType::DisableSpRegen => (self.id() as i32, 0, 0),
            BonusType::GainSpWhenHittingEnemy(val) => (self.id() as i32, *val as i32, 0),
            BonusType::GainSpWhenKillingEnemy(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SpConsumption(val) => (self.id() as i32, *val as i32, 0),
            BonusType::NormalAttackPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::NullifyAttackChancePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::PhysicalDamageAgainstSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32),
            BonusType::MagicalDamageAgainstSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32),
            BonusType::PhysicalDamageAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::MagicalDamageAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::PhysicalDamageAgainstElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32),
            BonusType::DamageAgainstMobGroupPercentage(group, val) => (self.id() as i32, group.value() as i32, *val as i32),
            BonusType::CriticalAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(class, val) => {
                (self.id() as i32, class.value() as i32, (*val * 100.0) as i32)
            }
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(race, val) => {
                (self.id() as i32, race.value() as i32, (*val * 100.0) as i32)
            }
            BonusType::PhysicalDamageAgainstClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32),
            BonusType::ResistanceDamageFromClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32),
            BonusType::ResistanceDamageFromElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32),
            BonusType::ResistanceDamageFromRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::ResistanceDamageFromSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32),
            BonusType::IgnoreDefClass(class) => (self.id() as i32, class.value() as i32, 0),
            BonusType::IgnoreDefRace(race) => (self.id() as i32, race.value() as i32, 0),
            BonusType::IgnoreDefRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::IgnoreMDefRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::IgnoreMDefClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32),
            BonusType::PhysicalDamageAgainstMobIdPercentage(mob_id, val) => (self.id() as i32, *mob_id as i32, *val as i32),
            BonusType::DamageUsingElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32),
            BonusType::MasteryDamageUsingWeaponType(weapon_type, val) => (self.id() as i32, weapon_type.value() as i32, *val as i32),
            BonusType::IncreaseDamageAgainstClassBaseOnDef(class) => (self.id() as i32, class.value() as i32, 0),
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(status, val) => {
                (self.id() as i32, status.value() as i32, (*val * 100.0) as i32)
            }
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(status, val) => {
                (self.id() as i32, status.value() as i32, (*val * 100.0) as i32)
            }
            BonusType::ChanceToInflictStatusOnAttackPercentage(status, val) => {
                (self.id() as i32, status.value() as i32, (*val * 100.0) as i32)
            }
            BonusType::ResistanceToStatusPercentage(status, val) => (self.id() as i32, status.value() as i32, (*val * 100.0) as i32),
            BonusType::GainExpWhenKillingRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::SpDrainWhenAttackingRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::SpDrainWhenKillingRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32),
            BonusType::BreakArmorPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::BreakWeaponPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::BreakSelfWeaponPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0),
            BonusType::ClassChangePercentageOnHit(val) => (self.id() as i32, *val as i32, 0),
            BonusType::LongRangeCriticalChance(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SkillDelayIncDecPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DoubleAttackChancePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HealSkillPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::HealSkillIdPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32),
            BonusType::ResistanceRangeAttackPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DamageRangedAtkPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::ResistanceMagicAttackPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::MagicAttackReflectChancePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::PhysicalAttackReflectChancePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::PhysicalAttackBlockChancePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SplashRadius(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SpeedPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::VisionDistance(val) => (self.id() as i32, *val as i32, 0),
            BonusType::EnableFullHpSpRecoverOnResurrect => (self.id() as i32, 0, 0),
            BonusType::EnableSeeHidden => (self.id() as i32, 0, 0),
            BonusType::EnableNoCancelCast => (self.id() as i32, 0, 0),
            BonusType::EnableNoGemstoneRequired => (self.id() as i32, 0, 0),
            BonusType::EnableIgnoreSizeModifier => (self.id() as i32, 0, 0),
            BonusType::EnableNoKnockback => (self.id() as i32, 0, 0),
            BonusType::EnableNoWalkDelay => (self.id() as i32, 0, 0),
            BonusType::UnbreakableArmor => (self.id() as i32, 0, 0),
            BonusType::UnbreakableShoulder => (self.id() as i32, 0, 0),
            BonusType::UnbreakableHelm => (self.id() as i32, 0, 0),
            BonusType::UnbreakableShield => (self.id() as i32, 0, 0),
            BonusType::UnbreakableShoes => (self.id() as i32, 0, 0),
            BonusType::UnbreakableWeapon => (self.id() as i32, 0, 0),
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(mob_id, val) => (self.id() as i32, *mob_id as i32, *val as i32),
            BonusType::DropChanceItemIdPercentage(item_id, val) => (self.id() as i32, *item_id as i32, *val as i32),
            BonusType::DropChanceJewelPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DropChanceOrePercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DropChanceRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::DropChanceFoodPercentage(val) => (self.id() as i32, *val as i32, 0),
            BonusType::KnockbackWhenUsingSkillId(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32),
            BonusType::GainZenyWhenKillingMonster(zeny, chance) => (self.id() as i32, *zeny as i32, *chance as i32),
            BonusType::HpDrainWhenAttackingPercentage(hp_percent, chance) => (self.id() as i32, *hp_percent as i32, *chance as i32),
            BonusType::SpDrainWhenAttackingPercentage(sp_percent, chance) => (self.id() as i32, *sp_percent as i32, *chance as i32),
            BonusType::SpDrainPerHit(val) => (self.id() as i32, *val as i32, 0),
            BonusType::SpBurnOnTargetWhenAttackingPercentage(percent, chance) => (self.id() as i32, *percent as i32, *chance as i32),
            BonusType::HpLossEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32),
            BonusType::HpRegenEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32),
            BonusType::SpLossEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32),
            BonusType::SpRegenEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32),
            BonusType::SkillIdDamagePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32),
            BonusType::EnableSkillId(skill_id, level) => (self.id() as i32, *skill_id as i32, *level as i32),
            BonusType::SkillIdSuccessPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, (*val * 100.0) as i32),
            BonusType::AutospellSkillIdChancePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, (*val * 100.0) as i32),
            BonusType::DoubleCastSkillIdChancePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32),
        }
    }

    pub fn deserialize_from_sc_data(bonus_type: i32, val1: i32, val2: i32) -> Option<BonusType> {
        match bonus_type {
            1 => Some(BonusType::Str(val1 as i8)),
            2 => Some(BonusType::Agi(val1 as i8)),
            3 => Some(BonusType::Vit(val1 as i8)),
            4 => Some(BonusType::Int(val1 as i8)),
            5 => Some(BonusType::Dex(val1 as i8)),
            6 => Some(BonusType::Luk(val1 as i8)),
            7 => Some(BonusType::AllStats(val1 as i8)),
            8 => Some(BonusType::Hit(val1 as i16)),
            9 => Some(BonusType::HitPercentage(val1 as i8)),
            10 => Some(BonusType::AccuracyPercentage(val1 as i8)),
            11 => Some(BonusType::Flee(val1 as i16)),
            12 => Some(BonusType::Crit(val1 as f32 / 100.0)),
            13 => Some(BonusType::PerfectDodge(val1 as i8)),
            14 => Some(BonusType::Aspd(val1 as i8)),
            15 => Some(BonusType::AspdPercentage(val1 as f32 / 100.0)),
            16 => Some(BonusType::Maxhp(val1)),
            17 => Some(BonusType::Maxsp(val1)),
            18 => Some(BonusType::MaxhpPercentage(val1 as i8)),
            19 => Some(BonusType::MaxspPercentage(val1 as i8)),
            20 => Some(BonusType::Atk(val1 as i16)),
            21 => Some(BonusType::AtkBaneAgainstRace(MobRace::from_value(val1 as usize), val2 as i16)),
            22 => Some(BonusType::Def(val1 as i16)),
            23 => Some(BonusType::VitDefPercentage(val1 as i8)),
            24 => Some(BonusType::DefPercentage(val1 as i8)),
            25 => Some(BonusType::Mdef(val1 as i16)),
            26 => Some(BonusType::Matk(val1 as i16)),
            27 => Some(BonusType::MatkBasedOnStaffPercentage(val1 as i8)),
            28 => Some(BonusType::MatkPercentage(val1 as i8)),
            29 => Some(BonusType::AtkPercentage(val1 as i8)),
            30 => Some(BonusType::PerfectHitPercentage(val1 as i8)),
            31 => Some(BonusType::ElementWeapon(Element::from_value(val1 as usize))),
            32 => Some(BonusType::ElementDefense(Element::from_value(val1 as usize))),
            33 => Some(BonusType::CriticalDamagePercentage(val1 as i8)),
            34 => Some(BonusType::CastTimePercentage(val1 as i8)),
            35 => Some(BonusType::CastTimeWhenUsingSkillIdPercentage(val1 as u32, val2 as i8)),
            36 => Some(BonusType::AfterCastDelayPercentage(val1 as i8)),
            37 => Some(BonusType::NaturalHpRecoveryPercentage(val1 as i8)),
            38 => Some(BonusType::NaturalSpRecoveryPercentage(val1 as i8)),
            39 => Some(BonusType::HpRecoveryMaxSpPercentage(val1 as f32 / 100.0)),
            40 => Some(BonusType::SpRecoveryMaxSpPercentage(val1 as f32 / 100.0)),
            41 => Some(BonusType::HpRegenFromItemPercentage(val1 as i8)),
            42 => Some(BonusType::SpRegenFromItemPercentage(val1 as i8)),
            43 => Some(BonusType::HpRegenFromItemIDPercentage(val1 as u32, val2 as i8)),
            44 => Some(BonusType::HpRegenFromHerbPercentage(val1 as i8)),
            45 => Some(BonusType::HpRegenFromFruitPercentage(val1 as i8)),
            46 => Some(BonusType::HpRegenFromMeatPercentage(val1 as i8)),
            47 => Some(BonusType::HpRegenFromCandyPercentage(val1 as i8)),
            48 => Some(BonusType::HpRegenFromJuicePercentage(val1 as i8)),
            49 => Some(BonusType::HpRegenFromFishPercentage(val1 as i8)),
            50 => Some(BonusType::HpRegenFromFoodPercentage(val1 as i8)),
            51 => Some(BonusType::HpRegenFromPotionPercentage(val1 as i8)),
            52 => Some(BonusType::GainHpWhenKillingEnemy(val1 as i8)),
            53 => Some(BonusType::GainHpWhenKillingEnemyWithMagicAttack(val1 as i8)),
            54 => Some(BonusType::GainSpWhenKillingEnemyWithMagicAttack(val1 as i8)),
            55 => Some(BonusType::HpRegenFromSkillPercentage(val1 as i8)),
            56 => Some(BonusType::DisableHpRegen),
            57 => Some(BonusType::DisableSpRegen),
            58 => Some(BonusType::GainSpWhenHittingEnemy(val1 as i8)),
            59 => Some(BonusType::GainSpWhenKillingEnemy(val1 as i8)),
            60 => Some(BonusType::SpConsumption(val1 as i8)),
            61 => Some(BonusType::NormalAttackPercentage(val1 as i8)),
            62 => Some(BonusType::NullifyAttackChancePercentage(val1 as u8)),
            63 => Some(BonusType::PhysicalDamageAgainstSizePercentage(
                Size::from_value(val1 as usize),
                val2 as i8,
            )),
            64 => Some(BonusType::MagicalDamageAgainstSizePercentage(
                Size::from_value(val1 as usize),
                val2 as i8,
            )),
            65 => Some(BonusType::PhysicalDamageAgainstRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            66 => Some(BonusType::MagicalDamageAgainstRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            67 => Some(BonusType::PhysicalDamageAgainstElementPercentage(
                Element::from_value(val1 as usize),
                val2 as i8,
            )),
            68 => Some(BonusType::DamageAgainstMobGroupPercentage(
                MobGroup::from_value(val1 as usize),
                val2 as i8,
            )),
            69 => Some(BonusType::CriticalAgainstRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            70 => Some(BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                MobClass::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            71 => Some(BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            72 => Some(BonusType::PhysicalDamageAgainstClassPercentage(
                MobClass::from_value(val1 as usize),
                val2 as i8,
            )),
            73 => Some(BonusType::ResistanceDamageFromClassPercentage(
                MobClass::from_value(val1 as usize),
                val2 as i8,
            )),
            74 => Some(BonusType::ResistanceDamageFromElementPercentage(
                Element::from_value(val1 as usize),
                val2 as i8,
            )),
            75 => Some(BonusType::ResistanceDamageFromRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            76 => Some(BonusType::ResistanceDamageFromSizePercentage(
                Size::from_value(val1 as usize),
                val2 as i8,
            )),
            77 => Some(BonusType::IgnoreDefClass(MobClass::from_value(val1 as usize))),
            78 => Some(BonusType::IgnoreDefRace(MobRace::from_value(val1 as usize))),
            79 => Some(BonusType::IgnoreDefRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            80 => Some(BonusType::IgnoreMDefRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            81 => Some(BonusType::IgnoreMDefClassPercentage(
                MobClass::from_value(val1 as usize),
                val2 as i8,
            )),
            82 => Some(BonusType::PhysicalDamageAgainstMobIdPercentage(val1 as u32, val2 as i8)),
            83 => Some(BonusType::DamageUsingElementPercentage(
                Element::from_value(val1 as usize),
                val2 as i8,
            )),
            84 => Some(BonusType::MasteryDamageUsingWeaponType(
                WeaponType::from_value(val1 as usize),
                val2 as i8,
            )),
            85 => Some(BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::from_value(
                val1 as usize,
            ))),
            86 => Some(BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(
                StatusEffect::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            87 => Some(BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(
                StatusEffect::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            88 => Some(BonusType::ChanceToInflictStatusOnAttackPercentage(
                StatusEffect::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            89 => Some(BonusType::ResistanceToStatusPercentage(
                StatusEffect::from_value(val1 as usize),
                val2 as f32 / 100.0,
            )),
            90 => Some(BonusType::GainExpWhenKillingRacePercentage(
                MobRace::from_value(val1 as usize),
                val2 as i8,
            )),
            91 => Some(BonusType::SpDrainWhenAttackingRace(
                MobRace::from_value(val1 as usize),
                val2 as u16,
            )),
            92 => Some(BonusType::SpDrainWhenKillingRace(
                MobRace::from_value(val1 as usize),
                val2 as u16,
            )),
            93 => Some(BonusType::BreakArmorPercentage(val1 as f32 / 100.0)),
            94 => Some(BonusType::BreakWeaponPercentage(val1 as i8)),
            95 => Some(BonusType::BreakSelfWeaponPercentage(val1 as f32 / 100.0)),
            96 => Some(BonusType::ClassChangePercentageOnHit(val1 as i8)),
            97 => Some(BonusType::LongRangeCriticalChance(val1 as i8)),
            98 => Some(BonusType::SkillDelayIncDecPercentage(val1 as i8)),
            99 => Some(BonusType::DoubleAttackChancePercentage(val1 as i8)),
            100 => Some(BonusType::HealSkillPercentage(val1 as i8)),
            101 => Some(BonusType::HealSkillIdPercentage(val1 as u32, val2 as i8)),
            102 => Some(BonusType::ResistanceRangeAttackPercentage(val1 as i8)),
            103 => Some(BonusType::DamageRangedAtkPercentage(val1 as i8)),
            104 => Some(BonusType::ResistanceMagicAttackPercentage(val1 as i8)),
            105 => Some(BonusType::MagicAttackReflectChancePercentage(val1 as i8)),
            106 => Some(BonusType::PhysicalAttackReflectChancePercentage(val1 as i8)),
            107 => Some(BonusType::PhysicalAttackBlockChancePercentage(val1 as i8)),
            108 => Some(BonusType::SplashRadius(val1 as i8)),
            109 => Some(BonusType::SpeedPercentage(val1 as i8)),
            110 => Some(BonusType::VisionDistance(val1 as i8)),
            111 => Some(BonusType::EnableFullHpSpRecoverOnResurrect),
            112 => Some(BonusType::EnableSeeHidden),
            113 => Some(BonusType::EnableNoCancelCast),
            114 => Some(BonusType::EnableNoGemstoneRequired),
            115 => Some(BonusType::EnableIgnoreSizeModifier),
            116 => Some(BonusType::EnableNoKnockback),
            117 => Some(BonusType::EnableNoWalkDelay),
            118 => Some(BonusType::UnbreakableArmor),
            119 => Some(BonusType::UnbreakableShoulder),
            120 => Some(BonusType::UnbreakableHelm),
            121 => Some(BonusType::UnbreakableShield),
            122 => Some(BonusType::UnbreakableShoes),
            123 => Some(BonusType::UnbreakableWeapon),
            124 => Some(BonusType::ResistancePhysicalAttackFromMobIdPercentage(val1 as u32, val2 as i8)),
            125 => Some(BonusType::DropChanceItemIdPercentage(val1 as u32, val2 as i8)),
            126 => Some(BonusType::DropChanceJewelPercentage(val1 as i8)),
            127 => Some(BonusType::DropChanceOrePercentage(val1 as i8)),
            128 => Some(BonusType::DropChanceRecoveryPercentage(val1 as i8)),
            129 => Some(BonusType::DropChanceFoodPercentage(val1 as i8)),
            130 => Some(BonusType::KnockbackWhenUsingSkillId(val1 as u32, val2 as i8)),
            131 => Some(BonusType::GainZenyWhenKillingMonster(val1 as u16, val2 as i8)),
            132 => Some(BonusType::HpDrainWhenAttackingPercentage(val1 as i8, val2 as i8)),
            133 => Some(BonusType::SpDrainWhenAttackingPercentage(val1 as i8, val2 as i8)),
            134 => Some(BonusType::SpDrainPerHit(val1 as i8)),
            135 => Some(BonusType::SpBurnOnTargetWhenAttackingPercentage(val1 as i8, val2 as u16)),
            136 => Some(BonusType::HpLossEveryMs(val1 as u16, val2 as u16)),
            137 => Some(BonusType::HpRegenEveryMs(val1 as u16, val2 as u16)),
            138 => Some(BonusType::SpLossEveryMs(val1 as u16, val2 as u16)),
            139 => Some(BonusType::SpRegenEveryMs(val1 as u16, val2 as u16)),
            140 => Some(BonusType::SkillIdDamagePercentage(val1 as u32, val2 as i8)),
            141 => Some(BonusType::EnableSkillId(val1 as u32, val2 as u8)),
            142 => Some(BonusType::SkillIdSuccessPercentage(val1 as u32, val2 as f32 / 100.0)),
            143 => Some(BonusType::AutospellSkillIdChancePercentage(val1 as u32, val2 as f32 / 100.0)),
            144 => Some(BonusType::DoubleCastSkillIdChancePercentage(val1 as u32, val2 as i8)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::EnumStackable;
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

        let merged_bonus = BonusType::merge_enums(&bonuses);
        println!("{:?}", merged_bonus);
        assert_eq!(merged_bonus.len(), 3);
        assert_eq!(merged_bonus[0], BonusType::Str(13));
        assert_eq!(
            merged_bonus[1],
            BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 15)
        );
        assert_eq!(
            merged_bonus[2],
            BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, 5)
        );
    }
    #[test]
    fn test_get_bonus() {
        let str1 = BonusType::Str(10);
        let str2 = BonusType::Str(3);
        let damage_water1 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 10);
        let damage_water2 = BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 5);
        let damage_fire = BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, 5);
        let bonuses = vec![str1, str2, damage_water1, damage_water2, damage_fire];

        let merged_bonus = BonusType::merge_enums(&bonuses);
        let bonuses_ref: Vec<&BonusType> = bonuses.iter().map(|b| b).collect();
        assert_eq!(
            *BonusType::get_enum(&BonusType::Str(0), &bonuses_ref).unwrap(),
            BonusType::Str(13)
        );
        assert_eq!(
            *BonusType::get_enum(
                &BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 0),
                &bonuses_ref
            )
            .unwrap(),
            BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 15)
        );
        assert_eq!(
            BonusType::get_enum(
                &BonusType::PhysicalDamageAgainstElementPercentage(Element::Earth, 0),
                &bonuses_ref
            ),
            None
        );
    }
}
