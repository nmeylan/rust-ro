var formElements;

var variableCastTime = 1;
var fixedCastTime = 1;
var fixedCastTimeSub = 0;
var totalCastTime = 1;
var globalCastDelay = 1;
var reuseDelay = 1;
var trifectaBlowActivationRate = 0; // Trifecta Blow activation chance
var trifectaBlowDamage = 0; // Trifecta Blow damage
var dupleLightChance = 0; // Duple Light chance
var dupleLightMagicalDamage = 0; // Duple Light damage
var dupleLightPhysicalDamage = 0; // Duple Light damage
var n_A_JOB = 0; // novi
var n_Nitou = 0; // Dual Hand
var rebirthClass = 0;
var thirdClass = 0;
var n_Ses = 0;
var damageType = 0;

var LoadDataMain = new Array();
var LoadDataName = new Array();
var DataShortCut = new Array();

var n_A_Equip = new Array();
var n_A_card = new Array();
var w_ASSP0bk = new Array();

var n_A_MaxHP = 1;
var n_A_MaxSP = 1;
var n_A_STR = 1;
var n_A_AGI = 1;
var n_A_VIT = 1;
var n_A_INT = 1;
var n_A_DEX = 1;
var n_A_LUK = 1;

var SU_STR = 1; // base STR

var n_A_HEAD_DEF_PLUS = 0;
var n_A_BODY_DEF_PLUS = 0;
var n_A_LEFT_DEF_PLUS = 0; // shield
var n_A_SHOES_DEF_PLUS = 0;
var n_A_SHOULDER_DEF_PLUS = 0;
var n_A_Weapon_ATKplus = 0;
var n_A_Weapon2_ATKplus = 0;

var n_A_Weapon_element;
var n_A_Weapon2_element;


var n_A_MobSkill = 0;
var n_A_MobSkillLV = 1;
var noequipatk = false;



// reset passive skills ---
var selfBuffs = new Array();
var acolyteBuffs = new Array();
var performerBuffs = new Array();
var guildBuffs = new Array();
var battleChantBuffs = new Array();
var otherBuffs = new Array();
var usableItems = new Array();
var miscEffects = new Array();
var battleEffects = new Array();
var monsterDebuffs = new Array();
var monsterBuffs = new Array();

var wBCEDPch = 0;
var wLAch = 0;
var wCriTyuu = 0;
var hunterPetHits = 0;
var hunterPetDamage = 0;
var not_use_card = 0;
var str_bSUBname = "";
var str_bSUB = "";
var SuperNoviceFullWeaponCHECK = 0;
var cast_kotei = 0;
var b = 0;
var n_PerHIT_DMG = 0;
var n_Delay = new Array();
var totalDelay = 0;
var w_AG = [100,95,90,86,82,79,76,74,72,71,70]; // Guard-Blockrate

var statusAttack = 0;
var strengthBonusAttack = 0;
var strengthBonusAttack2 = 0;
var masteryAttack = 0;
var equipmentAttack = 0;
var overrefineAttack = 0;
var overrefineAttack2 = 0;
var minOverrefineAttack = 0;
var minOverrefineAttack2 = 0;
var varianceAttack = 0;
var varianceAttack2 = 0;
var weaponUpgradeAttack = 0;
var weaponUpgradeAttack2 = 0;
var overrefineMagicAttack = 0;
var minOverrefineMagicAttack = 0;

// modifiers
var weaponSizeMod = 1;
var weapon2SizeMod = 1;
var weaponElementalMod = 1;
var statusElementalMod = 1;
var racialMod = 1;
var specialRacialMod = 1;
var sizeMod = 1;
var bossMod = 1;
var attackMod = 1;
var criticalMod = 1;
var rangedMod = 1;

var n_tok = new Array(); // stats & substats

var first_check = 0;
var n_B = new Array();
var Last_DMG_A = [0,0,0];
var Last_DMG_B = [0,0,0];
var InnStr = new Array();
var w_DMG = new Array();
var n_A_DMG = new Array();
var baseDamage = new Array();
var n_A_CriATK = new Array();
var n_AveATKnum = 0;
var doubleAttackHit = 0;
var criticalAttackChance = 0;

var SG_Special_HITnum = 0;
var SG_Special_DMG = [0,0,0];

var ItemCardNumberCheck = 142;
var determiningEDPdamage = 0;

// Cookies ?
var n_SaveMode = 1;
var n_LastSaveNum = 0;