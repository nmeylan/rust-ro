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
use enums::mob::{MobClass, MobRace};
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
                    bonus!(self, BonusType::EnableNoKnockback);}
                "bnosizefix" => {
                    bonus!(self, BonusType::EnableIgnoreSizeModifier);}
                "bnowalkdelay" => {
                    bonus!(self, BonusType::EnableNoWalkDelay);
                }
                "brestartfullrecover" => {
                    bonus!(self, BonusType::EnableFullHpSpRecoverOnResurrect);}
                "bunbreakablearmor" => {
                    bonus!(self, BonusType::UnbreakableArmor);
                }
                "bunbreakablegarment" => {
                    bonus!(self, BonusType::UnbreakableShoulder);}
                "bunbreakablehelm" => {
                    bonus!(self, BonusType::UnbreakableHelm);}
                "bunbreakableshield" => {
                    bonus!(self, BonusType::UnbreakableShield);}
                "bunbreakableshoes" => {
                    bonus!(self, BonusType::UnbreakableShoes);}
                "bunbreakableweapon" => {
                    bonus!(self, BonusType::UnbreakableWeapon);}
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
                        MobClass::All => bonus!(self, BonusType::IgnoreDefClassAll),
                        MobClass::Boss => bonus!(self, BonusType::IgnoreDefClassBoss),
                        MobClass::Guardian => bonus!(self, BonusType::IgnoreDefClassGuardian),
                    }
                }
                "bignoredefrace" => {
                    match MobRace::from_value(value as usize) {
                        MobRace::All => bonus!(self, BonusType::IgnoreDefRaceAll),
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
                    bonus!(self, BonusType::MagicAttackRelectChancePercentage(value));
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
                    bonus!(self, BonusType::MeleeAttackRelectChancePercentage(value));
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
                    bonus!(self, BonusType::SpConsumption(value));}
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