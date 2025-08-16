use crate::enums::element::Element;
use crate::enums::mob::{MobClass, MobGroup, MobRace};
use crate::enums::size::Size;
use crate::enums::status::StatusEffect;
use crate::enums::weapon::WeaponType;
use crate::enums::{EnumStackable, EnumWithNumberValue};
use crate::status::StatusSnapshot;
use enum_macro::{WithEq, WithStackable};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
#[derive(WithEq, WithStackable)]
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
            BonusType::Hit(hit) => { status_snapshot.set_hit(status_snapshot.hit() + { *hit }) }
            BonusType::Flee(flee) => { status_snapshot.set_flee(status_snapshot.flee() + { *flee }) }
            BonusType::Crit(crit) => { status_snapshot.set_crit(status_snapshot.crit() + { *crit }) }
            BonusType::Aspd(aspd) => { status_snapshot.set_aspd(status_snapshot.aspd() + *aspd as f32) }
            BonusType::Maxhp(hp) => { status_snapshot.set_hp(status_snapshot.hp() + *hp as u32) }
            BonusType::Maxsp(sp) => { status_snapshot.set_sp(status_snapshot.sp() + *sp as u32) }
            BonusType::MatkPercentage(matk_percentage) => { status_snapshot.set_matk_item_modifier(status_snapshot.matk_item_modifier() + (*matk_percentage as f32 / 100.0)) }
            BonusType::Atk(atk) => { status_snapshot.set_bonus_atk(status_snapshot.bonus_atk() + *atk as u16) }
            BonusType::Def(def) => { status_snapshot.set_def(status_snapshot.def() + { *def }) }
            BonusType::Mdef(mdef) => { status_snapshot.set_mdef(status_snapshot.mdef() + { *mdef }) }
            BonusType::Matk(matk) => {
                status_snapshot.set_matk_min(status_snapshot.matk_min() + *matk as u16);
                status_snapshot.set_matk_max(status_snapshot.matk_max() + *matk as u16);
            }
            BonusType::ElementDefense(element) => { status_snapshot.set_element(*element) }
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
            BonusType::HitPercentage(value) => { status_snapshot.set_hit((status_snapshot.hit() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16); }
            BonusType::AspdPercentage(value) => { status_snapshot.set_aspd(status_snapshot.aspd() + ((200.0 - status_snapshot.aspd()) * ({ *value } / 100.0))); }
            BonusType::MaxhpPercentage(value) => { status_snapshot.set_max_hp((status_snapshot.max_hp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32); }
            BonusType::MaxspPercentage(value) => { status_snapshot.set_max_sp((status_snapshot.max_sp() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u32); }
            BonusType::VitDefPercentage(_) => {}
            BonusType::DefPercentage(value) => { status_snapshot.set_def((status_snapshot.def() as f32 * (1.0 + *value as f32 / 100.0)).floor() as i16); }
            BonusType::MatkBasedOnStaffPercentage(_) => {}
            BonusType::AtkPercentage(value) => { status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16); }
            BonusType::PerfectHitPercentage(_) => {}
            BonusType::CriticalDamagePercentage(value) => { status_snapshot.set_bonus_atk((status_snapshot.bonus_atk() as f32 * (1.0 + *value as f32 / 100.0)).floor() as u16); }
            BonusType::CastTimePercentage(value) => { status_snapshot.set_cast_time(status_snapshot.cast_time() + *value as f32 / 100.0) }
            BonusType::CastTimeWhenUsingSkillIdPercentage(_, _) => {}
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
            BonusType::AtkBaneAgainstRace(_, _) => 21usize,
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
            BonusType::CastTimeWhenUsingSkillIdPercentage(_, _) => 35usize,
            BonusType::AfterCastDelayPercentage(_) => 36usize,
            BonusType::NaturalHpRecoveryPercentage(_) => 37usize,
            BonusType::NaturalSpRecoveryPercentage(_) => 38usize,
            BonusType::HpRecoveryMaxSpPercentage(_) => 39usize,
            BonusType::SpRecoveryMaxSpPercentage(_) => 40usize,
            BonusType::HpRegenFromItemPercentage(_) => 41usize,
            BonusType::SpRegenFromItemPercentage(_) => 42usize,
            BonusType::HpRegenFromItemIDPercentage(_, _) => 43usize,
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
            BonusType::PhysicalDamageAgainstSizePercentage(_, _) => 63usize,
            BonusType::MagicalDamageAgainstSizePercentage(_, _) => 64usize,
            BonusType::PhysicalDamageAgainstRacePercentage(_, _) => 65usize,
            BonusType::MagicalDamageAgainstRacePercentage(_, _) => 66usize,
            BonusType::PhysicalDamageAgainstElementPercentage(_, _) => 67usize,
            BonusType::DamageAgainstMobGroupPercentage(_, _) => 68usize,
            BonusType::CriticalAgainstRacePercentage(_, _) => 69usize,
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(_, _) => 70usize,
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(_, _) => 71usize,
            BonusType::PhysicalDamageAgainstClassPercentage(_, _) => 72usize,
            BonusType::ResistanceDamageFromClassPercentage(_, _) => 73usize,
            BonusType::ResistanceDamageFromElementPercentage(_, _) => 74usize,
            BonusType::ResistanceDamageFromRacePercentage(_, _) => 75usize,
            BonusType::ResistanceDamageFromSizePercentage(_, _) => 76usize,
            BonusType::IgnoreDefClass(_) => 77usize,
            BonusType::IgnoreDefRace(_) => 78usize,
            BonusType::IgnoreDefRacePercentage(_, _) => 79usize,
            BonusType::IgnoreMDefRacePercentage(_, _) => 80usize,
            BonusType::IgnoreMDefClassPercentage(_, _) => 81usize,
            BonusType::PhysicalDamageAgainstMobIdPercentage(_, _) => 82usize,
            BonusType::DamageUsingElementPercentage(_, _) => 83usize,
            BonusType::MasteryDamageUsingWeaponType(_, _) => 84usize,
            BonusType::IncreaseDamageAgainstClassBaseOnDef(_) => 85usize,
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(_, _) => 86usize,
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(_, _) => 87usize,
            BonusType::ChanceToInflictStatusOnAttackPercentage(_, _) => 88usize,
            BonusType::ResistanceToStatusPercentage(_, _) => 89usize,
            BonusType::GainExpWhenKillingRacePercentage(_, _) => 90usize,
            BonusType::SpDrainWhenAttackingRace(_, _) => 91usize,
            BonusType::SpDrainWhenKillingRace(_, _) => 92usize,
            BonusType::BreakArmorPercentage(_) => 93usize,
            BonusType::BreakWeaponPercentage(_) => 94usize,
            BonusType::BreakSelfWeaponPercentage(_) => 95usize,
            BonusType::ClassChangePercentageOnHit(_) => 96usize,
            BonusType::LongRangeCriticalChance(_) => 97usize,
            BonusType::SkillDelayIncDecPercentage(_) => 98usize,
            BonusType::DoubleAttackChancePercentage(_) => 99usize,
            BonusType::HealSkillPercentage(_) => 100usize,
            BonusType::HealSkillIdPercentage(_, _) => 101usize,
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
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(_, _) => 124usize,
            BonusType::DropChanceItemIdPercentage(_, _) => 125usize,
            BonusType::DropChanceJewelPercentage(_) => 126usize,
            BonusType::DropChanceOrePercentage(_) => 127usize,
            BonusType::DropChanceRecoveryPercentage(_) => 128usize,
            BonusType::DropChanceFoodPercentage(_) => 129usize,
            BonusType::KnockbackWhenUsingSkillId(_, _) => 130usize,
            BonusType::GainZenyWhenKillingMonster(_, _) => 131usize,
            BonusType::HpDrainWhenAttackingPercentage(_, _) => 132usize,
            BonusType::SpDrainWhenAttackingPercentage(_, _) => 133usize,
            BonusType::SpDrainPerHit(_) => 134usize,
            BonusType::SpBurnOnTargetWhenAttackingPercentage(_, _) => 135usize,
            BonusType::HpLossEveryMs(_, _) => 136usize,
            BonusType::HpRegenEveryMs(_, _) => 137usize,
            BonusType::SpLossEveryMs(_, _) => 138usize,
            BonusType::SpRegenEveryMs(_, _) => 139usize,
            BonusType::SkillIdDamagePercentage(_, _) => 140usize,
            BonusType::EnableSkillId(_, _) => 141usize,
            BonusType::SkillIdSuccessPercentage(_, _) => 142usize,
            BonusType::AutospellSkillIdChancePercentage(_, _) => 143usize,
            BonusType::DoubleCastSkillIdChancePercentage(_, _) => 144usize,
            _ => {
                panic!("Value can\'t be found for enum {:?}", self);
            }
        }
    }


    pub fn serialize_to_sc_data(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            BonusType::Str(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Agi(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Vit(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Int(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Dex(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Luk(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::AllStats(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Hit(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HitPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::AccuracyPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Flee(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Crit(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::PerfectDodge(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Aspd(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::AspdPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::Maxhp(val) => (self.id() as i32, *val, 0, 0, 0),
            BonusType::Maxsp(val) => (self.id() as i32, *val, 0, 0, 0),
            BonusType::MaxhpPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::MaxspPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Atk(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::AtkBaneAgainstRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::Def(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::VitDefPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DefPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Mdef(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::Matk(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::MatkBasedOnStaffPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::MatkPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::AtkPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::PerfectHitPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::ElementWeapon(element) => (self.id() as i32, element.value() as i32, 0, 0, 0),
            BonusType::ElementDefense(element) => (self.id() as i32, element.value() as i32, 0, 0, 0),
            BonusType::CriticalDamagePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::CastTimePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::CastTimeWhenUsingSkillIdPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32, 0, 0),
            BonusType::AfterCastDelayPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::NaturalHpRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::NaturalSpRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRecoveryMaxSpPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::SpRecoveryMaxSpPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::HpRegenFromItemPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SpRegenFromItemPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromItemIDPercentage(item_id, val) => (self.id() as i32, *item_id as i32, *val as i32, 0, 0),
            BonusType::HpRegenFromHerbPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromFruitPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromMeatPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromCandyPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromJuicePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromFishPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromFoodPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromPotionPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::GainHpWhenKillingEnemy(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::GainHpWhenKillingEnemyWithMagicAttack(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::GainSpWhenKillingEnemyWithMagicAttack(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HpRegenFromSkillPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DisableHpRegen => (self.id() as i32, 0, 0, 0, 0),
            BonusType::DisableSpRegen => (self.id() as i32, 0, 0, 0, 0),
            BonusType::GainSpWhenHittingEnemy(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::GainSpWhenKillingEnemy(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SpConsumption(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::NormalAttackPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::NullifyAttackChancePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::PhysicalDamageAgainstSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32, 0, 0),
            BonusType::MagicalDamageAgainstSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32, 0, 0),
            BonusType::PhysicalDamageAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::MagicalDamageAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::PhysicalDamageAgainstElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32, 0, 0),
            BonusType::DamageAgainstMobGroupPercentage(group, val) => (self.id() as i32, group.value() as i32, *val as i32, 0, 0),
            BonusType::CriticalAgainstRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(class, val) => (self.id() as i32, class.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(race, val) => (self.id() as i32, race.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::PhysicalDamageAgainstClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32, 0, 0),
            BonusType::ResistanceDamageFromClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32, 0, 0),
            BonusType::ResistanceDamageFromElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32, 0, 0),
            BonusType::ResistanceDamageFromRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::ResistanceDamageFromSizePercentage(size, val) => (self.id() as i32, size.value() as i32, *val as i32, 0, 0),
            BonusType::IgnoreDefClass(class) => (self.id() as i32, class.value() as i32, 0, 0, 0),
            BonusType::IgnoreDefRace(race) => (self.id() as i32, race.value() as i32, 0, 0, 0),
            BonusType::IgnoreDefRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::IgnoreMDefRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::IgnoreMDefClassPercentage(class, val) => (self.id() as i32, class.value() as i32, *val as i32, 0, 0),
            BonusType::PhysicalDamageAgainstMobIdPercentage(mob_id, val) => (self.id() as i32, *mob_id as i32, *val as i32, 0, 0),
            BonusType::DamageUsingElementPercentage(element, val) => (self.id() as i32, element.value() as i32, *val as i32, 0, 0),
            BonusType::MasteryDamageUsingWeaponType(weapon_type, val) => (self.id() as i32, weapon_type.value() as i32, *val as i32, 0, 0),
            BonusType::IncreaseDamageAgainstClassBaseOnDef(class) => (self.id() as i32, class.value() as i32, 0, 0, 0),
            BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(status, val) => (self.id() as i32, status.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::ChanceToInflictStatusToPartyOnAttackPercentage(status, val) => (self.id() as i32, status.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::ChanceToInflictStatusOnAttackPercentage(status, val) => (self.id() as i32, status.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::ResistanceToStatusPercentage(status, val) => (self.id() as i32, status.value() as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::GainExpWhenKillingRacePercentage(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::SpDrainWhenAttackingRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::SpDrainWhenKillingRace(race, val) => (self.id() as i32, race.value() as i32, *val as i32, 0, 0),
            BonusType::BreakArmorPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::BreakWeaponPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::BreakSelfWeaponPercentage(val) => (self.id() as i32, (*val * 100.0) as i32, 0, 0, 0),
            BonusType::ClassChangePercentageOnHit(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::LongRangeCriticalChance(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SkillDelayIncDecPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DoubleAttackChancePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HealSkillPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::HealSkillIdPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32, 0, 0),
            BonusType::ResistanceRangeAttackPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DamageRangedAtkPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::ResistanceMagicAttackPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::MagicAttackReflectChancePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::PhysicalAttackReflectChancePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::PhysicalAttackBlockChancePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SplashRadius(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SpeedPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::VisionDistance(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::EnableFullHpSpRecoverOnResurrect => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableSeeHidden => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableNoCancelCast => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableNoGemstoneRequired => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableIgnoreSizeModifier => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableNoKnockback => (self.id() as i32, 0, 0, 0, 0),
            BonusType::EnableNoWalkDelay => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableArmor => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableShoulder => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableHelm => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableShield => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableShoes => (self.id() as i32, 0, 0, 0, 0),
            BonusType::UnbreakableWeapon => (self.id() as i32, 0, 0, 0, 0),
            BonusType::ResistancePhysicalAttackFromMobIdPercentage(mob_id, val) => (self.id() as i32, *mob_id as i32, *val as i32, 0, 0),
            BonusType::DropChanceItemIdPercentage(item_id, val) => (self.id() as i32, *item_id as i32, *val as i32, 0, 0),
            BonusType::DropChanceJewelPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DropChanceOrePercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DropChanceRecoveryPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::DropChanceFoodPercentage(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::KnockbackWhenUsingSkillId(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32, 0, 0),
            BonusType::GainZenyWhenKillingMonster(zeny, chance) => (self.id() as i32, *zeny as i32, *chance as i32, 0, 0),
            BonusType::HpDrainWhenAttackingPercentage(hp_percent, chance) => (self.id() as i32, *hp_percent as i32, *chance as i32, 0, 0),
            BonusType::SpDrainWhenAttackingPercentage(sp_percent, chance) => (self.id() as i32, *sp_percent as i32, *chance as i32, 0, 0),
            BonusType::SpDrainPerHit(val) => (self.id() as i32, *val as i32, 0, 0, 0),
            BonusType::SpBurnOnTargetWhenAttackingPercentage(percent, chance) => (self.id() as i32, *percent as i32, *chance as i32, 0, 0),
            BonusType::HpLossEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32, 0, 0),
            BonusType::HpRegenEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32, 0, 0),
            BonusType::SpLossEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32, 0, 0),
            BonusType::SpRegenEveryMs(amount, ms) => (self.id() as i32, *amount as i32, *ms as i32, 0, 0),
            BonusType::SkillIdDamagePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32, 0, 0),
            BonusType::EnableSkillId(skill_id, level) => (self.id() as i32, *skill_id as i32, *level as i32, 0, 0),
            BonusType::SkillIdSuccessPercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::AutospellSkillIdChancePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, (*val * 100.0) as i32, 0, 0),
            BonusType::DoubleCastSkillIdChancePercentage(skill_id, val) => (self.id() as i32, *skill_id as i32, *val as i32, 0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::bonus::BonusType;
    use crate::enums::element::Element;
    use crate::enums::EnumStackable;

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

        let merged_bonus = BonusType::merge_enums(&bonuses);
        let bonuses_ref: Vec<&BonusType> = bonuses.iter().map(|b| b).collect();
        assert_eq!(*BonusType::get_enum(&BonusType::Str(0), &bonuses_ref).unwrap(), BonusType::Str(13));
        assert_eq!(*BonusType::get_enum(&BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 0), &bonuses_ref).unwrap(), BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, 15));
        assert_eq!(BonusType::get_enum(&BonusType::PhysicalDamageAgainstElementPercentage(Element::Earth, 0), &bonuses_ref), None);
    }
}
