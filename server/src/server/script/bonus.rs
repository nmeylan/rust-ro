use std::sync::{RwLock};

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::compiler::{CompilationDetail};
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::{NativeMethodHandler};

use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::EnumWithNumberValue;
use models::enums::item::ItemGroup;
use models::enums::mob::{MobClass, MobGroup, MobRace};
use models::enums::size::Size;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusEffect;

use crate::server::script::constant::load_constant;


pub struct BonusScriptHandler {
    pub(crate) bonuses: RwLock<Vec<BonusType>>,
}

#[macro_export]
macro_rules! bonus {
    ( $self:ident, $bonus:expr ) => {
       $self.bonuses.write().unwrap().push($bonus)
  };
}

impl NativeMethodHandler for BonusScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, _call_frame: &CallFrame, _source_line: &CompilationDetail, _class_name: String) {
        if native.name.eq("skill") {
            let skill_name = params[0].string_value().unwrap();
            let skill = SkillEnum::from_name(skill_name.as_str());
            let skill_level = params[1].number_value().unwrap() as u8;
            bonus!(self, BonusType::EnableSkillId(skill.id(), skill_level))
        } else if native.name.eq("bonus") {
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
        } else {
         //   println!("Function {} is not implemented", native.name);
        }
    }
}

impl BonusScriptHandler {
    pub fn new() -> Self {
        Self {
            bonuses: Default::default(),
        }
    }
    pub fn drain(&self) -> Vec<BonusType> {
        let mut write_guard = self.bonuses.write().unwrap();
        write_guard.drain(0..).collect()
    }
    pub fn clear(&self) {
        let mut write_guard = self.bonuses.write().unwrap();
        write_guard.clear();
    }
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
            let value = params[1].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "bagi" => {
                    bonus!(self, BonusType::Agi(value as i8));
                }
                "badditemhealrate" => {
                    bonus!(self, BonusType::HpRegenFromItemPercentage(value as i8));
                }
                "ballstats" => {
                    bonus!(self, BonusType::AllStats(value as i8));
                }
                "baspd" => {
                    bonus!(self, BonusType::Aspd(value as i8));
                }
                "baspdrate" => {
                    bonus!(self, BonusType::AspdPercentage(value as i8));
                }
                "batkele" => {
                    bonus!(self, BonusType::ElementWeapon(Element::from_value(value as usize)));
                }
                "batkrate" => {
                    bonus!(self, BonusType::AtkPercentage(value as i8));
                }
                "bbaseatk" => {
                    bonus!(self, BonusType::Atk(value as i16));
                }
                "bbreakarmorrate" => {
                    bonus!(self, BonusType::BreakArmorPercentage((value / 100) as i8));
                }
                "bbreakweaponrate" => {
                    bonus!(self, BonusType::BreakWeaponPercentage((value / 100) as i8));
                }
                "bcastrate" => {
                    bonus!(self, BonusType::CastTimePercentage(value as i8));
                }
                "bclasschange" => {
                    bonus!(self, BonusType::ClassChangePercentageOnHit((value / 100) as i8));
                }
                "bcritatkrate" => {
                    bonus!(self, BonusType::CriticalDamagePercentage(value as i8));
                }
                "bcritical" => {
                    bonus!(self, BonusType::Crit(value as i8));
                }
                "bcriticallong" => {
                    bonus!(self, BonusType::LongRangeCriticalChance(value as i8));
                }
                "bdef" => {
                    bonus!(self, BonusType::Def(value as i16));
                }
                "bdef2rate" => {
                    bonus!(self, BonusType::VitDefPercentage(value as i8));
                }
                "bdefele" => {
                    bonus!(self, BonusType::ElementDefense(Element::from_value(value as usize)));
                }
                "bdefrate" => {
                    bonus!(self, BonusType::DefPercentage(value as i8));
                }
                "bdefratioatkclass" => {
                    bonus!(self, BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::from_value(value as usize)))
                }
                "bdelayrate" => {
                    bonus!(self, BonusType::SkillDelayIncDecPercentage(value as i8));
                }
                "bdex" => {
                    bonus!(self, BonusType::Dex(value as i8));
                }
                "bdoublerate" => {
                    bonus!(self, BonusType::DoubleAttackChancePercentage(value as i8));
                }
                "bflee" => {
                    bonus!(self, BonusType::Flee(value as i16));
                }
                "bflee2" => {
                    bonus!(self, BonusType::PerfectDodge(value as i8));
                }
                "bhpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemy(value as i8));
                }
                "bhprecovrate" => {
                    bonus!(self, BonusType::NaturalHpRecoveryPercentage(value as i8));
                }
                "bhealpower" => {
                    bonus!(self, BonusType::HealSkillPercentage(value as i8));
                }
                "bhealpower2" => {
                    bonus!(self, BonusType::HpRegenFromSkillPercentage(value as i8));
                }
                "bhit" => {
                    bonus!(self, BonusType::Hit(value as i16));
                }
                "bhitrate" => {
                    bonus!(self, BonusType::HitPercentage(value as i8));
                }
                "bignoredefclass" => {
                    bonus!(self, BonusType::IgnoreDefClass(MobClass::from_value(value as usize)))
                }
                "bignoredefrace" => {
                    bonus!(self, BonusType::IgnoreDefRace(MobRace::from_value(value as usize)))
                }
                "bint" => {
                    bonus!(self, BonusType::Int(value as i8));
                }
                "blongatkdef" => {
                    bonus!(self, BonusType::ResistanceRangeAttackPercentage(value as i8));
                }
                "blongatkrate" => {
                    bonus!(self, BonusType::DamageRangedAtkPercentage(value as i8));
                }
                "bluk" => {
                    bonus!(self, BonusType::Luk(value as i8));
                }
                "bmatkrate" => {
                    bonus!(self, BonusType::MatkPercentage(value as i8));
                }
                "bmagicdamagereturn" => {
                    bonus!(self, BonusType::MagicAttackReflectChancePercentage(value as i8));
                }
                "bmagichpgainvalue" => {
                    bonus!(self, BonusType::GainHpWhenKillingEnemyWithMagicAttack(value as i8));
                }
                "bmagicspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemyWithMagicAttack(value as i8));
                }
                "bmatk" => {
                    bonus!(self, BonusType::Matk(value as i16));
                }
                "bmaxhp" => {
                    bonus!(self, BonusType::Maxhp(value));
                }
                "bmaxhprate" => {
                    bonus!(self, BonusType::MaxhpPercentage(value as i8));
                }
                "bmaxsp" => {
                    bonus!(self, BonusType::Maxsp(value));
                }
                "bmaxsprate" => {
                    bonus!(self, BonusType::MaxspPercentage(value as i8));
                }
                "bmdef" => {
                    bonus!(self, BonusType::Mdef(value as i16));
                }
                "bnomagicdamage" => {
                    bonus!(self, BonusType::ResistanceMagicAttackPercentage(value as i8));
                }
                "bnoregen" => {
                    if value == 1 {
                        bonus!(self, BonusType::DisableHpRegen);
                    } else if value == 2 {
                        bonus!(self, BonusType::DisableSpRegen);
                    }
                }
                "bperfecthitrate" => {
                    bonus!(self, BonusType::PerfectHitPercentage(value as i8));
                }
                "bspdrainvalue" => {
                    bonus!(self, BonusType::GainSpWhenHittingEnemy(value as i8));
                }
                "bspgainvalue" => {
                    bonus!(self, BonusType::GainSpWhenKillingEnemy(value as i8));
                }
                "bsprecovrate" => {
                    bonus!(self, BonusType::NaturalSpRecoveryPercentage(value as i8));
                }
                "bshortweapondamagereturn" => {
                    bonus!(self, BonusType::MeleeAttackReflectChancePercentage(value as i8));
                }
                "bspeedaddrate" | "bspeedrate" => {
                    bonus!(self, BonusType::SpeedPercentage(value as i8));
                }
                "bsplashrange" => {
                    bonus!(self, BonusType::SplashRadius(value as i8));
                }
                "bstr" => {
                    bonus!(self, BonusType::Str(value as i8));
                }
                "busesprate" => {
                    bonus!(self, BonusType::SpConsumption(value as i8));
                }
                "bvit" => {
                    bonus!(self, BonusType::Vit(value as i8));
                }
                _ => {}
            }
        }
    }

    pub fn handle_bonus2(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        if params[1].is_string() {
            let value1 = params[1].string_value().unwrap();
            let value2 = params[2].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "bskillatk" => {
                    bonus!(self, BonusType::SkillIdDamagePercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8));
                }
                "bskillheal" => {
                    bonus!(self, BonusType::HealSkillIdPercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8));
                }
                "baddskillblow" => {
                    bonus!(self, BonusType::KnockbackWhenUsingSkillId(SkillEnum::from_name(value1.as_str()).id(), value2 as i8));
                }
                "bcastrate2" => {
                    bonus!(self, BonusType::CastTimeWhenUsingSkillIdPercentage(SkillEnum::from_name(value1.as_str()).id(), value2 as i8))
                }
                _ => {}
            }
        } else {
            let value1 = params[1].number_value().unwrap();
            let value2 = params[2].number_value().unwrap();
            match bonus.to_lowercase().as_str() {
                "baddclass" => {
                    bonus!(self, BonusType::PhysicalDamageAgainstClassPercentage(MobClass::from_value(value1 as usize), value2 as i8))
                }
                "badddamageclass" => {
                    BonusType::PhysicalDamageAgainstMobIdPercentage(value1 as u32, value2 as i8);
                }
                "badddefmonster" => {
                    BonusType::ResistancePhysicalAttackFromMobIdPercentage(value1 as u32, value2 as i8);
                }
                "baddeff" => {
                    bonus!(self, BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::from_value(value1 as usize), value2 as f32 / 100.0))
                }
                "baddeff2" => {
                   bonus!(self, BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::from_value(value1 as usize), value2 as f32 / 100.0))
                }
                "baddeffwhenhit" => {
                    bonus!(self, BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::from_value(value1 as usize), value2 as f32 / 100.0))
                }
                "baddele" => {
                    bonus!(self, BonusType::PhysicalDamageAgainstElementPercentage(Element::from_value(value1 as usize), value2 as i8))
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
                    bonus!(self, BonusType::PhysicalDamageAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "baddrace2" => {
                    bonus!(self, BonusType::DamageAgainstMobGroupPercentage(MobGroup::from_value(value1 as usize), value2 as i8))
                }
                "baddsize" => {
                    bonus!(self, BonusType::PhysicalDamageAgainstSizePercentage(Size::from_value(value1 as usize), value2 as i8))
                }
                "bcomaclass" => {
                    bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(MobClass::from_value(value1 as usize), value2 as f32 / 100.0))
                }
                "bcomarace" => {
                    bonus!(self, BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::from_value(value1 as usize),  value2 as f32 / 100.0))
                }
                "bcriticaladdrace" => {
                    bonus!(self, BonusType::CriticalAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "bexpaddrace" => {
                    bonus!(self, BonusType::GainExpWhenKillingRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
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
                    bonus!(self, BonusType::IgnoreDefRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "bignoremdefracerate" => {
                    bonus!(self, BonusType::IgnoreMDefRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "bignoremdefclassrate" => {
                    bonus!(self, BonusType::IgnoreMDefClassPercentage(MobClass::from_value(value1 as usize), value2 as i8))
                }
                "bmagicaddrace" => {
                    bonus!(self, BonusType::MagicalDamageAgainstRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "breseff" => {
                    bonus!(self, BonusType::ResistanceToStatusPercentage(StatusEffect::from_value(value1 as usize), (value2 / 100) as f32))
                }
                "bspdrainrate" => {
                    bonus!(self, BonusType::SpDrainWhenAttackingPercentage(value2 as i8, (value1 / 10_i32) as i8));
                }
                "bspdrainvaluerace" => {
                    bonus!(self, BonusType::SpDrainWhenAttackingRace(MobRace::from_value(value1 as usize), value2 as u16))
                }
                "bspgainrace" => {
                    bonus!(self, BonusType::SpDrainWhenKillingRace(MobRace::from_value(value1 as usize), value2 as u16))
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
                "bsubclass" => {
                    bonus!(self, BonusType::ResistanceDamageFromClassPercentage(MobClass::from_value(value1 as usize), value2 as i8))
                }
                "bsubele" => {
                    bonus!(self, BonusType::ResistanceDamageFromElementPercentage(Element::from_value(value1 as usize), value2 as i8))
                }
                "bsubrace" => {
                    bonus!(self, BonusType::ResistanceDamageFromRacePercentage(MobRace::from_value(value1 as usize), value2 as i8))
                }
                "bsubsize" => {
                    bonus!(self, BonusType::ResistanceDamageFromSizePercentage(Size::from_value(value1 as usize), value2 as i8))
                }
                _ => {}
            }
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use rathena_script_lang_interpreter::lang::compiler::Compiler;
    use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
    use models::enums::bonus::BonusType;
    use models::enums::element::Element;
    use crate::server::script::bonus::BonusScriptHandler;

    #[test]
    fn test_simple_bonus() {
        // Given
        let script = "bonus bStr, 10;";
        let compilation_result = Compiler::compile_script("test".to_string(), script, "../native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::None.value());
        let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value()));
        // When
        let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]) };
        Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(), Box::new(&script_handler), vec![]);
        // Then
        assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::Str(10)))
    }

    #[test]
    fn test_simple_bonus_negative() {
        // Given
        let script = "bonus bStr, -10;";
        let compilation_result = Compiler::compile_script("test".to_string(), script, "../native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::All.value());
        let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::All.value()));
        // When
        let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]) };
        Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(), Box::new(&script_handler), vec![]);
        // Then
        assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::Str(-10)))
    }

    #[test]
    fn test_bonus_with_constant() {
        // Given
        let script = "bonus bAtkEle,Ele_Water;";
        let compilation_result = Compiler::compile_script("test".to_string(), script, "../native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::None.value());
        let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value()));
        // When
        let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]) };
        Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(), Box::new(&script_handler), vec![]);
        // Then
        assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::ElementWeapon(Element::Water)))
    }
}