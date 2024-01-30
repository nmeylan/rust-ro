use std::sync::{Arc, RwLock};
use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::compiler::{CompilationDetail, Compiler};
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, NativeMethodHandler, Vm};
use tokio::runtime::Runtime;
use enums::bonus::BonusType;
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
    fn handle(&self, native: &Native, params: Vec<Value>, _program: &Thread, _call_frame: &CallFrame, source_line: &CompilationDetail, _class_name: String) {
        if native.name.eq("bonus") {
            let bonus = params[0].string_value().unwrap();
            if params.len() == 1 {
                match bonus.as_str() {
                     "bIntravision" => {}
                     "bNoCastCancel" => {}
                     "bNoGemStone" => {}
                     "bNoKnockback" => {}
                     "bNoSizeFix" => {}
                     "bNoWalkDelay" => {}
                     "bRestartFullRecover" => {}
                     "bUnbreakableArmor" => {}
                     "bUnbreakableGarment" => {}
                     "bUnbreakableHelm" => {}
                     "bUnbreakableShield" => {}
                     "bUnbreakableShoes" => {}
                     "bUnbreakableWeapon" => {}
                    _ => {}
                }
            } else {
                let value = params[1].number_value().unwrap() as i8;
                match bonus.as_str() {
                    "bAGI" => {}
                    "bAddItemHealRate" => {}
                    "bAgi" => {}
                    "bAllStats" => {}
                    "bAspd" => {}
                    "bAspdRate" => {}
                    "bAtkEle" => {}
                    "bAtkRate" => {}
                    "bBaseAtk" => {}
                    "bBreakArmorRate" => {}
                    "bBreakWeaponRate" => {}
                    "bCastRate" => {}
                    "bCastrate" => {}
                    "bClassChange" => {}
                    "bCritAtkRate" => {}
                    "bCritical" => {}
                    "bCriticalLong" => {}
                    "bDef" => {}
                    "bDef2Rate" => {}
                    "bDefEle" => {}
                    "bDefRate" => {}
                    "bDefRatioAtkClass" => {}
                    "bDelayRate" => {}
                    "bDex" => {}
                    "bDoubleRate" => {}
                    "bFlee" => {}
                    "bFlee2" => {}
                    "bHPDrainRate" => {}
                    "bHPGainValue" => {}
                    "bHPrecovRate" => {}
                    "bHealPower" => {}
                    "bHealpower2" => {}
                    "bHit" => {}
                    "bHitRate" => {}
                    "bIgnoreDefClass" => {}
                    "bIgnoreDefRace" => {}
                    "bInt" => {}
                    "bLongAtkDef" => {}
                    "bLongAtkRate" => {}
                    "bLuk" => {}
                    "bMAtkRate" => {}
                    "bMagicDamageReturn" => {}
                    "bMagicHPGainValue" => {}
                    "bMagicSPGainValue" => {}
                    "bMatk" => {}
                    "bMatkRate" => {}
                    "bMatkrate" => {}
                    "bMaxHP" => {}
                    "bMaxHPRate" => {}
                    "bMaxHPrate" => {}
                    "bMaxSP" => {}
                    "bMaxSPRate" => {}
                    "bMaxSPrate" => {}
                    "bMdef" => {}
                    "bNoMagicDamage" => {}
                    "bNoRegen" => {}
                    "bPerfectHitRate" => {}
                    "bRestartFullRecover" => {}
                    "bSPDrainValue" => {}
                    "bSPGainValue" => {}
                    "bSPrecovRate" => {}
                    "bShortWeaponDamageReturn" => {}
                    "bSpeedAddRate" => {}
                    "bSpeedRate" => {}
                    "bSplashRange" => {}
                    "bStr" => {
                        bonus!(self, BonusType::Str(value));
                    }
                    "bUseSPrate" => {}
                    "bVit" => {}
                    "bdex" => {}
                    "buseSPRate" => {}
                    _ => {}
                }
            }
        } else if native.name.eq("bonus2") {
            let value = params[0].string_value().unwrap();
            match value.as_str() {
                "bAddClass" => {}
                "bAddDamageClass" => {}
                "bAddDefMonster" => {}
                "bAddEff" => {}
                "bAddEff2" => {}
                "bAddEffWhenHit" => {}
                "bAddEle" => {}
                "bAddItemGroupHealRate" => {}
                "bAddItemHealRate" => {}
                "bAddMonsterDropItem" => {}
                "bAddMonsterDropItemGroup" => {}
                "bAddRace" => {}
                "bAddRace2" => {}
                "bAddSize" => {}
                "bAddSkillBlow" => {}
                "bCastrate" => {}
                "bComaClass" => {}
                "bComaRace" => {}
                "bCriticalAddRace" => {}
                "bExpAddClass" => {}
                "bExpAddRace" => {}
                "bGetZenyNum" => {}
                "bHPDrainRate" => {}
                "bHPLossRate" => {}
                "bHPRegenRate" => {}
                "bIgnoreDefRaceRate" => {}
                "bIgnoreMdefClassRate" => {}
                "bIgnoreMdefRaceRate" => {}
                "bMagicAddRace" => {}
                "bResEff" => {}
                "bSPDrainRate" => {}
                "bSPDrainValueRace" => {}
                "bSPGainRace" => {}
                "bSPLossRate" => {}
                "bSPRegenRate" => {}
                "bSPVanishRate" => {}
                "bSkillAtk" => {}
                "bSkillHeal" => {}
                "bSubClass" => {}
                "bSubEle" => {}
                "bSubRace" => {}
                "bSubRace2" => {}
                "bSubSize" => {}
                _ => {}
            }
        }  else if native.name.eq("bonus3") {

            match "" {
                "bAddEff" => {}
                "bAddEffOnSkill" => {}
                "bAddEffWhenHit" => {}
                "bAddMonsterDropItem" => {}
                "bAddMonsterIdDropItem" => {}
                "bAutoSpell" => {}
                "bAutoSpellWhenHit" => {}
                "bAutoSpellwhenhit" => {}
                "bSubEle" => {}
                _ => {}
            }

        } else if native.name.eq("bonus4") {

            match "" {
                "bAutoSpell" => {}
                "bAutoSpellOnSkill" => {}
                "bAutoSpellWhenHit" => {}
                "bautospellonskill" => {}
                _ => {}
            }

        } else if native.name.eq("bonus5") {

            match "" {
                "bAutoSpell" => {}
                "bAutoSpellOnSkill" => {}
                "bAutoSpellWhenHit" => {}
                "bautospellonskill" => {}
                _ => {}
            }
        }
    }
}

#[test]
fn test_bonus() {
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