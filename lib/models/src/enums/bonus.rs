use std::fmt::Debug;
use enum_macro::{WithEq, WithStackable};
use crate::enums::EnumStackable;
use crate::enums::element::Element;
use crate::enums::mob::{MobClass, MobGroup, MobRace};
use crate::enums::size::Size;
use crate::enums::status::StatusEffect;
use crate::enums::weapon::WeaponType;
use crate::status::{StatusSnapshot};

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
            _ => { /* TODO */}
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
