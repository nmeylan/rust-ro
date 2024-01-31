use std::sync::{Arc, RwLock};
use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::compiler::{CompilationDetail, Compiler};
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, NativeMethodHandler, Vm};
use tokio::runtime::Runtime;
use enums::bonus::BonusType;
use enums::element::Element;
use enums::EnumWithNumberValue;
use enums::item::ItemGroup;
use enums::mob::{MobClass, MobRace};
use enums::size::Size;
use enums::status::StatusEffect;
use crate::server::script::constant::load_constant;
use crate::server::script::PlayerScriptHandler;

pub struct BonusScriptHandler {
    bonuses: RwLock<Vec<BonusType>>,
}

macro_rules! bonus {
    ( $self:ident, $bonus:expr ) => {
       $self.bonuses.write().unwrap().push($bonus)
  };
}

impl NativeMethodHandler for BonusScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, _call_frame: &CallFrame, source_line: &CompilationDetail, _class_name: String) {
        if native.name.eq("bonus") {
            self.handle_bonus(params);
        } else if native.name.eq("bonus2") {
            self.handle_bonus2(params);
        } else if native.name.eq("bonus3") {
            self.handle_bonus3(params);
        } else if native.name.eq("bonus4") {
            self.handle_bonus4(params);
        } else if native.name.eq("bonus5") {
            self.handle_bonus5(params);
        } else if native.name.eq("loadconstant") || native.name.eq("getglobalvariable") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
            }
        }
    }
}

