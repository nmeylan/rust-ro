{ // Classes - n_A_Job = % - n_A_JobSearchX() = %
cls_NOV = 0;
cls_SWO = 1;
cls_THI = 2;
cls_ACO = 3;
cls_ARC = 4;
cls_MAG = 5;
cls_MER = 6;
cls_KNI = 7;
cls_ASS = 8;
cls_PRI = 9;
cls_HUN = 10;
cls_WIZ = 11;
cls_BLA = 12;
cls_CRU = 13;
cls_ROG = 14;
cls_MON = 15;
cls_BAR = 16;
cls_DAN = 17;
cls_SAG = 18;
cls_ALC = 19;
cls_SNOVI = 20;
cls_LOR = 21;
cls_ASX = 22;
cls_HPR = 23;
cls_SNI = 24;
cls_HWI = 25;
cls_MAS = 26;
cls_PAL = 27;
cls_STA = 28;
cls_CHA = 29;
cls_CLO = 30;
cls_GYP = 31;
cls_SCH = 32;
cls_BIO = 33;
cls_HNOV = 34;
cls_HSWO = 35;
cls_HTHI = 36;
cls_HACO = 37;
cls_HARC = 38;
cls_HMAG = 39;
cls_HMER = 40;
cls_TKK = 41;
cls_TKM = 42;
cls_SL = 43;
cls_NIN = 44;
cls_GUN = 45;
cls_RUN = 46;
cls_RUNt = 47;
cls_GLT = 48;
cls_GLTt = 49;
cls_ABI = 50;
cls_ABIt = 51;
cls_RAN = 52;
cls_RANt = 53;
cls_WAR = 54;
cls_WARt = 55;
cls_MEC = 56;
cls_MECt = 57;
cls_ROY = 58;
cls_ROYt = 59;
cls_SHA = 60;
cls_SHAt = 61;
cls_SUR = 62;
cls_SURt = 63;
cls_MIN = 64;
cls_MINt = 65;
cls_WAN = 66;
cls_WANt = 67;
cls_SOR = 68;
cls_SORt = 69;
cls_GEN = 70;
cls_GENt = 71;
cls_KAGOB = 72;
cls_ENOVI = 73;
cls_COUNT = 74;
}

{ // Sizes
siz_SMALL = 0;
siz_MEDIUM = 1;
siz_LARGE = 2;
}

{ // Races - n_B[en_RACE] = %
race_FORMLESS = 0;
race_UNDEAD = 1;
race_BRUTE = 2;
race_PLANT = 3;
race_INSECT = 4;
race_FISH = 5;
race_DEMON = 6;
race_DEMI_HUMAN = 7;
race_ANGEL = 8;
race_DRAGON = 9;
}

{ // Elements - n_A_Weapon_element = % - n_A_BodyElement = % - element[..][%]
ele_NEUTRAL = 0;
ele_WATER = 1;
ele_EARTH = 2;
ele_FIRE = 3;
ele_WIND = 4;
ele_POISON = 5;
ele_HOLY = 6;
ele_DARK = 7;
ele_GHOST = 8;
ele_UNDEAD = 9;
}

{ // Damage Type
var kDmgTypeMelee = 0;
var kDmgTypeRanged = 1;
var kDmgTypeMagic = 2;
}
  // Bonuses (from Equip and Cards)/ Stats/ SubStats
{ // n_tok[%] - StPlusCalc2(%) - StPlusCard(%) - StPlusWeapon(%)
bon_NONE = 0;
bon_STR = 1;
bon_AGI = 2;
bon_VIT = 3;
bon_INT = 4;
bon_DEX = 5;
bon_LUK = 6;
bon_ALL_STATS = 7;

bon_HIT = 8;
bon_FLEE = 9;
bon_CRIT = 10;
bon_PDODGE = 11;
bon_ASPD_MUL = 12;
bon_HP_ADD = 13;
bon_SP_ADD = 14;
bon_HP_MUL = 15;
bon_SP_MUL = 16;
bon_ATK = 17; // flat attack
bon_DEF = 18;
bon_MDEF = 19;

bon_ELEMENT = 20;
bon_ELE = 20;

bon_ASPD_ADD = 21; // flat ASPD bonus

bon_IGN_DEF_NONBOSS = 22;
bon_ICE_PICK = 23;
bon_USR_DEF_DIV = 24; // 2 => userHDef/2 & userSDef/2

bon_DMG_RANGE = 25;
bon_DMG_BOSS = 26;
bon_DMG_SIZ_SMALL = 27;
bon_DMG_SIZ_MEDIUM = 28;
bon_DMG_SIZ_LARGE = 29;

bon_DMG_RC_FORMLESS = 30;
bon_DMG_RC_UNDEAD = 31;
bon_DMG_RC_BRUTE = 32;
bon_DMG_RC_PLANT = 33;
bon_DMG_RC_INSECT = 34;
bon_DMG_RC_FISH = 35;
bon_DMG_RC_DEMON = 36;
bon_DMG_RC_DEMI_HUMAN = 37;
bon_DMG_RC_ANGEL = 38;
bon_DMG_RC_DRAGON = 39;

bon_DMG_ELE_NEUTRAL = 40;
bon_DMG_ELE_WATER = 41;
bon_DMG_ELE_EARTH = 42;
bon_DMG_ELE_FIRE = 43;
bon_DMG_ELE_WIND = 44;
bon_DMG_ELE_POISON = 45;
bon_DMG_ELE_HOLY = 46;
bon_DMG_ELE_SHADOW = 47;
bon_DMG_ELE_DARK = 47;
bon_DMG_ELE_GHOST = 48;
bon_DMG_ELE_UNDEAD = 49;

bon_RED_RC_FORMLESS = 50;
bon_RED_RC_UNDEAD = 51;
bon_RED_RC_BRUTE = 52;
bon_RED_RC_PLANT = 53;
bon_RED_RC_INSECT = 54;
bon_RED_RC_FISH = 55;
bon_RED_RC_DEMON = 56;
bon_RED_RC_DEMI_HUMAN = 57;
bon_RED_RC_ANGEL = 58;
bon_RED_RC_DRAGON = 59;

bon_RED_ELE_NEUTRAL = 60;
bon_RED_ELE_WATER = 61;
bon_RED_ELE_EARTH = 62;
bon_RED_ELE_FIRE = 63;
bon_RED_ELE_WIND = 64;
bon_RED_ELE_POISON = 65;
bon_RED_ELE_HOLY = 66;
bon_RED_ELE_SHADOW = 67;
bon_RED_ELE_DARK = 67;
bon_RED_ELE_GHOST = 68;
bon_RED_ELE_UNDEAD = 69;

bon_DMG_CRIT = 70;
bon_REFLECT_PHY_DMG = 71;
bon_RED_FIXEDCAST = 72; // positive = shorter
bon_RED_CAST = 73; // negative = shorter
bon_RED_CASTDELAY = 74;
bon_HP_REG = 75;
bon_HP_REGEN = 75;
bon_SP_REG = 76;
bon_SP_REGEN = 76;

bon_RED_BOSS = 77;
bon_RED_RANGE = 78;
bon_RED_NON_BOSS = 79;
bon_RED_NONBOSS = 79;

bon_PHY_ATK = 80; // multiplier
bon_DMG_GOBLIN = 81;
bon_DMG_KOBOLD = 82;
bon_DMG_ORC = 83;
bon_DMG_GOLEM = 84;
bon_DEF_MUL = 85; // Def := Def * (1 - %/100)
bon_CH_GUIDE_ATK = 86; // %
// = 87; // ?
bon_MATK = 88; // new flat MAtk addition
bon_MATK_MUL = 89; // calced before some passive skills (spe bonus)
bon_SETID = 90; // do not insert manually, but in item.js
bon_HEAL_MUL = 91; // Increases Heal
bon_HEAL_REC = 92; // Increased incoming Heal and Items
bon_HEAL_UNDEAD = 93; // Increases OffensiveHeal
bon_SANC_MUL = 94; // radwisdom, gloCureWand, diaRing - don't get it
// = 95; // free
//94: +% [Sanctuary] Effectiveness
//95: +% [Sanctuary] Effectiveness on Self
bon_HEAL_UNDEADII = 96; // see 93 ?
// = 97; // free
// = 98; // free
// = 99; // free
// = 100; // free
// ...
bon_CRIT_RC_FORMLESS = 110;
bon_CRIT_RC_UNDEAD = 111;
bon_CRIT_RC_BRUTE = 112;
bon_CRIT_RC_PLANT = 113;
bon_CRIT_RC_INSECT = 114;
bon_CRIT_RC_FISH = 115;
bon_CRIT_RC_DEMON = 116;
bon_CRIT_RC_DEMI_HUMAN = 117;
bon_CRIT_RC_ANGEL = 118;
bon_CRIT_RC_DRAGON = 119;

bon_EXP_RC_FORMLESS = 120;
bon_EXP_RC_UNDEAD = 121;
bon_EXP_RC_BRUTE = 122;
bon_EXP_RC_PLANT = 123;
bon_EXP_RC_INSECT = 124;
bon_EXP_RC_FISH = 125;
bon_EXP_RC_DEMON = 126;
bon_EXP_RC_DEMI_HUMAN = 127;
bon_EXP_RC_ANGEL = 128;
bon_EXP_RC_DRAGON = 129;

bon_CH_STATUS_POISON = 130;
bon_CH_STATUS_STUN = 131;
bon_CH_STATUS_FREEZE = 132;
bon_CH_STATUS_CURSE = 133;
bon_CH_STATUS_BLIND = 134;
bon_CH_STATUS_SLEEP = 135;
bon_CH_STATUS_SILENCE = 136;
bon_CH_STATUS_CHAOS = 137;
bon_CH_STATUS_BLEEDING = 138;
bon_CH_STATUS_STONE = 139;
bon_CH_BREAK_WEAPON = 140;
bon_CH_BREAK_ARMOR = 141;
bon_CH_STATUS_DEEP_SLEEP = 142;
// = 142; // free
// = 143; // free
// = 144; // free
// = 145; // free
// = 146; // free
// = 147; // free
// = 148; // free
// = 149; // free
bon_RES_STATUS_POISON = 150; // Chaos, and Silence statuses
bon_RES_STATUS_STUN = 151;
bon_RES_STATUS_FREEZE = 152;
bon_RES_STATUS_CURSE = 153;
bon_RES_STATUS_BLIND = 154;
bon_RES_STATUS_SLEEP = 155;
bon_RES_STATUS_SILENCE = 156;
bon_RES_STATUS_CHAOS = 157;
bon_RES_STATUS_BLEEDING = 158;
bon_RES_STATUS_STONE = 159;
bon_RES_STATUS_DEEP_SLEEP = 160;
bon_RES_STATUS_BURNING = 161;
bon_RES_STATUS_CRISTALLIZATION = 162;

bon_MDMG_RC_FORMLESS = 170;
bon_MDMG_RC_UNDEAD = 171;
bon_MDMG_RC_BRUTE = 172;
bon_MDMG_RC_PLANT = 173;
bon_MDMG_RC_INSECT = 174;
bon_MDMG_RC_FISH = 175;
bon_MDMG_RC_DEMON = 176;
bon_MDMG_RC_DEMI_HUMAN = 177;
bon_MDMG_RC_ANGEL = 178;
bon_MDMG_RC_DRAGON = 179;

bon_IGN_DEF_RC_FORMLESS = 180; // 0 / 1
bon_IGN_DEF_RC_UNDEAD = 181;
bon_IGN_DEF_RC_BRUTE = 182;
bon_IGN_DEF_RC_PLANT = 183;
bon_IGN_DEF_RC_INSECT = 184;
bon_IGN_DEF_RC_FISH = 185;
bon_IGN_DEF_RC_DEMON = 186;
bon_IGN_DEF_RC_DEMI_HUMAN = 187;
bon_IGN_DEF_RC_ANGEL = 188;
bon_IGN_DEF_RC_DRAGON = 189;

bon_RED_SIZ_SMALL = 190;
bon_RED_SIZ_MEDIUM = 191;
bon_RED_SIZ_LARGE = 192;

bon_NO_UPGRADES = 193;
bon_UNBREAKABLE = 194;
bon_TWO_HANDED_STAFF = 195;
bon_USR_ELEMENT = 198; // Elemental Armor
bon_USR_ELE = 198;

bon_SET_STR = 211; // after mults ?
bon_SET_AGI = 212;
bon_SET_VIT = 213;
bon_SET_INT = 214;
bon_SET_DEX = 215;
bon_SET_LUK = 216;

bon_DEFIGN_RC_ALL = 295;

// [ +% Def-ignore on Race 300: Formless
bon_DEFIGN_RC_FORMLESS = 300;
bon_DEFIGN_RC_UNDEAD = 301;
bon_DEFIGN_RC_BRUTE = 302;
bon_DEFIGN_RC_PLANT = 303;
bon_DEFIGN_RC_INSECT = 304;
bon_DEFIGN_RC_FISH = 305;
bon_DEFIGN_RC_DEMON = 306;
bon_DEFIGN_RC_DEMI_HUMAN = 307;
bon_DEFIGN_RC_ANGEL = 308;
bon_DEFIGN_RC_DRAGON = 309;

// [ +% MDef-ignore on Race 310: Formless
bon_MDEFIGN_RC_FORMLESS = 310;
bon_MDEFIGN_RC_UNDEAD = 311;
bon_MDEFIGN_RC_BRUTE = 312;
bon_MDEFIGN_RC_PLANT = 313;
bon_MDEFIGN_RC_INSECT = 314;
bon_MDEFIGN_RC_FISH = 315;
bon_MDEFIGN_RC_DEMON = 316;
bon_MDEFIGN_RC_DEMI_HUMAN = 317;
bon_MDEFIGN_RC_ANGEL = 318;
bon_MDEFIGN_RC_DRAGON = 319;

// [ +% Critical Damage on Race 320: Formless
bon_CRIT_DMG_RC_FORMLESS = 320;
bon_CRIT_DMG_RC_UNDEAD = 321;
bon_CRIT_DMG_RC_BRUTE = 322;
bon_CRIT_DMG_RC_PLANT = 323;
bon_CRIT_DMG_RC_INSECT = 324;
bon_CRIT_DMG_RC_FISH = 325;
bon_CRIT_DMG_RC_DEMON = 326;
bon_CRIT_DMG_RC_DEMI_HUMAN = 327;
bon_CRIT_DMG_RC_ANGEL = 328;
bon_CRIT_DMG_RC_DRAGON = 329;

// New shieldcards ?
bon_RED_ELE2_NEUTRAL = 330;
bon_RED_ELE2_WATER = 331;
bon_RED_ELE2_EARTH = 332;
bon_RED_ELE2_FIRE = 333;
bon_RED_ELE2_WIND = 334;
bon_RED_ELE2_POISON = 335;
bon_RED_ELE2_HOLY = 336;
bon_RED_ELE2_SHADOW = 337;
bon_RED_ELE2_DARK = 337;
bon_RED_ELE2_GHOST = 338;
bon_RED_ELE2_UNDEAD = 339;
//Increase damage of elemental magic
bon_INC_MAGIC_NEUTRAL = 340;
bon_INC_MAGIC_WATER = 341;
bon_INC_MAGIC_EARTH = 342;
bon_INC_MAGIC_FIRE = 343;
bon_INC_MAGIC_WIND = 344;
bon_INC_MAGIC_POISON = 345;
bon_INC_MAGIC_HOLY = 346;
bon_INC_MAGIC_DARK = 347;
bon_INC_MAGIC_GHOST = 348;
bon_INC_MAGIC_UNDEAD = 349;

// [ +% Damage on Single MonsterID: (BonusID - 1000) 1000 - 2999 ]
bon_DMG_MONSTER = 1000;
bon_DMG_SPE_CRAB = 1072;
bon_DMG_SPE_ASTER = 1240;
bon_DMG_SPE_BONGUN = 1275;

// [ +% Resistance from Single MonsterID: (BonusID - 3000) 3000 - 4999 ]
bon_RED_MONSTER = 3000;
// [ +% Damage with SkillID: (BonusID - 5000) 5000 - 6999 ]
bon_DMG_SKILL = 5000;
// [ +% Cast Time Reduction with SkillID: (BonusID - 7000) 7000-8999 ]
bon_CAST_SKILL = 7000;
// [ +% Cast Time Reduction in second with SkillID: (BonusID - 9000) 9000-10999 ]
bon_CAST_SKILL_FLAT = 9000;
// [ +% Cast Time Reduction in second with SkillID: (BonusID - 11000) 11000-12999 ]
bon_DELAY_SKILL_FLAT = 11000;

//EnablesSkills 220, 221 // IceFalch & FBlend
bon_SKILL_ENABLE = 220;
bon_SKILL_AUTOCAST = 221;
// ^ ColdBolt, FireBolt, Bash, TStorm, TU,

bon_SCRIPT = 222;
}

