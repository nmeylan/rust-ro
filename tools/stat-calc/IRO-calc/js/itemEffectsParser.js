window.parseItemEffects = (function() {

  function sumKeys(current_key, value) {
    if (typeof value === 'number') {
      return parseInt(current_key);
    } else {
      for (var key in value) {
        return parseInt(current_key) + sumKeys(key, value[key]);
      }
    }
  }

  function deepExtractValue(value) {
    if (typeof value === 'number') {
      return value;
    } else {
      for (var key in value) {
        return value[key];
      }
    }
  }

  function sumKeysValue(key, object) {
    return {
      key: sumKeys(key, object),
      value: deepExtractValue(object)
    };
  }

  var keysRemapList = {
    "bMaxHP" : bon_HP_ADD,
    "bMaxSP" : bon_SP_ADD,
    "bStr" : bon_STR,
    "bAgi" : bon_AGI,
    "bVit" : bon_VIT,
    "bInt" : bon_INT,
    "bDex" : bon_DEX,
    "bLuk" : bon_LUK,
    "bAtk" : bon_ATK,
    "bAtk2" : bon_ATK,
    "bDef" : bon_DEF,
    "bDef2" : bon_DEF,
    "bMdef" : bon_MDEF,
    "bMdef2" : bon_MDEF,
    "bHit" : bon_HIT,
    "bFlee" : bon_FLEE,
    "bFlee2" : bon_PDODGE,
    "bCritical" : bon_CRIT,
    "bAspd" : 99999, //Check line 3281 roformulas.js
    "bFame" : 99999,
    "bUnbreakable" : 99999,
    "bAtkRange" : 99999,
    "bAtkEle" : bon_ELEMENT,
    "bDefEle" : bon_USR_ELEMENT,
    "bCastrate" : bon_RED_CAST,
    "bVariableCastrate" : bon_RED_CAST,
    "bMaxHPrate" : bon_HP_MUL,
    "bMaxSPrate" : bon_SP_MUL,
    "bUseSPrate" : 99999,
    "bAddEle" : function(value) { return sumKeysValue(bon_DMG_ELE_NEUTRAL, value); },
    "bAddRace" : function(value) { return sumKeysValue(bon_DMG_RC_FORMLESS, value); },
    "bAddSize" : function(value) { return sumKeysValue(bon_DMG_SIZ_SMALL, value); },
    "bSubEle" : function(value) { return sumKeysValue(bon_RED_ELE_NEUTRAL, value); },
    "bSubRace" : function(value) { return sumKeysValue(bon_RED_RC_FORMLESS, value); },
    "bAddEff" : function(value) { return sumKeysValue(bon_CH_STATUS_POISON, value); },
    "bResEff" : function(value) { return sumKeysValue(bon_RES_STATUS_POISON, value); },
    "bBaseAtk" : bon_ATK,
    "bAspdRate" : bon_ASPD_MUL,
    "bHPrecovRate" : 99999,
    "bSPrecovRate" : 99999,
    "bSpeedRate" : 99999,
    "bCriticalDef" : 99999,
    "bNearAtkDef" : 99999,
    "bLongAtkDef" : bon_RED_RANGE,
    "bDoubleRate" : 99999, // Double Attack with all skills, check 511 skillFormulas.js
    "bDoubleAddRate" : 99999,
    "bSkillHeal" : bon_HEAL_MUL, // add also bon_SANC_MUL
    "bMatkRate" : bon_MATK_MUL,
    "bMatk" : bon_MATK,
    "bLongAtkRate" : bon_DMG_RANGE,
    "bCritAtkRate" : bon_DMG_CRIT,
    "bIgnoreDefEle" : 99999,
    "bIgnoreDefRace" : function(value) { return sumKeysValue(bon_IGN_DEF_RC_FORMLESS, value); },
    "bAtkRate" : bon_PHY_ATK, // bAddRace BOSS e non-boss
    "bSpeedAddRate" : 99999,
    "bSPRegenRate" : 99999,
    "bMagicAtkDef" : 99999,
    "bMiscAtkDef" : 99999,
    "bIgnoreMdefEle" : 99999,
    "bIgnoreMdefRace" : 99999, // Vesper e High Wizard card
    "bMagicAddEle" : 99999, //Engkanto card
    "bMagicAddRace" : function(value) { return sumKeysValue(bon_MDMG_RC_FORMLESS, value); },
    "bMagicAddSize" : 99999, //Sealed Drake card
    "bPerfectHitRate" : bon_CH_GUIDE_ATK,
    "bPerfectHitAddRate" : 99999,
    "bCriticalRate" : bon_CRIT,
    "bGetZenyNum" : 99999,
    "bAddGetZenyNum" : 99999,
    "bAddDamageClass" : function(value) { return 1000; /* sumKeysValue(1000, value); */ },
    "bAddMagicDamageClass" : 99999,
    "bAddDefMonster" : function(value) { return 3000; /* sumKeysValue(3000, value); */ },
    "bAddMdefMonster" : 99999,
    "bAddMonsterDropItem" : 99999,
    "bDefRatioAtkEle" : 99999,
    "bDefRatioAtkRace" : 99999,
    "bUnbreakableGarment" : 99999,
    "bHitRate" : 99999,
    "bFleeRate" : 99999,
    'bFlee2Rate' : 99999,
    "bDefRate" : bon_USR_DEF_DIV,
    "bDef2Rate" : bon_USR_DEF_DIV,
    "bMdefRate" : 99999, // Tao Gunka card
    "bMdef2Rate" : 99999,
    "bSplashRange" : 99999,
    "bSplashAddRange" : 99999,
    "bAutoSpell" : 99999,
    "bHPDrainRate" : 99999,
    "bSPDrainRate" : 99999,
    "bShortWeaponDamageReturn" : bon_REFLECT_PHY_DMG,
    "bLongWeaponDamageReturn" : 99999,
    "bWeaponComaEle" : 99999,
    "bWeaponComaRace" : 99999,
    "bAddEff2" : 99999,
    "bBreakWeaponRate" : 99999,
    "bBreakArmorRate" : 99999,
    "bAddStealRate" : 99999,
    "bMagicDamageReturn" : 99999,
    "bAllStats" : bon_ALL_STATS,
    "bAgiVit" : 99999,
    "bAgiDexStr" : 99999,
    "bPerfectHide" : 99999,
    "bNoKnockback" : 99999,
    "bClassChange" : 99999,
    "bHPDrainValue" : 99999,
    "bSPDrainValue" : 99999,
    "bWeaponAtk" : 99999,
    "bWeaponAtkRate" : 99999,
    "bDelayrate" : function(value) { return { key: bon_RED_CASTDELAY, value: value } }, // Discordi (il delay è ridotto se bDelayrate è - quindi bon_RED_CASTDELAY è +)
    "bHPDrainValueRace" : 99999,
    "bSPDrainValueRace" : 99999,
    "bIgnoreMdefRaceRate" : 99999,
    "bIgnoreDefRaceRate" : 99999, // Engkanto card
    'bSkillHeal2' : 99999,
    "bAddEffOnSkill" : 99999,
    "bHealPower" : bon_HEAL_MUL, // Also bon_SANC_MUL
    "bHealPower2" : 99999,

    "bSkillAtk" : function(value) { return sumKeysValue(bon_DMG_SKILL, value); },

    // Skills
    "RK_HUNDREDSPEAR" : 448,
    "RK_IGNITIONBREAK" : 450,
    "RA_AIMEDBOLT" : 511,
    "RA_ARROWSTORM" : 512,
    "RA_CLUSTERBOMB" : 523,
    "RA_WUGSTRIKE" : 529,

    // Elements
    "Ele_Neutral": ele_NEUTRAL,
    "Ele_Water": ele_WATER,
    "Ele_Earth": ele_EARTH,
    "Ele_Fire": ele_FIRE,
    "Ele_Wind": ele_WIND,
    "Ele_Poison": ele_POISON,
    "Ele_Holy": ele_HOLY,
    "Ele_Dark": ele_DARK,
    "Ele_Ghost": ele_GHOST,
    "Ele_Undead": ele_UNDEAD,

    // Status
    "Eff_Poison": status_POISON,
    "Eff_Stun": status_STUN,
    "Eff_Freeze": status_FREEZE,
    "Eff_Curse": status_CURSE,
    "Eff_Blind": status_BLIND,
    "Eff_Sleep": status_SLEEP,
    "Eff_Silence": status_SILENCE,
    "Eff_Chaos": status_CHAOS,
    "Eff_Bleeding": status_BLEEDING,
    "Eff_Stone": status_STONE,

    // Races
    "RC_Formless" : race_FORMLESS,
    "RC_Undead" : race_UNDEAD,
    "RC_Brute" : race_BRUTE,
    "RC_Plant" : race_PLANT,
    "RC_Insect" : race_INSECT,
    "RC_Fish" : race_FISH,
    "RC_Demon" : race_DEMON,
    "RC_DemiHuman" : race_DEMI_HUMAN,
    "RC_Angel" : race_ANGEL,
    "RC_Dragon" : race_DRAGON,

    // Sizes
    "Size_Small" : siz_SMALL,
    "Size_Medium" : siz_MEDIUM,
    "Size_Big" : siz_LARGE,
  }

  function remapKeys(db) {
    if (typeof db === 'number') {
      return db;
    }
    var result = {}
    for (var key in db) {
      var value = remapKeys(db[key]);
      if (keysRemapList[key] === undefined) {
        result[key] = value;
      } else if (typeof keysRemapList[key] === 'number') {
        result[keysRemapList[key]] = value;
      } else {
        var remapped = keysRemapList[key](value);
        result[remapped.key] = remapped.value;
      }
    }
    return result;
  }

  return function(script, item) {
    var parsed = rathena_parser.parse(script);
    var bStr = 1,
        bInt = 2,
        bDex = 3,
        bVit = 4,
        bAgi = 5,
        bLuk = 6;
    function readparam(param) {
      switch(param) {
        case bStr:
          return SU_STR;
        case bInt:
          return SU_INT;
        case bDex:
          return SU_DEX;
        case bAgi:
          return SU_AGI;
        case bVit:
          return SU_VIT;
        case bLuk:
          return SU_LUK;
        default:
          return 0;
      }
    }

    function getrefine() {
      if (item.type > weapTyp_BARE && item.type <= weapTyp_GRENADE) {
        return n_A_Weapon_ATKplus;
      } else {
        switch (item.type) {
          case itm_type_HEAD_UPPER:
            return n_A_HEAD_DEF_PLUS;
          case itm_type_ARMOR:
            return n_A_BODY_DEF_PLUS;
          case itm_type_SHIELD:
            return n_A_LEFT_DEF_PLUS;
          case itm_type_GARMENT:
            return n_A_SHOULDER_DEF_PLUS;
          case itm_type_SHOES:
            return n_A_SHOES_DEF_PLUS;
          default:
            return 0;
        }
      }
    }

    var BaseLevel = n_A_BaseLV;
    var JobLevel = n_A_JobLV;

    var DB = {};

    function nestedAssign(obj, previous_value) {
      if (obj === parseInt(obj, 10)) {
        return parseInt(obj) + previous_value;
      } else if (typeof obj === 'string') {
        return parseInt(eval(obj.replace(".@", "_locals."))) + previous_value;
      } else {
        for (var key in obj) {
          if (previous_value[key]) {
            obj[key] = nestedAssign(obj[key], previous_value[key]);
          } else {
            obj[key] = nestedAssign(obj[key], 0);
          }
        }
        return obj;
      }
    }

    var _locals = {};

    function nestedEval(obj) {
      for (var key in obj) {
        if (key == 'command') {
          eval(obj[key]);
        } else if (key == 'conditions') {
          for (var i = 0; i < obj[key].length; i++) {
            var condition = obj[key][i];
            if (eval(condition.condition)) {
              for (var j = 0; j < condition.body.length; j++) {
                nestedEval(condition.body[j]);
              }
            } else {
            }
          }
        } else if (DB[key]) {
          DB[key] = nestedAssign(obj[key], DB[key])
        } else {
          DB[key] = nestedAssign(obj[key], 0)
        }
      }
    }
    nestedEval(parsed);
    console.log(remapKeys(DB))
    return remapKeys(DB);
  };
})();