impl BonusScriptHandler {
    pub fn handle_bonus(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        if params.len() == 1 {
            match bonus.to_lowercase().as_str() {
                "bintravision" => {
                    bonus!(self, BonusType::EnableSeeHidden);
                }
                "bnocastcancel" => {
                    bonus!(self, BonusType::EnableNoCancelCast);
                }
                "bnogemstone" => {
                    bonus!(self, BonusType::EnableNoGemstoneRequired);
                }
                "bnoknockback" => {
                    bonus!(self, BonusType::EnableNoKnockback);
                }
                "bnosizefix" => {
                    bonus!(self, BonusType::EnableIgnoreSizeModifier);
                }
                "bnowalkdelay" => {
                    bonus!(self, BonusType::EnableNoWalkDelay);
                }
                "brestartfullrecover" => {
                    bonus!(self, BonusType::EnableFullHpSpRecoverOnResurrect);
                }
                "bunbreakablearmor" => {
                    bonus!(self, BonusType::UnbreakableArmor);
                }
                "bunbreakablegarment" => {
                    bonus!(self, BonusType::UnbreakableShoulder);
                }
                "bunbreakablehelm" => {
                    bonus!(self, BonusType::UnbreakableHelm);
                }
                "bunbreakableshield" => {
                    bonus!(self, BonusType::UnbreakableShield);
                }
                "bunbreakableshoes" => {
                    bonus!(self, BonusType::UnbreakableShoes);
                }
                "bunbreakableweapon" => {
                    bonus!(self, BonusType::UnbreakableWeapon);
                }
                _ => {}
            }
        } else {
            let value = params[1].number_value().unwrap() as i8;
            match bonus.to_lowercase().as_str() {
                "bagi" => {
                    bonus!(self, BonusType::Agi(value));
                }
                "badditemhealrate" => {
                    bonus!(self, BonusType::HpRegenFromItemPercentage(value));
                }
                "ballstats" => {
                    bonus!(self, BonusType::AllStats(value));
                }
                "baspd" => {
                    bonus!(self, BonusType::Aspd(value));
                }
                "baspdrate" => {
                    bonus!(self, BonusType::AspdPercentage(value));
                }
                "batkele" => {
                    bonus!(self, BonusType::ElementWeapon(Element::from_value(value as usize)));
                }
                "batkrate" => {
                    bonus!(self, BonusType::AtkPercentage(value));
                }
                "bbaseatk" => {
                    bonus!(self, BonusType::Atk(value));
                }
                "bbreakarmorrate" => {
                    bonus!(self, BonusType::BreakArmorPercentage(value));
                }
                "bbreakweaponrate" => {
                    bonus!(self, BonusType::BreakWeaponPercentage(value));
                }
                "bcastrate" => {
                    bonus!(self, BonusType::CastTimePercentage(value));
                }
                "bclasschange" => {
                    bonus!(self, BonusType::ClassChangePercentageOnHit(value));
                }
                "bcritatkrate" => {
                    bonus!(self, BonusType::CriticalDamagePercentage(value));
                }
                "bcritical" => {
                    bonus!(self, BonusType::CriticalChance(value));
                }
                "bcriticallong" => {
                    bonus!(self, BonusType::LongRangeCriticalChance(value));
                }
                "bdef" => {
                    bonus!(self, BonusType::Def(value));
                }
                "bdef2rate" => {
                    bonus!(self, BonusType::VitDefPercentage(value));
                }
                "bdefele" => {
                    bonus!(self, BonusType::ElementDefense(Element::from_value(value as usize)));
                }
                "bdefrate" => {
                    bonus!(self, BonusType::DefPercentage(value));
                }
                "bdefratioatkclass" => {
                    match MobClass::from_value(value as usize) {
                        MobClass::Boss => bonus!(self, BonusType::IncreaseDamageAgainstClassBossBaseOnDef),
                        MobClass::Normal => bonus!(self, BonusType::IncreaseDamageAgainstClassNormalBaseOnDef),
                        MobClass::Guardian => bonus!(self, BonusType::IncreaseDamageAgainstClassGuardianBaseOnDef),
                        MobClass::All => {
                            bonus!(self, BonusType::IncreaseDamageAgainstClassBossBaseOnDef);
                            bonus!(self, BonusType::IncreaseDamageAgainstClassNormalBaseOnDef);
                            bonus!(self, BonusType::IncreaseDamageAgainstClassGuardianBaseOnDef);
                        }
                    }
                }
                "bdelayrate" => {
                    bonus!(self, BonusType::SkillDelayIncDecPercentage(value));
                }
                "bdex" => {
                    bonus!(self, BonusType::Dex(value));
                }
                "bdoublerate" => {
                    bonus!(self, BonusType::DoubleAttackChancePercentage(value));
                }
                "bflee" => {
                    bonus!(self, BonusType::Flee(value));
                }
                "bflee2" => {
                    bonus!(self, BonusType::PerfectDodge(value));
                }
                "bhpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemy(value));
                }
                "bhprecovrate" => {
                    bonus!(self, BonusType::NaturalHpRecoveryPercentage(value));
                }
                "bhealpower" => {
                    bonus!(self, BonusType::HealSkillPercentage(value));
                }
                "bhealpower2" => {
                    bonus!(self, BonusType::HpRegenFromSkillPercentage(value));
                }
                "bhit" => {
                    bonus!(self, BonusType::Hit(value));
                }
                "bhitrate" => {
                    bonus!(self, BonusType::HitPercentage(value));
                }
                "bignoredefclass" => {
                    match MobClass::from_value(value as usize) {
                        MobClass::Normal => bonus!(self, BonusType::IgnoreDefClassNormal),
                        MobClass::Boss => bonus!(self, BonusType::IgnoreDefClassBoss),
                        MobClass::Guardian => bonus!(self, BonusType::IgnoreDefClassGuardian),
                        MobClass::All => {
                            bonus!(self, BonusType::IgnoreDefClassNormal);
                            bonus!(self, BonusType::IgnoreDefClassBoss);
                            bonus!(self, BonusType::IgnoreDefClassGuardian);
                        }
                    }
                }
                "bignoredefrace" => {
                    match MobRace::from_value(value as usize) {
                        MobRace::Angel => bonus!(self, BonusType::IgnoreDefRaceAngel),
                        MobRace::Brute => bonus!(self, BonusType::IgnoreDefRaceBrute),
                        MobRace::DemiHuman => bonus!(self, BonusType::IgnoreDefRaceDemiHuman),
                        MobRace::Demon => bonus!(self, BonusType::IgnoreDefRaceDemon),
                        MobRace::Dragon => bonus!(self, BonusType::IgnoreDefRaceDragon),
                        MobRace::Fish => bonus!(self, BonusType::IgnoreDefRaceFish),
                        MobRace::Formless => bonus!(self, BonusType::IgnoreDefRaceFormless),
                        MobRace::Insect => bonus!(self, BonusType::IgnoreDefRaceInsect),
                        MobRace::Plant => bonus!(self, BonusType::IgnoreDefRacePlant),
                        MobRace::PlayerHuman => bonus!(self, BonusType::IgnoreDefRacePlayerHuman),
                        MobRace::PlayerDoram => bonus!(self, BonusType::IgnoreDefRacePlayerDoram),
                        MobRace::Undead => bonus!(self, BonusType::IgnoreDefRaceUndead),
                        MobRace::All => {
                            bonus!(self, BonusType::IgnoreDefRaceAngel);
                            bonus!(self, BonusType::IgnoreDefRaceBrute);
                            bonus!(self, BonusType::IgnoreDefRaceDemiHuman);
                            bonus!(self, BonusType::IgnoreDefRaceDemon);
                            bonus!(self, BonusType::IgnoreDefRaceDragon);
                            bonus!(self, BonusType::IgnoreDefRaceFish);
                            bonus!(self, BonusType::IgnoreDefRaceFormless);
                            bonus!(self, BonusType::IgnoreDefRaceInsect);
                            bonus!(self, BonusType::IgnoreDefRacePlant);
                            bonus!(self, BonusType::IgnoreDefRacePlayerHuman);
                            bonus!(self, BonusType::IgnoreDefRacePlayerDoram);
                            bonus!(self, BonusType::IgnoreDefRaceUndead);
                        }
                    }
                }
                "bint" => {
                    bonus!(self, BonusType::Int(value));
                }
                "blongatkdef" => {
                    bonus!(self, BonusType::ResistanceRangeAttack(value));
                }
                "blongatkrate" => {
                    bonus!(self, BonusType::DamageRangedAtkPercentage(value));
                }
                "bluk" => {
                    bonus!(self, BonusType::Luk(value));
                }
                "bmatkrate" => {
                    bonus!(self, BonusType::MatkPercentage(value));
                }
                "bmagicdamagereturn" => {
                    bonus!(self, BonusType::MagicAttackReflectChancePercentage(value));
                }
                "bmagichpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemyWithMagicAttack(value));
                }
                "bmagicspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemyWithMagicAttack(value));
                }
                "bmatk" => {
                    bonus!(self, BonusType::Matk(value));
                }
                "bmaxhp" => {
                    bonus!(self, BonusType::Maxhp(value));
                }
                "bmaxhprate" => {
                    bonus!(self, BonusType::MaxhpPercentage(value));
                }
                "bmaxsp" => {
                    bonus!(self, BonusType::Maxsp(value));
                }
                "bmaxsprate" => {
                    bonus!(self, BonusType::MaxspPercentage(value));
                }
                "bmdef" => {
                    bonus!(self, BonusType::Mdef(value));
                }
                "bnomagicdamage" => {
                    bonus!(self, BonusType::ResistanceMagicAttackPercentage(value));
                }
                "bnoregen" => {
                    if value == 1 {
                        bonus!(self, BonusType::DisableHpRegen);
                    } else if value == 2 {
                        bonus!(self, BonusType::DisableSpRegen);
                    }
                }
                "bperfecthitrate" => {
                    bonus!(self, BonusType::PerfectHitPercentage(value));
                }
                "bspdrainvalue" => {
                    bonus!(self, BonusType::GainSpWhenHittingEnemy(value));
                }
                "bspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemy(value));
                }
                "bsprecovrate" => {
                    bonus!(self, BonusType::NaturalSpRecoveryPercentage(value));
                }
                "bshortweapondamagereturn" => {
                    bonus!(self, BonusType::MeleeAttackReflectChancePercentage(value));
                }
                "bspeedaddrate" | "bspeedrate" => {
                    bonus!(self, BonusType::SpeedPercentage(value));
                }
                "bsplashrange" => {
                    bonus!(self, BonusType::SplashRadius(value));
                }
                "bstr" => {
                    bonus!(self, BonusType::Str(value));
                }
                "busesprate" => {
                    bonus!(self, BonusType::SpConsumption(value));
                }
                "bvit" => {
                    bonus!(self, BonusType::Vit(value));
                }
                _ => {}
            }
        }
    }

    pub fn handle_bonus2(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        let value1 = params[1].number_value().unwrap() as i32;
        let value2 = params[2].number_value().unwrap() as i32;
        match bonus.to_lowercase().as_str() {
            "baddclass" => {
                match MobClass::from_value(value1 as usize) {
                    MobClass::Boss => bonus!(self, BonusType::PhysicalDamageAgainstClassBossPercentage(value2 as i8)),
                    MobClass::Normal => bonus!(self, BonusType::PhysicalDamageAgainstClassNormalPercentage(value2 as i8)),
                    MobClass::Guardian => bonus!(self, BonusType::PhysicalDamageAgainstClassGuardianPercentage(value2 as i8)),
                    MobClass::All => {
                        bonus!(self, BonusType::PhysicalDamageAgainstClassBossPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstClassNormalPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstClassGuardianPercentage(value2 as i8));
                    }
                }
            }
            "badddamageclass" => {
                BonusType::PhysicalDamageAgainstMobIdPercentage(value1 as u32, value2 as i8);
            }
            "badddefmonster" => {
                BonusType::ResistancePhysicalAttackFromMobIdPercentage(value1 as u32, value2 as i8);
            }
            "baddeff" => {
                match StatusEffect::from_value(value1 as usize) {
                    StatusEffect::Poison => bonus!(self, BonusType::ChanceToInflictStatusPoisonOnAttackPercentage(value2 as i8)),
                    StatusEffect::Stun => bonus!(self, BonusType::ChanceToInflictStatusStunOnAttackPercentage(value2 as i8)),
                    StatusEffect::Freeze => bonus!(self, BonusType::ChanceToInflictStatusFreezeOnAttackPercentage(value2 as i8)),
                    StatusEffect::Curse => bonus!(self, BonusType::ChanceToInflictStatusCurseOnAttackPercentage(value2 as i8)),
                    StatusEffect::Blind => bonus!(self, BonusType::ChanceToInflictStatusBlindOnAttackPercentage(value2 as i8)),
                    StatusEffect::Sleep => bonus!(self, BonusType::ChanceToInflictStatusSleepOnAttackPercentage(value2 as i8)),
                    StatusEffect::Silence => bonus!(self, BonusType::ChanceToInflictStatusSilenceOnAttackPercentage(value2 as i8)),
                    StatusEffect::Chaos => bonus!(self, BonusType::ChanceToInflictStatusChaosOnAttackPercentage(value2 as i8)),
                    StatusEffect::Bleeding => bonus!(self, BonusType::ChanceToInflictStatusBleedingOnAttackPercentage(value2 as i8)),
                    StatusEffect::Stone => bonus!(self, BonusType::ChanceToInflictStatusStoneOnAttackPercentage(value2 as i8)),
                    StatusEffect::Burning => bonus!(self, BonusType::ChanceToInflictStatusBurningOnAttackPercentage(value2 as i8)),
                    StatusEffect::Confuse => bonus!(self, BonusType::ChanceToInflictStatusConfuseOnAttackPercentage(value2 as i8)),
                    StatusEffect::WeaponBreak => bonus!(self, BonusType::ChanceToInflictStatusWeaponBreakOnAttackPercentage(value2 as i8)),
                    StatusEffect::ArmorBreak => bonus!(self, BonusType::ChanceToInflictStatusArmorBreakOnAttackPercentage(value2 as i8)),
                    StatusEffect::Coma => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackPercentage(value2 as i8)),
                }
            }
            "baddeff2" => {
                match StatusEffect::from_value(value1 as usize) {
                    StatusEffect::Poison => bonus!(self, BonusType::ChanceToInflictStatusPoisonToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Stun => bonus!(self, BonusType::ChanceToInflictStatusStunToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Freeze => bonus!(self, BonusType::ChanceToInflictStatusFreezeToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Curse => bonus!(self, BonusType::ChanceToInflictStatusCurseToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Blind => bonus!(self, BonusType::ChanceToInflictStatusBlindToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Sleep => bonus!(self, BonusType::ChanceToInflictStatusSleepToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Silence => bonus!(self, BonusType::ChanceToInflictStatusSilenceToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Chaos => bonus!(self, BonusType::ChanceToInflictStatusChaosToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Bleeding => bonus!(self, BonusType::ChanceToInflictStatusBleedingToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Stone => bonus!(self, BonusType::ChanceToInflictStatusStoneToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Burning => bonus!(self, BonusType::ChanceToInflictStatusBurningToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Confuse => bonus!(self, BonusType::ChanceToInflictStatusConfuseToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::WeaponBreak => bonus!(self, BonusType::ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::ArmorBreak => bonus!(self, BonusType::ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage(value2 as i8)),
                    StatusEffect::Coma => bonus!(self, BonusType::ChanceToInflictStatusComaToSelfOnAttackPercentage(value2 as i8)),
                }
            }
            "baddeffwhenhit" => {
                match StatusEffect::from_value(value1 as usize) {
                    StatusEffect::Poison => bonus!(self, BonusType::ChanceToInflictStatusPoisonWhenHitPercentage(value2 as i8)),
                    StatusEffect::Stun => bonus!(self, BonusType::ChanceToInflictStatusStunWhenHitPercentage(value2 as i8)),
                    StatusEffect::Freeze => bonus!(self, BonusType::ChanceToInflictStatusFreezeWhenHitPercentage(value2 as i8)),
                    StatusEffect::Curse => bonus!(self, BonusType::ChanceToInflictStatusCurseWhenHitPercentage(value2 as i8)),
                    StatusEffect::Blind => bonus!(self, BonusType::ChanceToInflictStatusBlindWhenHitPercentage(value2 as i8)),
                    StatusEffect::Sleep => bonus!(self, BonusType::ChanceToInflictStatusSleepWhenHitPercentage(value2 as i8)),
                    StatusEffect::Silence => bonus!(self, BonusType::ChanceToInflictStatusSilenceWhenHitPercentage(value2 as i8)),
                    StatusEffect::Chaos => bonus!(self, BonusType::ChanceToInflictStatusChaosWhenHitPercentage(value2 as i8)),
                    StatusEffect::Bleeding => bonus!(self, BonusType::ChanceToInflictStatusBleedingWhenHitPercentage(value2 as i8)),
                    StatusEffect::Stone => bonus!(self, BonusType::ChanceToInflictStatusStoneWhenHitPercentage(value2 as i8)),
                    StatusEffect::Burning => bonus!(self, BonusType::ChanceToInflictStatusBurningWhenHitPercentage(value2 as i8)),
                    StatusEffect::Confuse => bonus!(self, BonusType::ChanceToInflictStatusConfuseWhenHitPercentage(value2 as i8)),
                    StatusEffect::WeaponBreak => bonus!(self, BonusType::ChanceToInflictStatusWeaponBreakWhenHitPercentage(value2 as i8)),
                    StatusEffect::ArmorBreak => bonus!(self, BonusType::ChanceToInflictStatusArmorBreakWhenHitPercentage(value2 as i8)),
                    StatusEffect::Coma => bonus!(self, BonusType::ChanceToInflictStatusComaWhenHitPercentage(value2 as i8)),
                }
            }
            "baddele" => {
                match Element::from_value(value1 as usize) {
                    Element::Neutral => bonus!(self, BonusType::PhysicalDamageAgainstElementNeutralPercentage(value2 as i8)),
                    Element::Water => bonus!(self, BonusType::PhysicalDamageAgainstElementWaterPercentage(value2 as i8)),
                    Element::Earth => bonus!(self, BonusType::PhysicalDamageAgainstElementEarthPercentage(value2 as i8)),
                    Element::Fire => bonus!(self, BonusType::PhysicalDamageAgainstElementFirePercentage(value2 as i8)),
                    Element::Wind => bonus!(self, BonusType::PhysicalDamageAgainstElementWindPercentage(value2 as i8)),
                    Element::Poison => bonus!(self, BonusType::PhysicalDamageAgainstElementPoisonPercentage(value2 as i8)),
                    Element::Holy => bonus!(self, BonusType::PhysicalDamageAgainstElementHolyPercentage(value2 as i8)),
                    Element::Dark => bonus!(self, BonusType::PhysicalDamageAgainstElementDarkPercentage(value2 as i8)),
                    Element::Ghost => bonus!(self, BonusType::PhysicalDamageAgainstElementGhostPercentage(value2 as i8)),
                    Element::Undead => bonus!(self, BonusType::PhysicalDamageAgainstElementUndeadPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "badditemgrouphealrate" => {
                match ItemGroup::from_value(value1 as usize) {
                    ItemGroup::Herb => bonus!(self, BonusType::HpRegenFromHerbPercentage(value2 as i8)),
                    ItemGroup::Fruit => bonus!(self, BonusType::HpRegenFromFruitPercentage(value2 as i8)),
                    ItemGroup::Meat => bonus!(self, BonusType::HpRegenFromMeatPercentage(value2 as i8)),
                    ItemGroup::Candy => bonus!(self, BonusType::HpRegenFromCandyPercentage(value2 as i8)),
                    ItemGroup::Juice => bonus!(self, BonusType::HpRegenFromJuicePercentage(value2 as i8)),
                    ItemGroup::Fish => bonus!(self, BonusType::HpRegenFromFishPercentage(value2 as i8)),
                    ItemGroup::Food => bonus!(self, BonusType::HpRegenFromFoodPercentage(value2 as i8)),
                    ItemGroup::Potion => bonus!(self, BonusType::HpRegenFromPotionPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "badditemhealrate" => {
                bonus!(self, BonusType::HpRegenFromItemIDPercentage(value1 as u32, value2 as i8));
            }
            "baddmonsterdropitem" => {
                bonus!(self, BonusType::DropChanceItemIdPercentage(value1 as u32, value2 as i8));
            }
            "baddmonsterdropitemgroup" => {
                match ItemGroup::from_value(value1 as usize) {
                    ItemGroup::Ore => bonus!(self, BonusType::DropChanceOrePercentage(value2 as i8)),
                    ItemGroup::Jewel => bonus!(self, BonusType::DropChanceJewelPercentage(value2 as i8)),
                    ItemGroup::Recovery => bonus!(self, BonusType::DropChanceRecoveryPercentage(value2 as i8)),
                    ItemGroup::Food => bonus!(self, BonusType::DropChanceFoodPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "baddrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Angel => bonus!(self, BonusType::PhysicalDamageAgainstRaceAngelPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::PhysicalDamageAgainstRaceBrutePercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::PhysicalDamageAgainstRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::PhysicalDamageAgainstRaceDemonPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::PhysicalDamageAgainstRaceDragonPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::PhysicalDamageAgainstRaceFishPercentage(value2 as i8)),
                    MobRace::Formless => bonus!(self, BonusType::PhysicalDamageAgainstRaceFormlessPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::PhysicalDamageAgainstRaceInsectPercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::PhysicalDamageAgainstRacePlantPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::PhysicalDamageAgainstRaceUndeadPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "baddrace2" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Angel => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceAngelPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceAngelPercentage(value2 as i8));
                    }
                    MobRace::Brute => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceBrutePercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceBrutePercentage(value2 as i8));
                    }
                    MobRace::DemiHuman => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDemiHumanPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceDemiHumanPercentage(value2 as i8));
                    }
                    MobRace::Demon => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDemonPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceDemonPercentage(value2 as i8));
                    }
                    MobRace::Dragon => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDragonPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceDragonPercentage(value2 as i8));
                    }
                    MobRace::Fish => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceFishPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceFishPercentage(value2 as i8));
                    }
                    MobRace::Formless => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceFormlessPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceFormlessPercentage(value2 as i8));
                    }
                    MobRace::Insect => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceInsectPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceInsectPercentage(value2 as i8));
                    }
                    MobRace::Plant => {
                        bonus!(self, BonusType::MagicalDamageAgainstRacePlantPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRacePlantPercentage(value2 as i8));
                    }
                    MobRace::Undead => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceUndeadPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstRaceUndeadPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "baddsize" => {
                match Size::from_value(value1 as usize) {
                    Size::Small => bonus!(self, BonusType::PhysicalDamageAgainstSizeSmallPercentage(value2 as i8)),
                    Size::Medium => bonus!(self, BonusType::PhysicalDamageAgainstSizeMediumPercentage(value2 as i8)),
                    Size::Large => bonus!(self, BonusType::PhysicalDamageAgainstSizeLargePercentage(value2 as i8)),
                    Size::All => {
                        bonus!(self, BonusType::PhysicalDamageAgainstSizeSmallPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstSizeMediumPercentage(value2 as i8));
                        bonus!(self, BonusType::PhysicalDamageAgainstSizeLargePercentage(value2 as i8));
                    }
                }
            }
            "baddskillblow" => {
                bonus!(self, BonusType::KnockbackWhenUsingSkillId(value1 as u32, value2 as i8));
            }
            "bcastrate2" => {
                bonus!(self, BonusType::CastTimeWhenUsingSkillIdPercentage(value1 as u32, value2 as i8))
            }
            "bcomaclass" => {
                match MobClass::from_value(value1 as usize) {
                    MobClass::Boss => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnBossClassPercentage((value2 / 100_i32) as i8)),
                    MobClass::Normal => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnNormalClassPercentage((value2 / 100_i32) as i8)),
                    MobClass::Guardian => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage((value2 / 100_i32) as i8)),
                    MobClass::All => {
                        bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnBossClassPercentage((value2 / 100_i32) as i8));
                        bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnNormalClassPercentage((value2 / 100_i32) as i8));
                        bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage((value2 / 100_i32) as i8));
                    }
                }
            }
            "bcomarace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Angel => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceAngelPercentage((value2 / 100_i32) as i8)),
                    MobRace::Brute => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceBrutePercentage((value2 / 100_i32) as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage((value2 / 100_i32) as i8)),
                    MobRace::Demon => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceDemonPercentage((value2 / 100_i32) as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceDragonPercentage((value2 / 100_i32) as i8)),
                    MobRace::Fish => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceFishPercentage((value2 / 100_i32) as i8)),
                    MobRace::Formless => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceFormlessPercentage((value2 / 100_i32) as i8)),
                    MobRace::Insect => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceInsectPercentage((value2 / 100_i32) as i8)),
                    MobRace::Plant => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRacePlantPercentage((value2 / 100_i32) as i8)),
                    MobRace::Undead => bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackRaceUndeadPercentage((value2 / 100_i32) as i8)),
                    _ => {}
                }
            }
            "bcriticaladdrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Angel => bonus!(self, BonusType::CriticalAgainstRaceAngelPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::CriticalAgainstRaceBrutePercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::CriticalAgainstRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::CriticalAgainstRaceDemonPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::CriticalAgainstRaceDragonPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::CriticalAgainstRaceFishPercentage(value2 as i8)),
                    MobRace::Formless => bonus!(self, BonusType::CriticalAgainstRaceFormlessPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::CriticalAgainstRaceInsectPercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::CriticalAgainstRacePlantPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::CriticalAgainstRaceUndeadPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "bexpaddrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::GainExpWhenKillingRaceFormlessPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::GainExpWhenKillingRaceUndeadPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::GainExpWhenKillingRaceBrutePercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::GainExpWhenKillingRacePlantPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::GainExpWhenKillingRaceInsectPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::GainExpWhenKillingRaceFishPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::GainExpWhenKillingRaceDemonPercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::GainExpWhenKillingRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Angel => bonus!(self, BonusType::GainExpWhenKillingRaceAngelPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::GainExpWhenKillingRaceDragonPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "bgetzenynum" => {
                bonus!(self, BonusType::GainZenyWhenKillingMonster(value1 as u16, value2 as i8));
            }
            "bhpdrainrate" => {
                bonus!(self, BonusType::HpDrainWhenAttackingPercentage(value2 as i8, (value1 / 10_i32) as i8));
            }
            "bhplossrate" => {
                bonus!(self, BonusType::HpLossEveryMs(value1 as u16, value2 as u16));
            }
            "bhpregenrate" => {
                bonus!(self, BonusType::HpRegenEveryMs(value1 as u16, value2 as u16));
            }
            "bignoredefracerate" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::IgnoreDefRaceFormlessPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::IgnoreDefRaceUndeadPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::IgnoreDefRaceBrutePercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::IgnoreDefRacePlantPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::IgnoreDefRaceInsectPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::IgnoreDefRaceFishPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::IgnoreDefRaceDemonPercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::IgnoreDefRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Angel => bonus!(self, BonusType::IgnoreDefRaceAngelPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::IgnoreDefRaceDragonPercentage(value2 as i8)),
                    MobRace::All => {
                        bonus!(self, BonusType::IgnoreDefRaceFormlessPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceUndeadPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceBrutePercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRacePlantPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceInsectPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceFishPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceDemonPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceDemiHumanPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceAngelPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreDefRaceDragonPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "bignoremdefracerate" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::IgnoreMDefRaceFormlessPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::IgnoreMDefRaceUndeadPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::IgnoreMDefRaceBrutePercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::IgnoreMDefRacePlantPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::IgnoreMDefRaceInsectPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::IgnoreMDefRaceFishPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::IgnoreMDefRaceDemonPercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::IgnoreMDefRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Angel => bonus!(self, BonusType::IgnoreMDefRaceAngelPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::IgnoreMDefRaceDragonPercentage(value2 as i8)),
                    MobRace::All => {
                        bonus!(self, BonusType::IgnoreMDefRaceFormlessPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceUndeadPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceBrutePercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRacePlantPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceInsectPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceFishPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceDemonPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceDemiHumanPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceAngelPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefRaceDragonPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "bignoremdefclassrate" => {
                match MobClass::from_value(value1 as usize) {
                    MobClass::Normal => bonus!(self, BonusType::IgnoreMDefClassNormalPercentage(value2 as i8)),
                    MobClass::Boss => bonus!(self, BonusType::IgnoreMDefClassBossPercentage(value2 as i8)),
                    MobClass::Guardian => bonus!(self, BonusType::IgnoreMDefClassGuardianPercentage(value2 as i8)),
                    MobClass::All => {
                        bonus!(self, BonusType::IgnoreMDefClassNormalPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefClassBossPercentage(value2 as i8));
                        bonus!(self, BonusType::IgnoreMDefClassGuardianPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "bmagicaddrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::MagicalDamageAgainstRaceFormlessPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::MagicalDamageAgainstRaceUndeadPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::MagicalDamageAgainstRaceBrutePercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::MagicalDamageAgainstRacePlantPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::MagicalDamageAgainstRaceInsectPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::MagicalDamageAgainstRaceFishPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::MagicalDamageAgainstRaceDemonPercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::MagicalDamageAgainstRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Angel => bonus!(self, BonusType::MagicalDamageAgainstRaceAngelPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::MagicalDamageAgainstRaceDragonPercentage(value2 as i8)),
                    MobRace::All => {
                        bonus!(self, BonusType::MagicalDamageAgainstRaceFormlessPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceUndeadPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceBrutePercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRacePlantPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceInsectPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceFishPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDemonPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDemiHumanPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceAngelPercentage(value2 as i8));
                        bonus!(self, BonusType::MagicalDamageAgainstRaceDragonPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "breseff" => {
                match StatusEffect::from_value(value1 as usize) {
                    StatusEffect::Poison => bonus!(self, BonusType::ResistanceToStatusPoisonPercentage(value2 as i8)),
                    StatusEffect::Bleeding => bonus!(self, BonusType::ResistanceToStatusBleedingPercentage(value2 as i8)),
                    StatusEffect::Blind => bonus!(self, BonusType::ResistanceToStatusBlindPercentage(value2 as i8)),
                    StatusEffect::Burning => bonus!(self, BonusType::ResistanceToStatusBurningPercentage(value2 as i8)),
                    StatusEffect::Confuse => bonus!(self, BonusType::ResistanceToStatusConfusePercentage(value2 as i8)),
                    StatusEffect::Curse => bonus!(self, BonusType::ResistanceToStatusCursePercentage(value2 as i8)),
                    StatusEffect::Freeze => bonus!(self, BonusType::ResistanceToStatusFreezePercentage(value2 as i8)),
                    StatusEffect::Silence => bonus!(self, BonusType::ResistanceToStatusSilencePercentage(value2 as i8)),
                    StatusEffect::Sleep => bonus!(self, BonusType::ResistanceToStatusSleepPercentage(value2 as i8)),
                    StatusEffect::Stone => bonus!(self, BonusType::ResistanceToStatusStonePercentage(value2 as i8)),
                    StatusEffect::Stun => bonus!(self, BonusType::ResistanceToStatusStunPercentage(value2 as i8)),
                    _ => {}
                }
            }
            "bspdrainrate" => {
                bonus!(self, BonusType::SpDrainWhenAttackingPercentage(value2 as i8, (value1 / 10_i32) as i8));
            }
            "bspdrainvaluerace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::SpDrainWhenAttackingRaceFormless(value2 as u16)),
                    MobRace::Undead => bonus!(self, BonusType::SpDrainWhenAttackingRaceUndead(value2 as u16)),
                    MobRace::Brute => bonus!(self, BonusType::SpDrainWhenAttackingRaceBrute(value2 as u16)),
                    MobRace::Plant => bonus!(self, BonusType::SpDrainWhenAttackingRacePlant(value2 as u16)),
                    MobRace::Insect => bonus!(self, BonusType::SpDrainWhenAttackingRaceInsect(value2 as u16)),
                    MobRace::Fish => bonus!(self, BonusType::SpDrainWhenAttackingRaceFish(value2 as u16)),
                    MobRace::Demon => bonus!(self, BonusType::SpDrainWhenAttackingRaceDemon(value2 as u16)),
                    MobRace::DemiHuman => bonus!(self, BonusType::SpDrainWhenAttackingRaceDemiHuman(value2 as u16)),
                    MobRace::Angel => bonus!(self, BonusType::SpDrainWhenAttackingRaceAngel(value2 as u16)),
                    MobRace::Dragon => bonus!(self, BonusType::SpDrainWhenAttackingRaceDragon(value2 as u16)),
                    MobRace::All => {
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceFormless(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceUndead(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceBrute(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRacePlant(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceInsect(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceFish(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceDemon(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceDemiHuman(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceAngel(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenAttackingRaceDragon(value2 as u16));
                    }
                    _ => {}
                }
            }
            "bspgainrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::SpDrainWhenKillingRaceFormless(value2 as u16)),
                    MobRace::Undead => bonus!(self, BonusType::SpDrainWhenKillingRaceUndead(value2 as u16)),
                    MobRace::Brute => bonus!(self, BonusType::SpDrainWhenKillingRaceBrute(value2 as u16)),
                    MobRace::Plant => bonus!(self, BonusType::SpDrainWhenKillingRacePlant(value2 as u16)),
                    MobRace::Insect => bonus!(self, BonusType::SpDrainWhenKillingRaceInsect(value2 as u16)),
                    MobRace::Fish => bonus!(self, BonusType::SpDrainWhenKillingRaceFish(value2 as u16)),
                    MobRace::Demon => bonus!(self, BonusType::SpDrainWhenKillingRaceDemon(value2 as u16)),
                    MobRace::DemiHuman => bonus!(self, BonusType::SpDrainWhenKillingRaceDemiHuman(value2 as u16)),
                    MobRace::Angel => bonus!(self, BonusType::SpDrainWhenKillingRaceAngel(value2 as u16)),
                    MobRace::Dragon => bonus!(self, BonusType::SpDrainWhenKillingRaceDragon(value2 as u16)),
                    MobRace::All => {
                        bonus!(self, BonusType::SpDrainWhenKillingRaceFormless(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceUndead(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceBrute(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRacePlant(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceInsect(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceFish(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceDemon(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceDemiHuman(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceAngel(value2 as u16));
                        bonus!(self, BonusType::SpDrainWhenKillingRaceDragon(value2 as u16));
                    }
                    _ => {}
                }
            }
            "bsplossrate" => {
                bonus!(self, BonusType::SpLossEveryMs(value1 as u16, value2 as u16));
            }
            "bspregenrate" => {
                bonus!(self, BonusType::SpRegenEveryMs(value1 as u16, value2 as u16));
            }
            "bspvanishrate" => {
                bonus!(self, BonusType::SpRegenEveryMs(value2 as u16, (value1 / 10_i32) as u16));
            }
            "bskillatk" => {
                bonus!(self, BonusType::SkillDamagePercentage(value1 as u32, value2 as i8));
            }
            "bskillheal" => {
                bonus!(self, BonusType::HealSkillIdPercentage(value1 as u32, value2 as i8));
            }
            "bsubclass" => {
                match MobClass::from_value(value1 as usize) {
                    MobClass::Boss => bonus!(self, BonusType::ResistanceDamageFromClassBossPercentage(value2 as i8)),
                    MobClass::Normal => bonus!(self, BonusType::ResistanceDamageFromClassNormalPercentage(value2 as i8)),
                    MobClass::Guardian => bonus!(self, BonusType::ResistanceDamageFromClassGuardianPercentage(value2 as i8)),
                    MobClass::All => {
                        bonus!(self, BonusType::ResistanceDamageFromClassBossPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromClassNormalPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromClassGuardianPercentage(value2 as i8));
                    }
                }
            }
            "bsubele" => {
                match Element::from_value(value1 as usize) {
                    Element::Neutral => bonus!(self, BonusType::ResistanceDamageFromElementNeutralPercentage(value2 as i8)),
                    Element::Water => bonus!(self, BonusType::ResistanceDamageFromElementWaterPercentage(value2 as i8)),
                    Element::Earth => bonus!(self, BonusType::ResistanceDamageFromElementEarthPercentage(value2 as i8)),
                    Element::Fire => bonus!(self, BonusType::ResistanceDamageFromElementFirePercentage(value2 as i8)),
                    Element::Wind => bonus!(self, BonusType::ResistanceDamageFromElementWindPercentage(value2 as i8)),
                    Element::Poison => bonus!(self, BonusType::ResistanceDamageFromElementPoisonPercentage(value2 as i8)),
                    Element::Holy => bonus!(self, BonusType::ResistanceDamageFromElementHolyPercentage(value2 as i8)),
                    Element::Dark => bonus!(self, BonusType::ResistanceDamageFromElementDarkPercentage(value2 as i8)),
                    Element::Ghost => bonus!(self, BonusType::ResistanceDamageFromElementGhostPercentage(value2 as i8)),
                    Element::Undead => bonus!(self, BonusType::ResistanceDamageFromElementUndeadPercentage(value2 as i8)),
                    Element::All => {
                        bonus!(self, BonusType::ResistanceDamageFromElementNeutralPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementWaterPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementEarthPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementFirePercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementWindPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementPoisonPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementHolyPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementDarkPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementGhostPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromElementUndeadPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "bsubrace" => {
                match MobRace::from_value(value1 as usize) {
                    MobRace::Formless => bonus!(self, BonusType::ResistanceDamageFromRaceFormlessPercentage(value2 as i8)),
                    MobRace::Undead => bonus!(self, BonusType::ResistanceDamageFromRaceUndeadPercentage(value2 as i8)),
                    MobRace::Brute => bonus!(self, BonusType::ResistanceDamageFromRaceBrutePercentage(value2 as i8)),
                    MobRace::Plant => bonus!(self, BonusType::ResistanceDamageFromRacePlantPercentage(value2 as i8)),
                    MobRace::Insect => bonus!(self, BonusType::ResistanceDamageFromRaceInsectPercentage(value2 as i8)),
                    MobRace::Fish => bonus!(self, BonusType::ResistanceDamageFromRaceFishPercentage(value2 as i8)),
                    MobRace::Demon => bonus!(self, BonusType::ResistanceDamageFromRaceDemonPercentage(value2 as i8)),
                    MobRace::DemiHuman => bonus!(self, BonusType::ResistanceDamageFromRaceDemiHumanPercentage(value2 as i8)),
                    MobRace::Angel => bonus!(self, BonusType::ResistanceDamageFromRaceAngelPercentage(value2 as i8)),
                    MobRace::Dragon => bonus!(self, BonusType::ResistanceDamageFromRaceDragonPercentage(value2 as i8)),
                    MobRace::All => {
                        bonus!(self, BonusType::ResistanceDamageFromRaceFormlessPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceUndeadPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceBrutePercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRacePlantPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceInsectPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceFishPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceDemonPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceDemiHumanPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceAngelPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromRaceDragonPercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            "bsubsize" => {
                match Size::from_value(value1 as usize) {
                    Size::Small => bonus!(self, BonusType::ResistanceDamageFromSizeSmallPercentage(value2 as i8)),
                    Size::Medium => bonus!(self, BonusType::ResistanceDamageFromSizeMediumPercentage(value2 as i8)),
                    Size::Large => bonus!(self, BonusType::ResistanceDamageFromSizeLargePercentage(value2 as i8)),
                    Size::All => {
                        bonus!(self, BonusType::ResistanceDamageFromSizeSmallPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromSizeMediumPercentage(value2 as i8));
                        bonus!(self, BonusType::ResistanceDamageFromSizeLargePercentage(value2 as i8));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    pub fn handle_bonus3(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        match bonus.to_lowercase().as_str() {
            "baddeff" => {}
            "baddeffonskill" => {}
            "baddeffwhenhit" => {}
            "baddmonsterdropitem" => {}
            "baddmonsteriddropitem" => {}
            "bautospell" => {}
            "bautospellwhenhit" => {}
            "bsubele" => {}
            _ => {}
        }
    }
    pub fn handle_bonus4(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        match bonus.to_lowercase().as_str() {
            "bautospell" => {}
            "bautospellonskill" => {}
            "bautospellwhenhit" => {}
            _ => {}
        }
    }
    pub fn handle_bonus5(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        match bonus.to_lowercase().as_str() {
            "bautospell" => {}
            "bautospellonskill" => {}
            "bautospellwhenhit" => {}
            _ => {}
        }
    }
}

#[test]
fn test_simple_bonus() {
    // Given
    let script = "bonus bStr, 10;";
    let compilation_result = Compiler::compile_script(format!("test"), script, "../native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::None.value());
    let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value()));
    // When
    let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]) };
    Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(),
             Box::new(&script_handler));
    // Then
    assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::Str(10)))
}

#[test]
fn test_bonus_with_constant() {
    // Given
    let script = "bonus bAtkEle,Ele_Water;";
    let compilation_result = Compiler::compile_script(format!("test"), script, "../native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::None.value());
    let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::All.value()));
    // When
    let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]) };
    Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(),
             Box::new(&script_handler));
    // Then
    assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::ElementWeapon(Element::Water)))
}