{ // ASPD-Pots
spdpot_NONE = 0;
spdpot_CONC = 1;
spdpot_AWAK = 2;
spdpot_BERSERK = 3;
}

{ // Enemy(Sub)Stats - n_B[%]
en_ID = 0;
en_CLASS = 1;
en_RACE = 2;
en_ELEMENT = 3;
en_SIZE = 4;
en_LEVEL = 5;
en_HP = 6;
en_VIT = 7;
en_AGI = 8;
en_INT = 9;
en_DEX = 10;
en_LUK = 11;
en_MINATK = 12;
en_MAXATK = 13;
en_HARDDEF = 14;
en_HARDMDEF = 15;
en_BASEEXP = 16;
en_JOBEXP = 17;
// ---
en_BOSS = 19;
en_RANGED = 20;
en_PERFECT_HIT = 21;
en_PERFECT_DODGE = 22;
en_SOFTDEF = 23;
en_MAXSOFTDEF = 24;
en_SOFTMDEF = 25;
en_HIT = 26;
en_FLEE = 27;

PVP_ENEMY = 578;
}

{ // EquipTypes - n_A_Equip[%]
eq_WEAPON = 0;
eq_WEAP = 0;
eq_LEFT_WEAPON = 1;
eq_LEFT_WEAP = 1;
eq_LEFT_HAND = 1;
eq_WEAPONII = 1;
eq_HEAD = 2;
eq_HEAD_UPPER = 2;
eq_HEAD_MIDDLE = 3;
eq_HEAD_LOWER = 4;
eq_LEFT = 5;
eq_SHIELD = 5;
eq_BODY = 6;
eq_ARMOR = 6;
eq_SHOULDER = 7;
eq_GARMENT = 7;
eq_SHOES = 8;
eq_ACCO = 9;
eq_ACCI = 9;
eq_ACCT = 10;
eq_ACCII = 10;
}

{ // Cards - n_A_card[%] - cardOBJ[id][%] - cardOBJ[id][card_att_COMP] = %
// Card Location - n_A_card[%]
card_loc_WEAPON_I = 0;
card_loc_WEAPON_II = 1;
card_loc_WEAPON_III = 2;
card_loc_WEAPON_IV = 3;
card_loc_WEAPONII_I = 4;
card_loc_WEAPONII_II = 5;
card_loc_WEAPONII_III = 6;
card_loc_WEAPONII_IV = 7;
card_loc_HEAD_UPPER = 8;
card_loc_HEAD_MIDDLE = 9;
card_loc_SHIELD = 10;
card_loc_ARMOR = 11;
card_loc_GARMENT = 12;
card_loc_SHOES = 13;
card_loc_ACCI = 14;
card_loc_ACCII = 15;
card_loc_SET = 16; // not used
// Card Attributes - cardOBJ[id][%]
card_att_ID = 0;
card_att_COMP = 1; // not above !!, but similar
card_att_NAME = 2;
card_att_DESC = 3;
card_att_BONUS_START = 4;
// What's in card_COMP - cardOBJ[id][card_att_COMP] = %
card_comp_NONE = 0;
card_comp_WEAPON = 1;
card_comp_HEAD = 2;
card_com_SHIELD = 3;
card_com_ARMOR = 4;
card_com_GARMENT = 5;
card_com_SHOES = 6;
card_com_ACC = 7;
card_com_IMPOSSIBLE = 10;

card_CARD = 0;
}

