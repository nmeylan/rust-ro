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
use enums::mob::MobClass;
use crate::server::script::constant::load_constant;
use crate::server::script::PlayerScriptHandler;

pub struct BonusScriptHandler {
    bonuses: RwLock<Vec<BonusType>>
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
        }  else if native.name.eq("bonus3") {
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
                "bintravision" => {}
                "bnocastcancel" => {}
                "bnogemstone" => {}
                "bnoknockback" => {}
                "bnosizefix" => {}
                "bnowalkdelay" => {}
                "brestartfullrecover" => {}
                "bunbreakablearmor" => {}
                "bunbreakablegarment" => {}
                "bunbreakablehelm" => {}
                "bunbreakableshield" => {}
                "bunbreakableshoes" => {}
                "bunbreakableweapon" => {}
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
                    bonus!(self, BonusType::AllStats(value));}
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
                    bonus!(self, BonusType::BreakWeaponPercentage(value));}
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
                    bonus!(self, BonusType::IncreaseDecreaseCriticalChance(value));
                }
                "bcriticallong" => {
                    bonus!(self, BonusType::IncreaseDecreaseLongRangeCriticalChance(value));
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
                        MobClass::Boss => bonus!(self, BonusType::IncreaseDamageAgainstBossBaseOnDef),
                        MobClass::All => bonus!(self, BonusType::IncreaseDamageAgainstAllBaseOnDef),
                        MobClass::Normal => bonus!(self, BonusType::IncreaseDamageAgainstNormalBaseOnDef),
                        MobClass::Guardian => bonus!(self, BonusType::IncreaseDamageAgainstGuardianBaseOnDef),
                    }
                }
                "bdelayrate" => {}
                "bdex" => {
                    bonus!(self, BonusType::Dex(value));
                }
                "bdoublerate" => {}
                "bflee" => {}
                "bflee2" => {}
                "bhpdrainrate" => {}
                "bhpgainvalue" => {}
                "bhprecovrate" => {}
                "bhealpower" => {}
                "bhealpower2" => {}
                "bhit" => {}
                "bhitrate" => {}
                "bignoredefclass" => {}
                "bignoredefrace" => {}
                "bint" => {}
                "blongatkdef" => {}
                "blongatkrate" => {}
                "bluk" => {}
                "bmatkrate" => {}
                "bmagicdamagereturn" => {}
                "bmagichpgainvalue" => {}
                "bmagicspgainvalue" => {}
                "bmatk" => {}
                "bmaxhp" => {}
                "bmaxhprate" => {}
                "bmaxsp" => {}
                "bmaxsprate" => {}
                "bmdef" => {}
                "bnomagicdamage" => {}
                "bnoregen" => {}
                "bperfecthitrate" => {}
                "brestartfullrecover" => {}
                "bspdrainvalue" => {}
                "bspgainvalue" => {}
                "bsprecovrate" => {}
                "bshortweapondamagereturn" => {}
                "bspeedaddrate" => {}
                "bspeedrate" => {}
                "bsplashrange" => {}
                "bstr" => {
                    bonus!(self, BonusType::Str(value));
                }
                "busesprate" => {}
                "bvit" => {
                    bonus!(self, BonusType::Vit(value));
                }
                _ => {}
            }
        }
    }

    pub fn handle_bonus2(&self, params: Vec<Value>) {
        let bonus = params[0].string_value().unwrap();
        match bonus.to_lowercase().as_str() {
            "baddclass" => {}
            "badddamageclass" => {}
            "badddefmonster" => {}
            "baddeff" => {}
            "baddeff2" => {}
            "baddeffwhenhit" => {}
            "baddele" => {}
            "badditemgrouphealrate" => {}
            "badditemhealrate" => {}
            "baddmonsterdropitem" => {}
            "baddmonsterdropitemgroup" => {}
            "baddrace" => {}
            "baddrace2" => {}
            "baddsize" => {}
            "baddskillblow" => {}
            "bcastrate" => {}
            "bcomaclass" => {}
            "bcomarace" => {}
            "bcriticaladdrace" => {}
            "bexpaddclass" => {}
            "bexpaddrace" => {}
            "bgetzenynum" => {}
            "bhpdrainrate" => {}
            "bhplossrate" => {}
            "bhpregenrate" => {}
            "bignoredefracerate" => {}
            "bignoremdefclassrate" => {}
            "bignoremdefracerate" => {}
            "bmagicaddrace" => {}
            "breseff" => {}
            "bspdrainrate" => {}
            "bspdrainvaluerace" => {}
            "bspgainrace" => {}
            "bsplossrate" => {}
            "bspregenrate" => {}
            "bspvanishrate" => {}
            "bskillatk" => {}
            "bskillheal" => {}
            "bsubclass" => {}
            "bsubele" => {}
            "bsubrace" => {}
            "bsubrace2" => {}
            "bsubsize" => {}
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
    let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]), };
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
    let script_handler = BonusScriptHandler { bonuses: RwLock::new(vec![]), };
    Vm::repl(vm.clone(), compilation_result.unwrap().pop().as_ref().unwrap(),
             Box::new(&script_handler));
    // Then
    assert!(matches!(script_handler.bonuses.read().unwrap()[0], BonusType::ElementWeapon(Element::Water)))
}