{ // WeaponTypes - n_A_WeaponType = % - ItemOBJ[id][itm_TYPE] = % - itemAsPD[5+1]
weapTyp_NONE = 0;
weapTyp_BARE = 0;
weapTyp_DAGGER = 1;
weapTyp_SWORD = 2;
weapTyp_SW = 2;
weapTyp_2HSWORD = 3;
weapTyp_2HS = 3;
weapTyp_SWORDII = 3;
weapTyp_SPEAR = 4;
weapTyp_2HSPEAR = 5;
weapTyp_SPEARII = 5;
weapTyp_AXE = 6;
weapTyp_2HAXE = 7;
weapTyp_2HA = 7;
weapTyp_AXEII = 7;
weapTyp_MACE = 8;
weapTyp_ROD = 9;
weapTyp_STAFF = 9;
weapTyp_BOW = 10;
weapTyp_KATAR = 11;
weapTyp_BOOK = 12;
weapTyp_KNUCKLE = 13;
weapTyp_INSTRUMENT = 14;
weapTyp_INSTRU = 14;
weapTyp_WHIP = 15;
weapTyp_HUUMA_SHURIKEN = 16;
weapTyp_HUUMA = 16;
weapTyp_HANDGUN = 17;
weapTyp_RIFLE = 18;
weapTyp_SHOTGUN = 19;
weapTyp_GATLING_GUN = 20;
weapTyp_GATLING = 20;
weapTyp_GRENADE_LAUNCHER = 21;
weapTyp_GRENADE = 21;
weapTyp_SHIELD = 22; // for ASPD calculation
}

{ // ItemStats - ItemOBJ[id][%] - ItemOBJ[id][itm_TYPE] = %
itm_ID = 0;
itm_TYPE = 1;
itm_REQ_JOB = 2;
itm_ATK = 3;
itm_DEF = 3;
itm_WLVL = 4;
itm_SLOTS = 5; // Text!
itm_WEIGHT = 6;
itm_REQ_BLVL = 7;
itm_BONUS_START = 8;
// BonusX, AmountX, BonusY, AmountY, ..., 0

// ItemOBJ[id][itm_TYPE] = %
// 1~21 --> WeaponTypes
itm_type_HEAD_UPPER = 50;
itm_type_HEAD_MIDDLE = 51;
itm_type_HEAD_LOWER = 52;

itm_type_ARMOR = 60;
itm_type_SHIELD = 61;
itm_type_GARMENT = 62;
itm_type_SHOES = 63;
itm_type_ACCESSORY = 64;

itm_type_SET = 100;
itm_type_UNOBTAINABLE = 999;
}

{ // ArrowTypes - n_A_Arrow = % - ArrowOBJ[n_A_Arrow][%]
// n_A_Arrow = %
arrTyp_NONE = 0;
arrTyp_ARROW = 1;
arrTyp_SILVER = 2;
arrTyp_FIRE = 3;
arrTyp_IRON = 4;
arrTyp_STONE = 5;
arrTyp_CRYSTAL = 6;
arrTyp_WIND = 7;
arrTyp_SHADOW = 8;
arrTyp_DARK = 8;
arrTyp_IMMATERIAL = 9;
arrTyp_IMMA = 9;
arrTyp_RUSTY = 10;
arrTyp_STEEL = 11;
arrTyp_ORIDECON = 12;
arrTyp_ORI = 12;
arrTyp_COUNTER_EVIL = 13;
arrTyp_COUNTER = 13;
arrTyp_EVIL = 13;
arrTyp_FROZEN = 14;
arrTyp_POISON = 15;
arrTyp_SHARP = 16;
arrTyp_HOLY = 17;
arrTyp_ELVEN = 18;
arrTyp_HUNTING = 19;
arrTyp_OTHER = 20;
// Arrow Attributes - ArrowOBJ[n_A_Arrow][%]
arr_att_ATK = 0;
arr_att_ELEMENT = 1;
arr_att_DESC = 2; // + Language
}

{ // BulletTypes
bulTyp_BULLET = 0;
bulTyp_SILVER = 1;
bulTyp_BLOOD = 2;
}

{ // GrenadeTypes
grenTyp_FLARE = 0;
grenTyp_FREEZING = 1;
grenTyp_LIGHTNING = 2;
grenTyp_BLIND = 3;
grenTyp_POISON = 4;
}

{ // ShurikenTypes
shuTyp_SHURIKEN = 0;
shuTyp_NUMBUS = 1;
shuTyp_FLASH = 2;
shuTyp_SHARP_LEAF = 3;
shuTyp_SHARP = 3;
shuTyp_THORN_NEEDLE = 4;
shuTyp_THORN = 4;
shuTyp_NEEDLE = 4;
}

{ // KunaiTypes
kunTyp_HEAT_WAVE = 0;
kunTyp_HEAT = 0;
kunTyp_FIRE = 0;
kunTyp_ICICLE = 1;
kunTyp_WATER = 1;
kunTyp_ICE = 1;
kunTyp_HIGH_WIND = 2;
kunTyp_WIND = 2;
kunTyp_BLACK_EARTH = 3;
kunTyp_EARTH = 3;
kunTyp_FELL_POISON = 4;
kunTyp_POISON = 4;
}

{ // Sorcerer Summon Type
    summonType_AGNI = 0;
    summonType_VENTUS = 1;
    summonType_AQUA = 2;
    summonType_TERRA = 3;
}

{ // Sorcerer Summon Stances
    summonStance_NOSPIRIT = 0;
    summonStance_IDLE = 1;
    summonStance_DEFENSE = 2;
    summonStance_OFFENSE = 3;
}

{ // Statuses - ? - monsterDebuffs[%]
// = ?
status_POISON = 0;
status_STUN = 1;
status_FREEZE = 2;
status_CURSE = 3;
status_BLIND = 4;
status_SLEEP = 5;
status_SILENCE = 6;
status_CHAOS = 7;
status_BLEEDING = 8;
status_STONE = 9;
status_WEAPON_BREAK = 10;
status_ARMOR_BREAK = 11;

// monsterDebuffs[%]
status_en_PROVOKE = 0;
status_en_QUAG = 1;
status_en_POISON = 2;
status_en_BLIND = 3;
status_en_FREEZE = 4;
status_en_BLESS = 5;
status_en_LEXA = 6;
status_en_STUN = 7;
status_en_SLEEP = 8;
status_en_STONE = 9;
status_en_CURSE = 10;
status_en_AGIDOWN = 11;
status_en_SCRUSIS = 12;
status_en_STRIPW = 13;
status_en_STRIPS = 14;
status_en_STRIPA = 15;
status_en_STRIPH = 16;
status_en_FIBER = 17;
status_en_MBREAK = 18;
status_en_SLGRACE = 19;
status_en_DOWNTEMPO = 20;
status_en_ESKA = 21;
status_en_ESKE = 22;
status_en_ELECHANGE = 23;
status_en_COINFLING = 24;
status_en_DEEPSLEEP = 25;
status_en_VENOM_IMPRESS = 26;
status_en_MARSH_OF_ABYSS = 27;
status_en_GLOOMY_DAY = 28;
status_en_DARK_CLAW = 29;

// monsterBuffs
var status_en_buff_IncreaseAGI = 0;
var status_en_buff_Assumptio = 1;
var status_en_buff_AdrenalineRush = 2;
var status_en_buff_MaximizePower = 3;
var status_en_buff_PowerUp = 4;
var status_en_buff_FleeUp = 5;
var status_en_buff_ElementalChange = 6;
var status_en_buff_StoneSkin = 7;
var status_en_buff_MagicMirror = 8;
var status_en_buff_Keeping = 9;
var status_en_buff_Race = 10;
var status_en_buff_Elemental = 11;
var status_en_buff_Ranged = 12;
var status_en_buff_Size = 13;
var status_en_buff_Normal = 14;
var status_en_buff_Other = 15;
}

{ // Skills - SkillSearch(%) - n_A_ActiveSkill = %
skill_ALL_BASIC_ATTACK = 0;
skill_ALL_FIRST_AID = 1;
skill_ALL_PLAY_DEAD = 2;

skill_SW_SWORD_MASTERY = 3;
skill_SW_TWO_HAND_SWORD_MASTERY = 4;
skill_SW_INCREASED_HP_RECOVERY = 5;
skill_SW_BASH = 6;
skill_SW_MAGNUM_BREAK = 7;
skill_SW_PROVOKE = 8;
skill_SW_ENDURE = 9;
skill_SW_HP_RECOVERY_WHILE_MOVING = 10;
skill_SW_FATAL_BLOW = 11;
skill_SW_BERSERK = 12;

skill_TH_DOUBLE_ATTACK = 13;
skill_TH_IMPROVE_DODGE = 14;
skill_TH_STEAL = 15;
skill_TH_HIDING = 16;
skill_TH_HIDE = 16;
skill_TH_ENVENOM = 17;
skill_TH_DETOXIFY = 18;
skill_TH_SAND_ATTACK = 19;
skill_TH_BACK_SLIDE = 20;
skill_TH_FIND_STONE = 21;
skill_TH_STONE_FLING = 22;

skill_AC_DIVINE_PROTECTION = 23;
skill_AC_DEMON_BANE = 24;
skill_AC_HEAL = 25;
skill_AC_CURE = 26;
skill_AC_INCREASE_AGI = 27;
skill_AC_AGI = 27;
skill_AC_AGI_UP = 27;
skill_AC_DECREASE_AGI = 28;
skill_AC_AGI_DOWN = 28;
skill_AC_SIGNUM_CRUSIS = 29;
skill_AC_ANGELUS = 30;
skill_AC_BLESSING = 31;
skill_AC_BLESS = 31;
skill_AC_PNEUMA = 32;
skill_AC_AQUA_BENEDICTA = 33;
skill_AC_RUWACH = 34;
skill_AC_TELEPORT = 35;
skill_AC_WARP_PORTAL = 36;
skill_AC_HOLY_LIGHT = 37;
// Archer
skill_AR_OWL_EYE = 38;
skill_AR_OWLS_EYE = 38;
skill_AR_VULTURE_EYE = 39;
skill_AR_VULTURES_EYE = 39;
skill_AR_DOUBLE_STRAFE = 40;
skill_AR_DS = 40;
skill_AR_ARROW_SHOWER = 41;
skill_AR_IMPROVE_CONCENTRATION = 42;
skill_AR_IC = 42;
skill_AR_ARROW_CRAFTING = 43;
skill_AR_ARROW_REPEL = 44;
// Mage
skill_MA_INCREASED_SP_RECOVERY = 45;
skill_MA_NAPALM_BEAT = 46;
skill_MA_SOUL_STRIKE = 47;
skill_MA_SAFETY_WALL = 48;
skill_PR_SAFETY_WALL = 48;
skill_MA_STONE_CURSE = 49;
skill_MA_SIGHT = 50;
skill_MA_FIRE_BOLT = 51;
skill_MA_FIRE_BALL = 52;
skill_MA_FIRE_WALL = 53;
skill_MA_COLD_BOLT = 54;
skill_MA_FROST_DIVER = 55;
skill_MA_LIGHTNING_BOLT = 56;
skill_MA_THUNDER_STORM = 57;
skill_MA_ENERGY_COAT = 58;

// Merchent
skill_ME_ENLARGE_WEIGHT_LIMIT = 59;
skill_ME_DISCOUNT = 60;
skill_ME_OVERCHARGE = 61;
skill_ME_PUSHCART = 62;
skill_ME_ITEM_APPRAISAL = 63;
skill_ME_VENDING = 64;
skill_ME_MAMMONITE = 65;
skill_ME_CART_REVOLUTION = 66;
skill_ME_CHANGE_CART = 67;
skill_ME_CRAZY_UPROAR = 68;

skill_KN_SPEAR_MASTERY = 69;
skill_KN_PIERCE = 70;
skill_KN_SPEAR_STAB = 71;
skill_KN_SPEAR_BOOMERANG = 72;
skill_KN_BRANDISH_SPEAR = 73;
skill_KN_TWOHAND_QUICKEN = 74;
skill_KN_AUTO_COUNTER = 75;
skill_KN_BOWLING_BASH = 76;
skill_KN_PECO_PECO_RIDE = 77;
skill_KN_CAVALIER_MASTERY = 78;

skill_AS_RIGHTHAND_MASTERY = 79;
skill_AS_LEFTHAND_MASTERY = 80;
skill_AS_KATAR_MASTERY = 81;
skill_AS_CLOAKING = 82;
skill_AS_CLOAK = 82;
skill_AS_SONIC_BLOW = 83;
skill_AS_SBLOW = 83;
skill_AS_GRIMTOOTH = 84;
skill_AS_ENCHANT_POISON = 85;
skill_AS_POISON_REACT = 86;
skill_AS_VENOM_DUST = 87;
skill_AS_VENOM_SPLASHER = 88;

skill_PR_MACE_MASTERY = 89;
skill_PR_IMPOSITIO_MANUS = 90;
skill_PR_IMPO = 90;
skill_PR_SUFFRAGIUM = 91;
skill_PR_SUFF = 91;
skill_PR_ASPERSIO = 92;
skill_PR_ASP = 92;
skill_PR_BS_SACRIMENTI = 93;
skill_PR_BSS = 93;
skill_PR_SANCTUARY = 94;
skill_PR_SANC = 94;
skill_PR_STATUS_RECOVERY = 95;
skill_PR_STAT_REC = 95;
skill_PR_SLOW_POISON = 96;
skill_PR_RESSURECTION = 97;
skill_PR_RESS = 97;
skill_PR_KYRIE_ELEISON = 98;
skill_PR_KYRIE = 98;
skill_PR_MAGNIFICAT = 99;
skill_PR_MAGNI = 99;
skill_PR_GLORIA = 100;
skill_PR_LEX_DIVINA = 101;
skill_PR_LEX_D = 101;
skill_PR_TURN_UNDEAD = 102;
skill_PR_TU = 102;
skill_PR_LEX_AETERNA = 103;
skill_PR_LEX_A = 103;
skill_PR_MAGNUS_EXORCISMUS = 104;
skill_PR_ME = 104;

skill_HU_SKID_TRAP = 105;
skill_HU_LAND_MINE = 106;
skill_HU_ANKLE_SNARE = 107;
skill_HU_FLASHER = 108;
skill_HU_SHOCKWAVE_TRAP = 109;
skill_HU_SANDMAN = 110;
skill_HU_FREEZING_TRAP = 111;
skill_HU_BLAST_MINE = 112;
skill_HU_CLAYMORE_TRAP = 113;
skill_HU_CLAYMORE = 113;
skill_HU_REMOVE_TRAP = 114;
skill_HU_TALKIE_BOX = 115;
skill_HU_BEAST_BANE = 116;
skill_HU_FALCONRY_MASTERY = 117;
skill_HU_BLITZ_BEAT = 118;
skill_HU_STEEL_CROW = 119;
skill_HU_DETECT = 120;
skill_HU_SPRING_TRAP = 121;

skill_WI_FIRE_PILLAR = 122;
skill_WI_SENSE = 123;
skill_SA_SENSE = 123;
skill_WI_SIGHTRASHER = 124;
skill_WI_METEOR_STORM = 125;
skill_WI_MS = 125;
skill_WI_JUPITEL_THUNDER = 126;
skill_WI_JT = 126;
skill_WI_LORD_OF_VERMILLION = 127;
skill_WI_LOV = 127;
skill_WI_WATER_BALL = 128;
skill_WI_WBALL = 128;
skill_WI_ICE_WALL = 129;
skill_WI_IWALL = 129;
skill_WI_FROST_NOVA = 130;
skill_WI_STORM_GUST = 131;
skill_WI_SG = 131;
skill_WI_EARTH_SPIKE = 132;
skill_WI_HEAVENS_DRIVE = 133;
skill_WI_HD = 133;
skill_SA_HEAVENS_DRIVE = 133;
skill_SA_HD = 133;
skill_WI_QUAGMIRE = 134;
skill_WI_QUAG = 134;

skill_BS_IRON_TEMPERING = 135;
skill_BS_STEEL_TEMPERING = 136;
skill_BS_ENCHANTEDSTONE_CRAFT = 137;
skill_BS_ORIDECON_RESEARCH = 138;
skill_BS_SMITH_DAGGER = 139;
skill_BS_SMITH_SWORD = 140;
skill_BS_SMITH_TWO_HANDED_SWORD = 141;
skill_BS_SMITH_AXE = 142;
skill_BS_SMITH_MACE = 143;
skill_BS_SMITH_KNUCKLEBRACE = 144;
skill_BS_SMITH_SPEAR = 145;
skill_BS_HILT_BINDING = 146;
skill_BS_ORE_DISCOVERY = 147;
skill_BS_WEAPONRY_RESEARCH = 148;
skill_BS_WEAPON_REPAIR = 149;
skill_BS_SKIN_TEMPERING = 150;
skill_BS_HAMMER_FALL = 151;
skill_BS_ANDRENALINE_RUSH = 152;
skill_BS_WEAPON_PERFECTION = 153;
skill_BS_POWER_THRUST = 154;
skill_BS_POWER_MAXIMIZE = 155;

skill_CR_FAITH = 156;
skill_CR_GUARD = 157;
skill_CR_SMITE = 158;
skill_CR_SHIELD_BOOMERANG = 159;
skill_CR_SHIELD_REFLECT = 160;
skill_CR_HOLY_CROSS = 161;
skill_CR_GRAND_CROSS = 162;
skill_CR_SACRIFICE = 163; // DEVO !!!
skill_CR_SACRI = 163;
skill_CR_DEVOTION = 163;
skill_CR_DEVO = 163;
skill_CR_RESISTANT_SOULS = 164;
skill_CR_DEFENDING_AURA = 165;
skill_CR_SPEAR_QUICKEN = 166;

skill_RG_SNATCH = 167;
skill_RG_MUG = 168;
skill_RG_BACK_STAB = 169;
skill_RG_STALK = 170;
skill_RG_SIGHTLESS_MIND = 171;
skill_RG_DIVEST_WEAPON = 172;
skill_RG_DIVEST_SHIELD = 173;
skill_RG_DIVEST_ARMOR = 174;
skill_RG_DIVEST_HELM = 175;
skill_RG_GANK = 176;
skill_RG_SCRIBBLE = 177;
skill_RG_PIECE = 178;
skill_RG_REMOVER = 179;
skill_RG_SLYNESS = 180;
skill_RG_HAGGLE = 181;
skill_RG_INTIMIDATE = 182;

skill_MO_IRON_FIST = 183;
skill_MO_SPIRITUAL_CADENCE = 184;
skill_MO_SUMMON_SPIRIT_SPHERE = 185;
skill_MO_SPIRITUAL_SPHERE_ABSORPTION = 186;
skill_MO_ABSORB_SPHERE = 186;
skill_MO_RAGING_TRIFECTA_BLOW = 187;
skill_MO_TRIPLE_STRIKE = 187;
skill_MO_RAGING_QUADRUPLE_BLOW = 188;
skill_MO_RAGING_THRUST = 189;
skill_MO_SNAP = 190;
skill_MO_FLEE = 191;
skill_MO_THROW_SPIRIT_SPHERES = 192;
skill_MO_TSS = 192;
skill_MO_FINGER_OFFENSIVE = 192;
skill_MO_FO = 192;
skill_MO_OCCULT_IMPACTION = 193;
skill_MO_OI = 193;
skill_MO_INVESTIGATE = 193;
skill_MO_INVEST = 193;
skill_MO_ROOT = 194;
skill_MO_FURY = 195;
skill_MO_MENTAL_STRENGTH = 196;
skill_MO_STEEL_BODY = 196;
skill_MO_GUILLOTINE_FIST = 197;

skill_BA_MUSIC_LESSONS = 198;
skill_BA_MELODY_STRIKE = 199;
skill_BA_UNCHAINED_SERENADE = 200;
skill_BA_UNBARRING_OCTAVE = 201;
skill_BA_FROST_JOKE = 201;
skill_BA_PERFECT_TABLATURE = 202;
skill_BA_IMPRESSIVE_RIFT = 203;
skill_BA_MAGIC_STRINGS = 204;
skill_BA_STRINGS = 204;
skill_BA_BRAGI = 204;
skill_BA_SONG_OF_LUTIE = 205;

skill_DA_DANCE_LESSONS = 206;
skill_DA_SLINGING_ARROW = 207;
skill_DA_HIP_SHAKER = 208;
skill_DA_DAZZLER = 209;
skill_DA_SCREAM = 209;
skill_DA_FOCUS_BALLET = 210;
skill_DA_SLOW_GRACE = 211;
skill_DA_LADY_LUCK = 212;
skill_DA_GYPSYS_KISS = 213;

skill_BD_AMP = 214;
skill_BD_ENCORE = 215;
skill_BD_LULLABY = 216;
skill_BD_MENTAL_SENSING = 217;
skill_BD_DOWN_TEMPO = 218;
skill_BD_BATTLE_THEME = 219;
skill_BD_HARMONIC_LICK = 220;
skill_BD_CLASSICAL_PLUCK = 221;
skill_BD_POWER_CORD = 222;
skill_BD_ACOUSTIC_RHYTHM = 223;

skill_SA_STUDY = 224;
skill_SA_CAST_CANCEL = 225;
skill_SA_MAGIC_ROD = 226;
skill_SA_SPELL_BREAKER = 227;
skill_SA_FREE_CAST = 228;
skill_SA_HINDSIGHT = 229;
skill_SA_AUTOCAST = 229;
skill_SA_ENDOW_BLAZE = 230;
skill_SA_ENDOW_TSUNAMI = 231;
skill_SA_ENDOW_TORNADO = 232;
skill_SA_ENDOW_QUAKE = 233;
skill_SA_ENDOW_FIRE = 230;
skill_SA_ENDOW_WATER = 231;
skill_SA_ENDOW_WIND = 232;
skill_SA_ENDOW_EARTH = 233;
skill_SA_DRAGONOLOGY = 234;
skill_SA_VOLCANO = 235;
skill_SA_DELUGE = 236;
skill_SA_WHIRLWIND = 237;
skill_SA_MAGNETIC_EARTH = 238;
skill_SA_LAND_PROTECTOR = 238;
skill_SA_DISPELL = 239;
skill_SA_HOCUS_POCUS = 240;

skill_AL_AXE_MASTERY = 241;
skill_AL_POTION_RESEARCH = 242;
skill_AL_PREPARE_POTION = 243;
skill_AL_ACID_TERROR = 244;
skill_AL_POTION_PITCHER = 245;
skill_AL_SUMMON_FLORA = 246;
skill_AL_SUMMON_MARINE_SPHERE = 247;
skill_AL_BOMB = 248;
skill_AL_ALCHEMICAL_WEAPON = 249;
skill_AL_SYNTHESIZED_SHIELD = 250;
skill_AL_SYNTHETIC_ARMOR = 251;
skill_AL_BIOCHEMICAL_HELM = 252;

skill_SN_FURY = 253;

skill_LK_AURA_BLADE = 254;
skill_LK_PARRYING = 255;
skill_LK_SPEAR_DYNAMO = 256;
skill_LK_TENSION_RELAX = 257;
skill_LK_FRENZY = 258;
skill_LK_BERSERK = 258;
skill_LK_CLASHING_SPIRAL = 259;
skill_LK_SPIRAL_PIERCE = 259;
skill_LK_TRAUMATIC_BLOW = 260;
skill_LK_VITAL_STRIKE = 261;

skill_AX_ADVANCED_KATAR_MASTERY = 262;
skill_AX_SOUL_DESTROYER = 263;
skill_AX_METEOR_ASSAULT = 264;
skill_AX_CREATE_DEADLY_POISON = 265;
skill_AX_ENCHANT_DEADLY_POISON = 266;

skill_HP_ASSUMPTIO = 267;
skill_HP_BASILICA = 268;
skill_HP_MEDIATIO = 269;

skill_SN_FALCON_EYES = 270;
skill_SN_FALCON_ASSAULT = 271;
skill_SN_FOCUSED_ARROW_STRIKE = 272;
skill_SN_WIND_WALK = 273;

skill_HW_SOUL_DRAIN = 274;
skill_HW_STAVE_CRASHER = 275;
skill_HW_MYSTICAL_AMPLIFICATION = 276;
skill_HW_NAPALM_VULCAN = 277;

skill_MS_SHATTERING_STRIKE = 278;
skill_MS_COIN_CRAFT = 279;
skill_MS_NUGGET_CRAFT = 280;
skill_MS_CART_BOOST = 281;
skill_MS_BATTLE_MACHINE_CRAFT = 282;

skill_PA_GLORIA_DOMINI = 283;
skill_PA_PRESSURE = 283;
skill_PA_MARTYR_RECONING = 284;
skill_PA_BATTLE_CHANT = 285;

skill_ST_STEALTH = 286;
skill_ST_COUNTER_INSTINCT = 287;

skill_CH_RAGING_PALM_STRIKE = 288;
skill_CH_GLACIER_FIST = 289;
skill_CH_CHAIN_CRUSH_COMBO = 290;
skill_CH_ZEN = 291;

skill_CG_ARROW_VULCAN = 292;
skill_CG_SHELTERING_BLISS = 293;
skill_CG_MARIONETTE_CONTROL = 294;

skill_PR_INDULGE = 295;
skill_PR_SOUL_EXHALE = 296;
skill_PR_SOUL_SIPHON = 297;
skill_PR_MIND_BREAKER = 298;

skill_BC_ALCHEMY = 299;
skill_BC_POTION_SYNTHESIS = 300;

skill_ALL_TOMAHAWK_THROWING = 302;

skill_MON_PULSE_STRIKE = 303;
skill_ALL_BERSERK_POTION_PITCHER = 304;
skill_ALL_BERSERK_PITCHER = 304;
skill_TKK_FLYING_KICK_SPRINT = 305;
skill_AS_VENOM_KNIFE = 306;
skill_HU_FANTASTIC_ARROW = 307;
skill_KN_CHARGE_ATTACK = 308;
skill_ALL_NO_DEATH_BONUS = 309;
skill_ALL_MARRIAGE_STATUS = 310;
skill_BS_SMITHS = 311;
skill_MON_DARK_STRIKE = 312;
skill_PR_FIBER_LOCK = 313;
skill_PR_SPIDER_WEB = 313;
skill_SA_ELEMENTAL_CHANGE = 314;
skill_HEAT = 317;
skill_HEAT_WALL = 318;
skill_HEAVENS_DRIVE2 = 319;
skill_WATER_BALL2 = 320;

skill_MO_MAX_GUILLOTINE_FIST = 321;
skill_PR_FORESIGHT = 322;
skill_PR_MEMORIZE = 322;
skill_PR_MEMO = 322;
skill_AX_DEADLY_POISON_CONSUMED = 323;
skill_PA_RAPID_SMITING = 324;
skill_HW_GRAVITY_FIELD = 325;
skill_MS_HIGH_SPEED_CART_RAM = 326;
skill_MS_MAXIMUM_POWER_THUST = 327;
skill_BC_ACID_DEMONSTRATION = 328;

skill_TK_SPRINT = 329;
skill_TK_TORNADO_STANCE = 330;
skill_TK_TORNADO_KICK = 331;
skill_TK_HEEL_DROP_STANCE = 332;
skill_TK_HEEL_DROP = 333;
skill_TK_ROUNDHOUSE_STANCE = 334;
skill_TK_ROUNDOUSE = 335;
skill_TK_COUNTER_KICK_STANCE = 336;
skill_TK_COUNTER_KICK = 337;
skill_TK_TUMBLING = 338;
skill_TK_FLYING_KICK = 339;
skill_TK_PEACEFUL_BREAK = 340;
skill_TK_HAPPY_BREAK = 341;
skill_TK_KIHOP = 342;
skill_TK_LEAP = 343;
skill_TK_TAEKWON_MISSION = 344;
skill_TK_TAEKWON_RANKER = 345;
skill_TK_MILD_WIND = 346;

skill_TKM_SOLAR_LUNAR_AND_STELLAR_PERCEPTION = 347;
skill_TKM_PERCEPTION = 347;
skill_TKM_SOLAR_HEAT = 348;
skill_TKM_LUNAR_HEAT = 349;
skill_TKM_STELLAR_HEAT = 350;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_OPPOSITION = 351;
skill_TKM_OPPOSITION = 351;
skill_TKM_SOLAR_WRATH = 352;
skill_TKM_LUNAR_WRATH = 353;
skill_TKM_STELLAR_WRATH = 354;
skill_TKM_SOLAR_PROTECTION = 355;
skill_TKM_LUNAR_PROTECTION = 356;
skill_TKM_STELLAR_PROTECTION = 357;
skill_TKM_SOLAR_BLESSINGS = 358;
skill_TKM_LUNAR_BLESSINGS = 359;
skill_TKM_STELLAR_BLESSINGS = 360;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_SHADOW = 361;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_TEAM_UP = 362;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_COURIER = 363;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_UNION = 364;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_MIRACLE = 365;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_ANGEL = 366;
skill_TKM_SHADOW = 361;
skill_TKM_TEAM_UP = 362;
skill_TKM_COURIER = 363;
skill_TKM_UNION = 364;
skill_TKM_MIRACLE = 365;
skill_TKM_ANGEL = 366;
skill_TKM_SOLAR_LUNAR_AND_STELLAR_BLESSINGS = 367;
skill_TKM_BLESSINGS = 367;

skill_SL_KAIZEL = 368;
skill_SL_KAAHI = 369;
skill_SL_KAUPE = 370;
skill_SL_KAITE = 371;
skill_SL_KAINA = 372;
skill_SL_ESTIN = 373;
skill_SL_ESTUN = 374;
skill_SL_ESMA = 375;
skill_SL_ESWOO = 376;
skill_SL_ESKA = 377;
skill_SL_ESKE = 378;

skill_TK_SPRINT_STR_STATE = 379;
skill_TK_KIHOP_PARTY_BONUS = 380;
skill_TK_KIHOP_PARTY = 380;

skill_AS_SONIC_ACCELERATION = 381;
skill_MO_EXCRUCIATING_PALM = 382;
skill_RG_CLOSE_CONFINE = 383;
skill_CR_SHIELD_BOOMERANG_SL = 384;
skill_SN_SUPER_NOVICE_SPIRIT = 385;
skill_KN_ONE_HAND_QUICKEN = 386;
skill_AC_HOLY_LIGHT_SL = 387;
skill_PR_HOLY_LIGHT_SL = 387;
skill_AS_SONIC_BLOW_SL = 388;
skill_AS_SBLOW_SL = 388;
skill_BS_FULL_ANDRENALINE_RUSH = 389;
skill_HU_HUNTER_SPIRIT = 390;
skill_HU_BEAST_STRAFING = 391;
skill_FIRST_TRANSCENDENT_SPIRIT = 392;

skill_NIN_DAGGER_THROWING_PRACTICE = 393;
skill_NIN_THROW_DAGGER = 394;
skill_NIN_THROW_KUNAI = 395;
skill_NIN_THROW_HUUMA_SHURIKEN = 396;
skill_NIN_THROW_COINS = 397;
skill_NIN_FLIP_TATAMI = 398;
skill_NIN_SHADOW_LEAP = 399;
skill_NIN_HAZE_SLASHER = 400;
skill_NIN_SHADOW_SLASH = 401;
skill_NIN_CICADA_SKIN_SHED = 402;
skill_NIN_MIRROR_IMAGE = 403;
skill_NIN_NINJA_AURA = 404;
skill_NIN_KILLING_STRIKE = 405;
skill_NIN_NINJA_MASTERY = 406;
skill_NIN_FLAMING_PETALS = 407;
skill_NIN_BLAZE_SHIELD = 408;
skill_NIN_EXPLODING_DRAGON = 409;
skill_NIN_FREEZING_SPEAR = 410;
skill_NIN_WATERY_EVASION = 411;
skill_NIN_SNOW_FLAKE_DRAFT = 412;
skill_NIN_WIND_BLADE = 413;
skill_NIN_LIGHTNING_JOLT = 414;
skill_NIN_FIRST_WIND = 415;

skill_GS_COIN_FLIP = 416;
skill_GS_COING_FLING = 417;
skill_GS_TRIPLE_ACTION = 418;
skill_GS_BULLS_EYE = 419;
skill_GS_LAST_STAND = 420;
skill_GS_GUNSLINGER_PANIC = 421;
skill_GS_INCREASE_ACCURACY = 422;
skill_GS_MAGICAL_BULLET = 423;
skill_GS_CRACKER = 424;
skill_GS_SINGLE_ACTION = 425;
skill_GS_SNAKE_EYES = 426;
skill_GS_CHAIN_ACTION = 427;
skill_GS_TRIGGER_HAPPY_SHOT = 428;
skill_GS_DESPERADO = 429;
skill_GS_TRACKING = 430;
skill_GS_DISARM = 431;
skill_GS_WOUNDING_SHOT = 432;
skill_GS_GATLING_FEVER = 433;
skill_GS_CROWD_CONTROL_SHOT = 434;
skill_GS_FULL_BLAST = 435;
skill_GS_SPREAD_SHOT = 436;
skill_GS_GUNSLINGER_MINE = 437;

skill_NIN_KILLING_STRIKE_MAX = 438;

skill_MON_POWER_UP = 439;
skill_MON_AGI_UP = 440;
skill_MON_CHANGE_TO_ELEMENTAL = 441;
skill_MON_STONE_SKIN = 442;
skill_MON_ANTI_MAGIC = 443;
skill_MON_KEEPING = 444;

skill_RUN_ENCHANT_BLADE = 445;
skill_RUN_SONIC_WAVE = 446;
skill_RUN_DEATH_BOUND = 447;
skill_RUN_HUNDRED_SPEAR = 448;
skill_RUN_WIND_CUTTER = 449;
skill_RUN_IGNITION_BREAK = 450;
skill_RUN_PHANTOM_THRUST = 451;
skill_RUN_DRAGON_TRAINING = 452;
skill_RUN_DRAGON_BREATH = 453;
skill_RUN_DRAGON_HOWLING = 454;
skill_RUN_RUNE_MASTERY = 455;
skill_RUN_GIANT_GROWTH = 456;
skill_RUN_VITALITY_ACTIVATION = 457;
skill_RUN_STORM_BLAST = 458;
skill_RUN_STONEHARD_SKIN = 459;
skill_RUN_FIGHTING_SPIRIT = 460;
skill_RUN_ABUNDANCE = 461;
skill_RUN_CRUSH_STRIKE = 462;
skill_RUN_REFRESH = 463;
skill_RUN_MILLENIUM_SHIELD = 464;

skill_GLT_VENOM_IMPRESS = 465;
skill_GLT_CROSS_IMPACT = 466;
skill_GLT_DARK_ILLUSION = 467;
skill_GLT_RESEARCH_NEW_POISON = 468;
skill_GLT_CREATE_NEW_POISON = 469;
skill_GLT_ANTIDOTE = 470;
skill_GLT_POISONING_WEAPON = 471;
skill_GLT_WEAPON_BLOCKING = 472;
skill_GLT_COUNTER_SLASH = 473;
skill_GLT_WEAPON_CRUSH = 474;
skill_GLT_VENOM_PRESSURE = 475;
skill_GLT_POISON_SMOKE = 476;
skill_GLT_CLOAKING_EXCEED = 477;
skill_GLT_PHANTOM_MENACE = 478;
skill_GLT_HALLUCINATION_WALK = 479;
skill_GLT_ROLLING_CUTTER = 480;
skill_GLT_CROSS_RIPPER_SLASHER = 481;
skill_GLT_POISON_PARALYZE = 482;
skill_GLT_POISON_PYREXIA = 483;
skill_GLT_POISON_DISHEART = 484;
skill_GLT_POISON_LEECH_END = 485;
skill_GLT_POISON_VENOM_BLEED = 486;
skill_GLT_POISON_MAGIC_MUSHROOM = 487;
skill_GLT_POISON_TOXIN = 488;
skill_GLT_POISON_OBLIVION_CURSE = 489;

skill_ABI_JUDEX = 490;
skill_ABI_ANCILLA = 491;
skill_ABI_ADORAMUS = 492;
skill_ABI_CLEMENTIA = 493;
skill_ABI_CANTO_CANDIDUS = 494;
skill_ABI_COLUCEO_HEAL = 495;
skill_ABI_EPICLESIS = 496;
skill_ABI_PRAEFATIO = 497;
skill_ABI_ORATIO = 498;
skill_ABI_LAUDA_AGNUS = 499;
skill_ABI_LAUDA_RAMUS = 500;
skill_ABI_EUCHARISTICA = 501;
skill_ABI_RENOVATIO = 502;
skill_ABI_HIGHNESS_HEAL = 503;
skill_ABI_CLEARANCE = 504;
skill_ABI_EXPIATIO = 505;
skill_ABI_DUPLE_LIGHT = 506;
skill_ABI_SILENTIUM = 507;
skill_ABI_SACRAMENT = 508;

skill_RAN_RANGER_MAIN = 509;
skill_RAN_CAMOUFLAGE = 510;
skill_RAN_AIMED_BOLT = 511;
skill_RAN_ARROW_STORM = 512;
skill_RAN_FEAR_BREEZE = 513;
skill_RAN_DETONATOR = 514;
skill_RAN_FIRING_TRAP = 515;
skill_RAN_ICEBOUND_TRAP = 516;
skill_RAN_ELECTRIC_SHOCKER = 517;
skill_RAN_RESEARCH_TRAP = 518;
skill_RAN_MARGENTA_TRAP = 519;
skill_RAN_COBALT_TRAP = 520;
skill_RAN_MAIZE_TRAP = 521;
skill_RAN_VERDURE_TRAP = 522;
skill_RAN_CLUSTER_BOMB = 523;
skill_RAN_WARG_MASTERY = 524;
skill_RAN_WARG_RIDER = 525;
skill_RAN_WARG_DASH = 526;
skill_RAN_WARG_BITE = 527;
skill_RAN_TOOTH_OF_WARG = 528;
skill_RAN_WARG_STRIKE = 529;
skill_RAN_KEEN_NOSE = 530;

skill_WAR_READING_SPELLBOOK = 531;
skill_WAR_FREEZING_SPELL = 532;
skill_WAR_RADIUS = 533;
skill_WAR_DRAIN_LIFE = 534;
skill_WAR_SOUL_EXPANSION = 535;
skill_WAR_WHITE_IMPRISON = 536;
skill_WAR_STASIS = 537;
skill_WAR_RECOGNIZED_SPELL = 538;
skill_WAR_MARSH_OF_ABYSS = 539;
skill_WAR_CRIMSON_ROCK = 540;
skill_WAR_HELL_INFERNO = 541;
skill_WAR_COMET = 542;
skill_WAR_FROSTY_MISTY = 543;
skill_WAR_FROST_MISTY = 543;
skill_WAR_JACK_FROST = 544;
skill_WAR_CHAIN_LIGHTNING = 545;
skill_WAR_SIENNA_EXECRATE = 546;
skill_WAR_EARTH_STRAIN = 547;
skill_WAR_RELEASE = 548;
skill_WAR_SUMMON_FIRE_BALL = 549;
skill_WAR_SUMMON_WATER_BALL = 550;
skill_WAR_SUMMON_LIGHTNING_BALL = 551;
skill_WAR_SUMMON_STONE = 552;
skill_WAR_TETRA_VORTEX = 553;

skill_MEC_AXE_TRAINING = 554;
skill_MEC_AXE_TORNADO = 555;
skill_MEC_AXE_BOOMERANG = 556;
skill_MEC_POWER_SWING = 557;
skill_MEC_RESEARCH_FIRE_EARTH = 558;
skill_MEC_FAW_SILVER_SNIPER = 559;
skill_MEC_FAW_MAGIC_DECOY = 560;
skill_MEC_FAW_REMOVAL = 561;
skill_MEC_MAGIC_GEAR_LICENSE = 562;
skill_MEC_REPAIR = 563;
skill_MEC_ACCELERATION = 564;
skill_MEC_HOVERING = 565;
skill_MEC_FRONT_SIDE_SLIDE = 566;
skill_MEC_BACK_SIDE_SLIDE = 567;
skill_MEC_BOOST_KNUCKLE = 568;
skill_MEC_PILE_BUNKER = 569;
skill_MEC_VULCAN_ARM = 570;
skill_MEC_FLAME_LAUNCHER = 571;
skill_MEC_COLD_SLOWER = 572;
skill_MEC_ARM_CANNON = 573;
skill_MEC_MAINFRAME_RESTRUCTURE = 574;
skill_MEC_SELF_DESTRUCTION = 575;
skill_MEC_EMERGENCY_COOL = 576;
skill_MEC_MAGNETIC_FIELD = 577;
skill_MEC_NEUTRAL_BARRIER = 578;
skill_MEC_SHAPE_SHIFT = 579;
skill_MEC_INFRARED_SCAN = 580;
skill_MEC_ANALYZE = 581;
skill_MEC_STEALTH_FIELD = 582;

skill_ROY_SPEAR_CANNON = 583;
skill_ROY_VANISHING_POINT = 584;
skill_ROY_TRAMPLE = 585;
skill_ROY_SHIELD_PRESS = 586;
skill_ROY_REFLECT_DAMAGE = 587;
skill_ROY_PINPOINT_ATTACK = 588;
skill_ROY_FORCE_OF_VANGUARD = 589;
skill_ROY_RAGE_BURST = 590;
skill_ROY_SHIELD_SPELL = 591;
skill_ROY_EXCEED_BREAK = 592;
skill_ROY_OVERBRAND = 593;
skill_ROY_PRESTIGE = 594;
skill_ROY_BANDING = 595;
skill_ROY_MOON_SLASHER = 596;
skill_ROY_RAY_OF_GENESIS = 597;
skill_ROY_PIETY = 598;
skill_ROY_EARTH_DRIVE = 599;
skill_ROY_HESPERUS_LIT = 600;
skill_ROY_INSPIRATION = 601;

skill_SHA_FATAL_MENACE = 602;
skill_SHA_REPRODUCE = 603;
skill_SHA_AUTO_SHADOW_SPELL = 604;
skill_SHA_SHADOW_FORM = 605;
skill_SHA_TRIANGLE_SHOT = 606;
skill_SHA_STRIP_ACCESSORY = 607;
skill_SHA_INVISIBILITY = 608;
skill_SHA_DEADLY_INFECT = 609;
skill_SHA_BODY_PAINTING = 610;
skill_SHA_MASQUERADE_ENERVATION = 611;
skill_SHA_MASQUERADE_GLOOMY = 612;
skill_SHA_MASQUERADE_IGNORANCE = 613;
skill_SHA_MASQUERADE_LAZINESS = 614;
skill_SHA_MASQUERADE_WEAKNESS = 615;
skill_SHA_MASQUERADE_UNLUCKY = 616;
skill_SHA_MAN_HOLE = 617;
skill_SHA_DIMENSION_DOOR = 618;
skill_SHA_CHAOS_PANIC = 619;
skill_SHA_MAELSTROM = 620;
skill_SHA_BLOODY_LUST = 621;
skill_SHA_FEINT_BOMB = 622;

skill_SUR_DRAGON_COMBO = 623;
skill_SUR_SKY_NET_BLOW = 624;
skill_SUR_EARTH_SHAKER = 625;
skill_SUR_RAMPAGE_BLASTER = 626;
skill_SUR_KNUCKLE_ARROW = 627;
skill_SUR_FALLEN_EMPIRE = 628;
skill_SUR_TIGER_CANNON = 629;
skill_SUR_GATE_OF_HELL = 630;
skill_SUR_CRESCENT_ELBOW = 631;
skill_SUR_WINDMILL = 632;
skill_SUR_CURSED_CIRCLE = 633;
skill_SUR_LIGHTNING_WALK = 634;
skill_SUR_RISING_DRAGON = 635;
skill_SUR_LION_HOWLING = 636;
skill_SUR_LIGHTENING_RIDE = 637;
skill_SUR_GENTLE_TOUCH_SILENCE = 638;
skill_SUR_GENTLE_TOUCH_CURE = 639;
skill_SUR_GENTLE_TOUCH_ENERGY_GAIN = 640;
skill_SUR_GENTLE_TOUCH_CHANGE = 641;
skill_SUR_GENTLE_TOUCH_REVITALIZE = 642;
skill_SUR_POWER_ABSORB = 643;
skill_SUR_POWER_IMPLANTATION = 644;

// MAESTRO AND WANDERER
skill_MIN_WINDMILL = 645;
skill_MIN_ECHO_SONG = 646;
skill_MIN_HARMONIZE = 647;
skill_WAN_SWING_DANCE = 648;
skill_WAN_SYMPHONY = 649;
skill_WAN_MOONLIGHT = 650;

skill_MIWA_VOICE_LESSONS = 651;
skill_MIWA_REVERBERATION = 652;
skill_MIWA_DOMINION_IMPULSE = 653;
skill_MIWA_METALLIC_SOUND = 654;
skill_MIWA_GREAT_ECHO = 655;
skill_MIWA_SEVERE_RAINSTORM = 656;
skill_MIWA_DEEP_SLEEP_LULLABY = 657;
skill_MIWA_SONG_OF_DESPAIR = 658;
skill_MIWA_IMPROVISED_SONG = 659;
skill_MIWA_GLOOMY_SHYNESS = 660;
skill_MIWA_VOICE_OF_SIREN = 661;
skill_MIWA_CIRCLE_OF_NATURE = 662;
skill_MIWA_DEATH_VALLEY = 663;
skill_MIWA_DANCES_WITH_WARGS = 664;
skill_MIWA_SATURDAY_NIGHT_FEVER = 665;
skill_MIWA_SOUND_OF_DESTRUCTION = 666;
skill_MIWA_LERADS_DEW = 667;
skill_MIWA_WARCRY_FROM_BEYOND = 668;
skill_MIWA_UNLIMITED_HUMMING_VOICE = 669;
skill_MIWA_SONG_OF_MANA = 670;
skill_MIWA_SINKING_MELODY = 671;

// SORCERER
skill_SOR_FIRE_WALK = 672;
skill_SOR_ELECTRIC_WALK = 673;
skill_SOR_SPELL_FIST_FBOLT = 674;
skill_SOR_SPELL_FIST_CBOLT = 675;
skill_SOR_SPELL_FIST_LBOLT = 676;
skill_SOR_VACUUM_EXTREME = 677;
skill_SOR_PSYCHIC_WAVE = 678;
skill_SOR_CLOUD_KILL = 679;
skill_SOR_POISON_BUSTER = 680;
skill_SOR_STRIKING = 681;
skill_SOR_EARTH_GRAVE = 682;
skill_SOR_DIAMOND_DUST = 683;
skill_SOR_WARMER = 684;
skill_SOR_VARETYR_SPEAR = 685;
skill_SOR_ARRULLO = 686;

skill_GEN_SWORD_TRAINING = 687;
skill_GEN_CART_REMODELING = 688;
skill_GEN_CART_TORNADO = 689;
skill_GEN_CART_CANNON = 690;
skill_GEN_CART_BOOST = 691;
skill_GEN_THORN_TRAP = 692;
skill_GEN_BLOOD_SUCKER = 693;
skill_GEN_SPORE_EXPLOSION = 694;
skill_GEN_WALL_OF_THORNS = 695;
skill_GEN_CRAZY_WEED = 696;
skill_GEN_DEMONIC_FIRE = 697;
skill_GEN_FIRE_EXPANSION = 698;
skill_GEN_HELLS_PLANT = 699;
skill_GEN_HOWLING_OF_MANDRAGORA = 700;
skill_GEN_SLING_ITEM = 701;
skill_GEN_CHANGE_MATERIAL = 702;
skill_GEN_MIX_COOKING = 703;
skill_GEN_CREATE_BOMB = 704;
skill_GEN_SPECIAL_PHARMACY = 705;

skill_SOR_SPIRIT_CONTROL = 706;
skill_SOR_SUMMON_AGNI = 707;
skill_SOR_SUMMON_VENTUS = 708;
skill_SOR_SUMMON_AQUA = 709;
skill_SOR_SUMMON_TERRA = 710;
skill_SOR_INSIGNIA_FIRE = 711;
skill_SOR_INSIGNIA_WIND = 712;
skill_SOR_INSIGNIA_WATER = 713;
skill_SOR_INSIGNIA_EARTH = 714;
skill_SOR_SUMMON_TYPE = 715;
skill_SOR_SUMMON_LEVEL = 716;

skill_ROY_SHIELD_SPELL_ATK = 717;
skill_ROY_SHIELD_SPELL_MATK = 718;
skill_ROY_NUM_GUARDS = 719;
skill_ROY_OVERBRAND_OLD = 720;

skill_ALL_ODINS_POWER = 721;
skill_SG_FIBER_LOCK = 722;

skill_KAG_OVERTHROW = 723;
skill_KAG_CROSS_STRIKE = 724;
skill_KAG_16TH_NIGHT = 725;
skill_KAG_SPINTHROW_KUNAI = 726;
skill_KAG_SPIRIT_BREAKER = 727;
skill_KAG_HUUMA_SHURIKEN_STRIKE = 728;
skill_KAG_THROW_EXPLOSIVE_KUNAI = 729;
skill_KAG_SUMMON_ELEMENTAL_SEAL = 730; //ONE SKILL FOR EACH ELEMENT
skill_KAG_GET_ELEMENTAL_SEAL = 731;

skill_RUN_DRAGON_BREATH_WATER = 732;
skill_GLT_DARK_CLAW = 733;
skill_ABI_OFFERTORIUM = 734;
skill_RAN_NO_LIMITS = 735;
skill_WAR_INTENSE_TELEKINESIS = 736;
skill_MEC_LAVA_FLOW = 737;
skill_ROY_KINGS_GRACE = 738;
skill_SHA_EMERGENCY_ESCAPE = 739;
skill_SUR_FLASH_COMBO = 740;
skill_MIWA_FRIGGS_SONG = 741;
skill_SOR_ELEMENTAL_SHIELD = 742;
skill_GEN_HALLUCINATION_DRUG = 743;
skill_3RD_FULL_THROTTLE = 744;

skill_FINISH_DELIMITER = 999;
}

// Buffs
{
// Acolyte Buffs
var ksBlessing = 0;
var ksIncreaseAgi = 1;
var ksAngelus = 2;
var ksImposito = 3;
var ksSuffragium = 4;
var ksGloria = 5;
var ksAssumptio = 6;
var ksSpheres = 7;
var ksClementia = 8;
var ksCandidus = 9;
var ksExpiatio = 10;
var ksSacrament = 11;
var ksLaudaAgnus = 12;
var ksLaudaRamus = 13
var ksPPChange = 14;
var ksPPRevitalize = 15;
var ksSuraStrength = 16;
var ksSuraAgility = 17;
var ksSuraVitality = 18;
var ksSuraIntelligence = 19;
var ksSuraDexterity = 20;
var ksAcolyteBuffCount = 21;

// Performer Buffs
var ksBardSolo = 0;
var ksPerfectTablature = 1;
var ksImpressiveRiff = 2;
var ksMagicStrings = 3;
var ksSongOfLutie = 4;
var ksBardSoloLevel = 1;
var ksMusicLessons = 2;
var ksBardAgi = 3;
var ksBardInt = 4;
var ksBardDex = 5;
var ksBardVit = 6;
var ksMaestroSolo = 7;
var ksHarmonize = 1;
var ksEchoSong = 2;
var ksWindmillRush = 3;
var ksGloomyShynessM = 4;
var ksFriggsSongM = 5;
var ksMaestroSoloLevel = 8;
var ksMaestroVoiceLessons = 9;
var ksMaestroJobLevel = 10;

var ksDancerSolo = 11;
var ksFocusBallet = 1;
var ksSlowGrace = 2;
var ksLadyLuck = 3;
var ksGypsysKiss = 4;
var ksDancerSoloLevel = 12;
var ksDanceLessons = 13;
var ksDancerAgi = 14;
var ksDancerInt = 15;
var ksDancerDex = 16;
var ksDancerLuk = 17;
var ksWandererSolo = 18;
var ksSwingDance = 1;
var ksLoversSymphony = 2;
var ksMoonlightSerenade = 3;
var ksGloomyShynessW = 4;
var ksFriggsSongW = 5;
var ksWandererSoloLevel = 19;
var ksWandererVoiceLessons = 20;
var ksWandererJobLevel = 21;

var ksEnsemble = 22;
var ksAcousticRhythm = 1;
var ksBattleTheme = 2;
var ksDownTempo = 3;
var ksHarmonicLick = 4;
var ksMentalSensing = 5;
var ksBardEnsembleLevel = 23;
var ksDancerEnsembleLevel = 24;
var ksEnsembleLevel = 25;
var ksChorus = 26;
var ksDancesWithWargs = 1;
var ksSongOfMana = 2;
var ksLeradsDew = 3;
var ksSaturdayNightFever = 4;
var ksWarcryFromBeyond = 5;
var ksSinkingMelody = 6;
var ksChorusLevel = 27;
var ksNumPerformers = 28;

var ksMarionette = 29;
var ksPerformerStr = 30;
var ksPerformerAgi = 31;
var ksPerformerVit = 32;
var ksPerformerInt = 33;
var ksPerformerDex = 34;
var ksPerformerLuk = 35;

// Guild Skills
pass_IV_BAT_ORDER = 0;
pass_IV_GRE_LEADER = 1;
pass_IV_WOU_GLORY = 2;
pass_IV_SOU_COLD = 3;
pass_IV_SHA_EYES = 4;

// Battle Chant Effects
pass_V_STATS = 0;
pass_V_HP = 1;
pass_V_SP = 2;
pass_V_ATK = 3;
pass_V_HIT_FLEE = 4;
pass_V_DAMAGE = 5;

// otherBuffs
var ksElementField = 0;
var ksElementFieldLvl = 1;
var ksMurderBonus = 2;
var ksImproveConcentration = 3;
var ksMindBreaker = 4;
var ksProvoke = 5;
var ksBSS = 6;
var ksAdrenalineRush = 7;
var ksWeaponPerfection = 8;
var ksPowerThrust = 9;
var ksWindWalker = 10;
var ksMagnumBreak = 11;
var ksAloe = 12;
var ksResistantSouls = 13;
var ksStriking = 14;
var ksOdinsPower = 15;
var ksStrikingEndowBonus = 16;
// elemental field level values
var ksVolcano = 0;
var ksDeluge = 1;
var ksWhirlwind = 2;

// miscEffects
var ksPetEffects = 0;
var ksSupNovMarriage = 1;
var ksFirstTempEffect = 2;
var ksSecondTempEffect = 3;
var ksThirdTempEffect = 4;
var ksFourthTempEffect = 5;
var ksNumEnemies = 6;
var ksTransFirstSpirit = 7;
var ksNoCrit = 8;
var ksQuagmire = 9;
var ksAgiDown = 10;
var ksPoisoned = 11;
var ksCursed = 12;

// Usable Items
var ksSesamePastry = 0;
var ksHoneyPastry = 1;
var ksRainbowCake = 2;
var ksStrengthFood = 3;
var ksAgilityFood = 4;
var ksVitalityFood = 5;
var ksIntelligenceFood = 6;
var ksDexterityFood = 7;
var ksLuckFood = 8;
var ksBoxOfResentment = 9;
var ksBoxOfDrowsiness = 10;
var ksColdproof = 11;
var ksEarthproof = 12;
var ksFireproof = 13;
var ksThunderproof = 14;
var ksCastScrolls = 15;
var ksBoucheDeNoel = 16;
var ksRuneStrawberryCake = 17;
var ksSchwartzwaldPineJubilee = 18;
var ksArunafeltzDesertSandwich = 19;
var ksDurian = 20;
var ksCelermineJuice = 21;
var ksGuaranaCandy = 22;
var ksLuckyRiceCake = 23;
var ksMilitaryRationB = 24;
var ksMilitaryRationC = 25;
var ksPinkRation = 26;
var ksWhiteRation = 27;
var ksVitataFiveHundred = 28;
var ksAttackSpeed = 29;
var ksDistilledFightingSpirit = 39;
var ksIncreaseHP = 31;
var ksIncreaseSP = 32;
var ksVipBuffs = 33;
var ksAbrasive = 34;
var ksHolyElemental = 35;
var ksUndeadElemental = 36;

// battleEffects[%]
pass_VIII_PET = 0;
pass_VIII_BAT_MANUAL = 1;
pass_VIII_JOB_MANUAL = 2;
pass_VIII_VIPSTATUS = 3;
pass_VIII_ALL_STAT = 4;
pass_VIII_PAR_COUNT = 5;
pass_VIII_EXP_TAB = 6;
pass_VIII_SER_EXP = 7;
pass_VIII_TEMPI = 8;
pass_VIII_TEMPII = 9;
pass_VIII_TEMPIII = 10;
pass_VIII_TEMPIV = 11;
pass_VIII_NUM_ENEMY = 12;
//pass_VIII_ = 13; // not used
pass_VIII_SPE_ENVIRONMENT = 14;
pass_VIII_DEF_INVEST = 15;
pass_VIII_NO_CRIT = 16;
pass_VIII_ADV_SPIRIT = 17;
pass_VIII_BUC_NOEL = 18;
pass_VIII_RUNE_STRAW_BERRY_CAKE = 19;
pass_VIII_RUN_CAKE = 19;
pass_VIII_SCH_JUBILEE = 20;
pass_VIII_ARU_SANDWINCH = 21;
}

{ // Delay types
var ksDelayA = 0;
var ksDelayASPD = 1;
var ksDelayGlobal = 2;
var ksDelayAnimation = 3;
var ksDelayE = 4; //Martyr Reckoning?
var ksDelayF = 5; //TKM Heat?
var ksDelaySkillDuration = 6;
var ksDelayCooldown = 7;
}

{ // Temporary Effects - pass8[8~11] = % - TimeItemNumSearch(%)
temp_NONE = 0;
temp_ISILLA = 1;
temp_ICETITAN = 2;
temp_ATROCE = 3;
temp_ANOLIAN = 4;
temp_ALCHESET = 5;
temp_VAL_ASSADAMA = 6;
temp_IXIWINGS = 7;
temp_VANBERK = 8;
temp_ULFHEDINN = 9;
temp_ANGELICRING = 10;
temp_GLOR_GRENADE = 11;
temp_GLOR_JAMAD = 12;
temp_GLOR_TABLET = 13;
temp_GLOR_SHURI = 14;
temp_GLOR_BLOODY = 15;
temp_NAGHT_RED = 16;
temp_CUPROAR = 17;
temp_TWILIGHT = 18;
temp_SHAD_GUARD = 19;
temp_VAL_HUUMA = 20;
temp_NAGHT_BLUE = 21;
temp_SOL_GATLING = 22;
temp_SOL_GRNADE = 23;
temp_SOL_REVOL = 24;
temp_TAE_GOO = 25;
temp_BRAV_FIST = 26;
temp_NAGA = 27;
temp_NOBLE = 28;
temp_VIOLET = 29;
temp_BLO_EATER = 30;
temp_BLU_RIBBON = 31;
temp_HODREMLIN = 32;
temp_MMMANT = 33;
temp_ROFL = 34;
temp_LK_CARD = 35;
temp_SOL_SHOTGUN = 36;
temp_BINER = 37;
temp_ROUBEL = 38;
temp_CRONOS = 39;
temp_NEMESIS = 40;
}

{
var CONST_MAXLVL = 99;
var CONST_MAXLVL_THIRD = 175;
var CONST_MAXLVL_KAGOB_ENOVI = 160;
var CONST_MAXSTAT = 99;
var CONST_MAXSTAT_THIRD = 130;
var CONST_MAXSTAT_KAGOB_ENOVI = 125;
}