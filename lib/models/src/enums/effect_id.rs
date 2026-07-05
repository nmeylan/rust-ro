// Auto-generated from rathena `e_special_effects` (src/map/script.hpp).
#![allow(dead_code, non_camel_case_types, clippy::enum_clike_unportable_variant)]

use crate::enums::*;
use crate::enums::skill_enums::SkillEnum;

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EffectId {
    /// `EF_HIT1`
    #[value_string = "EF_HIT1"]
    #[value = 0]
    Hit1,
    /// `EF_HIT2`
    #[value_string = "EF_HIT2"]
    Hit2,
    /// `EF_HIT3`
    #[value_string = "EF_HIT3"]
    Hit3,
    /// `EF_HIT4`
    #[value_string = "EF_HIT4"]
    Hit4,
    /// `EF_HIT5`
    #[value_string = "EF_HIT5"]
    Hit5,
    /// `EF_HIT6`
    #[value_string = "EF_HIT6"]
    Hit6,
    /// `EF_ENTRY`
    #[value_string = "EF_ENTRY"]
    Entry,
    /// `EF_EXIT`
    #[value_string = "EF_EXIT"]
    Exit,
    /// `EF_WARP`
    #[value_string = "EF_WARP"]
    Warp,
    /// `EF_ENHANCE`
    #[value_string = "EF_ENHANCE"]
    Enhance,
    /// `EF_COIN`
    #[value_string = "EF_COIN"]
    Coin,
    /// `EF_ENDURE`
    #[value_string = "EF_ENDURE"]
    Endure,
    /// `EF_BEGINSPELL`
    #[value_string = "EF_BEGINSPELL"]
    Beginspell,
    /// `EF_GLASSWALL`
    #[value_string = "EF_GLASSWALL"]
    Glasswall,
    /// `EF_HEALSP`
    #[value_string = "EF_HEALSP"]
    Healsp,
    /// `EF_SOULSTRIKE`
    #[value_string = "EF_SOULSTRIKE"]
    Soulstrike,
    /// `EF_BASH`
    #[value_string = "EF_BASH"]
    Bash,
    /// `EF_MAGNUMBREAK`
    #[value_string = "EF_MAGNUMBREAK"]
    Magnumbreak,
    /// `EF_STEAL`
    #[value_string = "EF_STEAL"]
    Steal,
    /// `EF_HIDING`
    #[value_string = "EF_HIDING"]
    Hiding,
    /// `EF_PATTACK`
    #[value_string = "EF_PATTACK"]
    Pattack,
    /// `EF_DETOXICATION`
    #[value_string = "EF_DETOXICATION"]
    Detoxication,
    /// `EF_SIGHT`
    #[value_string = "EF_SIGHT"]
    Sight,
    /// `EF_STONECURSE`
    #[value_string = "EF_STONECURSE"]
    Stonecurse,
    /// `EF_FIREBALL`
    #[value_string = "EF_FIREBALL"]
    Fireball,
    /// `EF_FIREWALL`
    #[value_string = "EF_FIREWALL"]
    Firewall,
    /// `EF_ICEARROW`
    #[value_string = "EF_ICEARROW"]
    Icearrow,
    /// `EF_FROSTDIVER`
    #[value_string = "EF_FROSTDIVER"]
    Frostdiver,
    /// `EF_FROSTDIVER2`
    #[value_string = "EF_FROSTDIVER2"]
    Frostdiver2,
    /// `EF_LIGHTBOLT`
    #[value_string = "EF_LIGHTBOLT"]
    Lightbolt,
    /// `EF_THUNDERSTORM`
    #[value_string = "EF_THUNDERSTORM"]
    Thunderstorm,
    /// `EF_FIREARROW`
    #[value_string = "EF_FIREARROW"]
    Firearrow,
    /// `EF_NAPALMBEAT`
    #[value_string = "EF_NAPALMBEAT"]
    Napalmbeat,
    /// `EF_RUWACH`
    #[value_string = "EF_RUWACH"]
    Ruwach,
    /// `EF_TELEPORTATION`
    #[value_string = "EF_TELEPORTATION"]
    Teleportation,
    /// `EF_READYPORTAL`
    #[value_string = "EF_READYPORTAL"]
    Readyportal,
    /// `EF_PORTAL`
    #[value_string = "EF_PORTAL"]
    Portal,
    /// `EF_INCAGILITY`
    #[value_string = "EF_INCAGILITY"]
    Incagility,
    /// `EF_DECAGILITY`
    #[value_string = "EF_DECAGILITY"]
    Decagility,
    /// `EF_AQUA`
    #[value_string = "EF_AQUA"]
    Aqua,
    /// `EF_SIGNUM`
    #[value_string = "EF_SIGNUM"]
    Signum,
    /// `EF_ANGELUS`
    #[value_string = "EF_ANGELUS"]
    Angelus,
    /// `EF_BLESSING`
    #[value_string = "EF_BLESSING"]
    Blessing,
    /// `EF_INCAGIDEX`
    #[value_string = "EF_INCAGIDEX"]
    Incagidex,
    /// `EF_SMOKE`
    #[value_string = "EF_SMOKE"]
    Smoke,
    /// `EF_FIREFLY`
    #[value_string = "EF_FIREFLY"]
    Firefly,
    /// `EF_SANDWIND`
    #[value_string = "EF_SANDWIND"]
    Sandwind,
    /// `EF_TORCH`
    #[value_string = "EF_TORCH"]
    Torch,
    /// `EF_SPRAYPOND`
    #[value_string = "EF_SPRAYPOND"]
    Spraypond,
    /// `EF_FIREHIT`
    #[value_string = "EF_FIREHIT"]
    Firehit,
    /// `EF_FIRESPLASHHIT`
    #[value_string = "EF_FIRESPLASHHIT"]
    Firesplashhit,
    /// `EF_COLDHIT`
    #[value_string = "EF_COLDHIT"]
    Coldhit,
    /// `EF_WINDHIT`
    #[value_string = "EF_WINDHIT"]
    Windhit,
    /// `EF_POISONHIT`
    #[value_string = "EF_POISONHIT"]
    Poisonhit,
    /// `EF_BEGINSPELL2`
    #[value_string = "EF_BEGINSPELL2"]
    Beginspell2,
    /// `EF_BEGINSPELL3`
    #[value_string = "EF_BEGINSPELL3"]
    Beginspell3,
    /// `EF_BEGINSPELL4`
    #[value_string = "EF_BEGINSPELL4"]
    Beginspell4,
    /// `EF_BEGINSPELL5`
    #[value_string = "EF_BEGINSPELL5"]
    Beginspell5,
    /// `EF_BEGINSPELL6`
    #[value_string = "EF_BEGINSPELL6"]
    Beginspell6,
    /// `EF_BEGINSPELL7`
    #[value_string = "EF_BEGINSPELL7"]
    Beginspell7,
    /// `EF_LOCKON`
    #[value_string = "EF_LOCKON"]
    Lockon,
    /// `EF_WARPZONE`
    #[value_string = "EF_WARPZONE"]
    Warpzone,
    /// `EF_SIGHTRASHER`
    #[value_string = "EF_SIGHTRASHER"]
    Sightrasher,
    /// `EF_BARRIER`
    #[value_string = "EF_BARRIER"]
    Barrier,
    /// `EF_ARROWSHOT`
    #[value_string = "EF_ARROWSHOT"]
    Arrowshot,
    /// `EF_INVENOM`
    #[value_string = "EF_INVENOM"]
    Invenom,
    /// `EF_CURE`
    #[value_string = "EF_CURE"]
    Cure,
    /// `EF_PROVOKE`
    #[value_string = "EF_PROVOKE"]
    Provoke,
    /// `EF_MVP`
    #[value_string = "EF_MVP"]
    Mvp,
    /// `EF_SKIDTRAP`
    #[value_string = "EF_SKIDTRAP"]
    Skidtrap,
    /// `EF_BRANDISHSPEAR`
    #[value_string = "EF_BRANDISHSPEAR"]
    Brandishspear,
    /// `EF_CONE`
    #[value_string = "EF_CONE"]
    Cone,
    /// `EF_SPHERE`
    #[value_string = "EF_SPHERE"]
    Sphere,
    /// `EF_BOWLINGBASH`
    #[value_string = "EF_BOWLINGBASH"]
    Bowlingbash,
    /// `EF_ICEWALL`
    #[value_string = "EF_ICEWALL"]
    Icewall,
    /// `EF_GLORIA`
    #[value_string = "EF_GLORIA"]
    Gloria,
    /// `EF_MAGNIFICAT`
    #[value_string = "EF_MAGNIFICAT"]
    Magnificat,
    /// `EF_RESURRECTION`
    #[value_string = "EF_RESURRECTION"]
    Resurrection,
    /// `EF_RECOVERY`
    #[value_string = "EF_RECOVERY"]
    Recovery,
    /// `EF_EARTHSPIKE`
    #[value_string = "EF_EARTHSPIKE"]
    Earthspike,
    /// `EF_SPEARBMR`
    #[value_string = "EF_SPEARBMR"]
    Spearbmr,
    /// `EF_PIERCE`
    #[value_string = "EF_PIERCE"]
    Pierce,
    /// `EF_TURNUNDEAD`
    #[value_string = "EF_TURNUNDEAD"]
    Turnundead,
    /// `EF_SANCTUARY`
    #[value_string = "EF_SANCTUARY"]
    Sanctuary,
    /// `EF_IMPOSITIO`
    #[value_string = "EF_IMPOSITIO"]
    Impositio,
    /// `EF_LEXAETERNA`
    #[value_string = "EF_LEXAETERNA"]
    Lexaeterna,
    /// `EF_ASPERSIO`
    #[value_string = "EF_ASPERSIO"]
    Aspersio,
    /// `EF_LEXDIVINA`
    #[value_string = "EF_LEXDIVINA"]
    Lexdivina,
    /// `EF_SUFFRAGIUM`
    #[value_string = "EF_SUFFRAGIUM"]
    Suffragium,
    /// `EF_STORMGUST`
    #[value_string = "EF_STORMGUST"]
    Stormgust,
    /// `EF_LORD`
    #[value_string = "EF_LORD"]
    Lord,
    /// `EF_BENEDICTIO`
    #[value_string = "EF_BENEDICTIO"]
    Benedictio,
    /// `EF_METEORSTORM`
    #[value_string = "EF_METEORSTORM"]
    Meteorstorm,
    /// `EF_YUFITEL`
    #[value_string = "EF_YUFITEL"]
    Yufitel,
    /// `EF_YUFITELHIT`
    #[value_string = "EF_YUFITELHIT"]
    Yufitelhit,
    /// `EF_QUAGMIRE`
    #[value_string = "EF_QUAGMIRE"]
    Quagmire,
    /// `EF_FIREPILLAR`
    #[value_string = "EF_FIREPILLAR"]
    Firepillar,
    /// `EF_FIREPILLARBOMB`
    #[value_string = "EF_FIREPILLARBOMB"]
    Firepillarbomb,
    /// `EF_HASTEUP`
    #[value_string = "EF_HASTEUP"]
    Hasteup,
    /// `EF_FLASHER`
    #[value_string = "EF_FLASHER"]
    Flasher,
    /// `EF_REMOVETRAP`
    #[value_string = "EF_REMOVETRAP"]
    Removetrap,
    /// `EF_REPAIRWEAPON`
    #[value_string = "EF_REPAIRWEAPON"]
    Repairweapon,
    /// `EF_CRASHEARTH`
    #[value_string = "EF_CRASHEARTH"]
    Crashearth,
    /// `EF_PERFECTION`
    #[value_string = "EF_PERFECTION"]
    Perfection,
    /// `EF_MAXPOWER`
    #[value_string = "EF_MAXPOWER"]
    Maxpower,
    /// `EF_BLASTMINE`
    #[value_string = "EF_BLASTMINE"]
    Blastmine,
    /// `EF_BLASTMINEBOMB`
    #[value_string = "EF_BLASTMINEBOMB"]
    Blastminebomb,
    /// `EF_CLAYMORE`
    #[value_string = "EF_CLAYMORE"]
    Claymore,
    /// `EF_FREEZING`
    #[value_string = "EF_FREEZING"]
    Freezing,
    /// `EF_BUBBLE`
    #[value_string = "EF_BUBBLE"]
    Bubble,
    /// `EF_GASPUSH`
    #[value_string = "EF_GASPUSH"]
    Gaspush,
    /// `EF_SPRINGTRAP`
    #[value_string = "EF_SPRINGTRAP"]
    Springtrap,
    /// `EF_KYRIE`
    #[value_string = "EF_KYRIE"]
    Kyrie,
    /// `EF_MAGNUS`
    #[value_string = "EF_MAGNUS"]
    Magnus,
    /// `EF_BOTTOM`
    #[value_string = "EF_BOTTOM"]
    Bottom,
    /// `EF_BLITZBEAT`
    #[value_string = "EF_BLITZBEAT"]
    Blitzbeat,
    /// `EF_WATERBALL`
    #[value_string = "EF_WATERBALL"]
    Waterball,
    /// `EF_WATERBALL2`
    #[value_string = "EF_WATERBALL2"]
    Waterball2,
    /// `EF_FIREIVY`
    #[value_string = "EF_FIREIVY"]
    Fireivy,
    /// `EF_DETECTING`
    #[value_string = "EF_DETECTING"]
    Detecting,
    /// `EF_CLOAKING`
    #[value_string = "EF_CLOAKING"]
    Cloaking,
    /// `EF_SONICBLOW`
    #[value_string = "EF_SONICBLOW"]
    Sonicblow,
    /// `EF_SONICBLOWHIT`
    #[value_string = "EF_SONICBLOWHIT"]
    Sonicblowhit,
    /// `EF_GRIMTOOTH`
    #[value_string = "EF_GRIMTOOTH"]
    Grimtooth,
    /// `EF_VENOMDUST`
    #[value_string = "EF_VENOMDUST"]
    Venomdust,
    /// `EF_ENCHANTPOISON`
    #[value_string = "EF_ENCHANTPOISON"]
    Enchantpoison,
    /// `EF_POISONREACT`
    #[value_string = "EF_POISONREACT"]
    Poisonreact,
    /// `EF_POISONREACT2`
    #[value_string = "EF_POISONREACT2"]
    Poisonreact2,
    /// `EF_OVERTHRUST`
    #[value_string = "EF_OVERTHRUST"]
    Overthrust,
    /// `EF_SPLASHER`
    #[value_string = "EF_SPLASHER"]
    Splasher,
    /// `EF_TWOHANDQUICKEN`
    #[value_string = "EF_TWOHANDQUICKEN"]
    Twohandquicken,
    /// `EF_AUTOCOUNTER`
    #[value_string = "EF_AUTOCOUNTER"]
    Autocounter,
    /// `EF_GRIMTOOTHATK`
    #[value_string = "EF_GRIMTOOTHATK"]
    Grimtoothatk,
    /// `EF_FREEZE`
    #[value_string = "EF_FREEZE"]
    Freeze,
    /// `EF_FREEZED`
    #[value_string = "EF_FREEZED"]
    Freezed,
    /// `EF_ICECRASH`
    #[value_string = "EF_ICECRASH"]
    Icecrash,
    /// `EF_SLOWPOISON`
    #[value_string = "EF_SLOWPOISON"]
    Slowpoison,
    /// `EF_BOTTOM2`
    #[value_string = "EF_BOTTOM2"]
    Bottom2,
    /// `EF_FIREPILLARON`
    #[value_string = "EF_FIREPILLARON"]
    Firepillaron,
    /// `EF_SANDMAN`
    #[value_string = "EF_SANDMAN"]
    Sandman,
    /// `EF_REVIVE`
    #[value_string = "EF_REVIVE"]
    Revive,
    /// `EF_PNEUMA`
    #[value_string = "EF_PNEUMA"]
    Pneuma,
    /// `EF_HEAVENSDRIVE`
    #[value_string = "EF_HEAVENSDRIVE"]
    Heavensdrive,
    /// `EF_SONICBLOW2`
    #[value_string = "EF_SONICBLOW2"]
    Sonicblow2,
    /// `EF_BRANDISH2`
    #[value_string = "EF_BRANDISH2"]
    Brandish2,
    /// `EF_SHOCKWAVE`
    #[value_string = "EF_SHOCKWAVE"]
    Shockwave,
    /// `EF_SHOCKWAVEHIT`
    #[value_string = "EF_SHOCKWAVEHIT"]
    Shockwavehit,
    /// `EF_EARTHHIT`
    #[value_string = "EF_EARTHHIT"]
    Earthhit,
    /// `EF_PIERCESELF`
    #[value_string = "EF_PIERCESELF"]
    Pierceself,
    /// `EF_BOWLINGSELF`
    #[value_string = "EF_BOWLINGSELF"]
    Bowlingself,
    /// `EF_SPEARSTABSELF`
    #[value_string = "EF_SPEARSTABSELF"]
    Spearstabself,
    /// `EF_SPEARBMRSELF`
    #[value_string = "EF_SPEARBMRSELF"]
    Spearbmrself,
    /// `EF_HOLYHIT`
    #[value_string = "EF_HOLYHIT"]
    Holyhit,
    /// `EF_CONCENTRATION`
    #[value_string = "EF_CONCENTRATION"]
    Concentration,
    /// `EF_REFINEOK`
    #[value_string = "EF_REFINEOK"]
    Refineok,
    /// `EF_REFINEFAIL`
    #[value_string = "EF_REFINEFAIL"]
    Refinefail,
    /// `EF_JOBCHANGE`
    #[value_string = "EF_JOBCHANGE"]
    Jobchange,
    /// `EF_LVUP`
    #[value_string = "EF_LVUP"]
    Lvup,
    /// `EF_JOBLVUP`
    #[value_string = "EF_JOBLVUP"]
    Joblvup,
    /// `EF_TOPRANK`
    #[value_string = "EF_TOPRANK"]
    Toprank,
    /// `EF_PARTY`
    #[value_string = "EF_PARTY"]
    Party,
    /// `EF_RAIN`
    #[value_string = "EF_RAIN"]
    Rain,
    /// `EF_SNOW`
    #[value_string = "EF_SNOW"]
    Snow,
    /// `EF_SAKURA`
    #[value_string = "EF_SAKURA"]
    Sakura,
    /// `EF_STATUS_STATE`
    #[value_string = "EF_STATUS_STATE"]
    StatusState,
    /// `EF_BANJJAKII`
    #[value_string = "EF_BANJJAKII"]
    Banjjakii,
    /// `EF_MAKEBLUR`
    #[value_string = "EF_MAKEBLUR"]
    Makeblur,
    /// `EF_TAMINGSUCCESS`
    #[value_string = "EF_TAMINGSUCCESS"]
    Tamingsuccess,
    /// `EF_TAMINGFAILED`
    #[value_string = "EF_TAMINGFAILED"]
    Tamingfailed,
    /// `EF_ENERGYCOAT`
    #[value_string = "EF_ENERGYCOAT"]
    Energycoat,
    /// `EF_CARTREVOLUTION`
    #[value_string = "EF_CARTREVOLUTION"]
    Cartrevolution,
    /// `EF_VENOMDUST2`
    #[value_string = "EF_VENOMDUST2"]
    Venomdust2,
    /// `EF_CHANGEDARK`
    #[value_string = "EF_CHANGEDARK"]
    Changedark,
    /// `EF_CHANGEFIRE`
    #[value_string = "EF_CHANGEFIRE"]
    Changefire,
    /// `EF_CHANGECOLD`
    #[value_string = "EF_CHANGECOLD"]
    Changecold,
    /// `EF_CHANGEWIND`
    #[value_string = "EF_CHANGEWIND"]
    Changewind,
    /// `EF_CHANGEFLAME`
    #[value_string = "EF_CHANGEFLAME"]
    Changeflame,
    /// `EF_CHANGEEARTH`
    #[value_string = "EF_CHANGEEARTH"]
    Changeearth,
    /// `EF_CHAINGEHOLY`
    #[value_string = "EF_CHAINGEHOLY"]
    Chaingeholy,
    /// `EF_CHANGEPOISON`
    #[value_string = "EF_CHANGEPOISON"]
    Changepoison,
    /// `EF_HITDARK`
    #[value_string = "EF_HITDARK"]
    Hitdark,
    /// `EF_MENTALBREAK`
    #[value_string = "EF_MENTALBREAK"]
    Mentalbreak,
    /// `EF_MAGICALATTHIT`
    #[value_string = "EF_MAGICALATTHIT"]
    Magicalatthit,
    /// `EF_SUI_EXPLOSION`
    #[value_string = "EF_SUI_EXPLOSION"]
    SuiExplosion,
    /// `EF_DARKATTACK`
    #[value_string = "EF_DARKATTACK"]
    Darkattack,
    /// `EF_SUICIDE`
    #[value_string = "EF_SUICIDE"]
    Suicide,
    /// `EF_COMBOATTACK1`
    #[value_string = "EF_COMBOATTACK1"]
    Comboattack1,
    /// `EF_COMBOATTACK2`
    #[value_string = "EF_COMBOATTACK2"]
    Comboattack2,
    /// `EF_COMBOATTACK3`
    #[value_string = "EF_COMBOATTACK3"]
    Comboattack3,
    /// `EF_COMBOATTACK4`
    #[value_string = "EF_COMBOATTACK4"]
    Comboattack4,
    /// `EF_COMBOATTACK5`
    #[value_string = "EF_COMBOATTACK5"]
    Comboattack5,
    /// `EF_GUIDEDATTACK`
    #[value_string = "EF_GUIDEDATTACK"]
    Guidedattack,
    /// `EF_POISONATTACK`
    #[value_string = "EF_POISONATTACK"]
    Poisonattack,
    /// `EF_SILENCEATTACK`
    #[value_string = "EF_SILENCEATTACK"]
    Silenceattack,
    /// `EF_STUNATTACK`
    #[value_string = "EF_STUNATTACK"]
    Stunattack,
    /// `EF_PETRIFYATTACK`
    #[value_string = "EF_PETRIFYATTACK"]
    Petrifyattack,
    /// `EF_CURSEATTACK`
    #[value_string = "EF_CURSEATTACK"]
    Curseattack,
    /// `EF_SLEEPATTACK`
    #[value_string = "EF_SLEEPATTACK"]
    Sleepattack,
    /// `EF_TELEKHIT`
    #[value_string = "EF_TELEKHIT"]
    Telekhit,
    /// `EF_PONG`
    #[value_string = "EF_PONG"]
    Pong,
    /// `EF_LEVEL99`
    #[value_string = "EF_LEVEL99"]
    Level99,
    /// `EF_LEVEL99_2`
    #[value_string = "EF_LEVEL99_2"]
    Level992,
    /// `EF_LEVEL99_3`
    #[value_string = "EF_LEVEL99_3"]
    Level993,
    /// `EF_GUMGANG`
    #[value_string = "EF_GUMGANG"]
    Gumgang,
    /// `EF_POTION1`
    #[value_string = "EF_POTION1"]
    Potion1,
    /// `EF_POTION2`
    #[value_string = "EF_POTION2"]
    Potion2,
    /// `EF_POTION3`
    #[value_string = "EF_POTION3"]
    Potion3,
    /// `EF_POTION4`
    #[value_string = "EF_POTION4"]
    Potion4,
    /// `EF_POTION5`
    #[value_string = "EF_POTION5"]
    Potion5,
    /// `EF_POTION6`
    #[value_string = "EF_POTION6"]
    Potion6,
    /// `EF_POTION7`
    #[value_string = "EF_POTION7"]
    Potion7,
    /// `EF_POTION8`
    #[value_string = "EF_POTION8"]
    Potion8,
    /// `EF_DARKBREATH`
    #[value_string = "EF_DARKBREATH"]
    Darkbreath,
    /// `EF_DEFFENDER`
    #[value_string = "EF_DEFFENDER"]
    Deffender,
    /// `EF_KEEPING`
    #[value_string = "EF_KEEPING"]
    Keeping,
    /// `EF_SUMMONSLAVE`
    #[value_string = "EF_SUMMONSLAVE"]
    Summonslave,
    /// `EF_BLOODDRAIN`
    #[value_string = "EF_BLOODDRAIN"]
    Blooddrain,
    /// `EF_ENERGYDRAIN`
    #[value_string = "EF_ENERGYDRAIN"]
    Energydrain,
    /// `EF_POTION_CON`
    #[value_string = "EF_POTION_CON"]
    PotionCon,
    /// `EF_POTION_`
    #[value_string = "EF_POTION_"]
    Potion,
    /// `EF_POTION_BERSERK`
    #[value_string = "EF_POTION_BERSERK"]
    PotionBerserk,
    /// `EF_POTIONPILLAR`
    #[value_string = "EF_POTIONPILLAR"]
    Potionpillar,
    /// `EF_DEFENDER`
    #[value_string = "EF_DEFENDER"]
    Defender,
    /// `EF_GANBANTEIN`
    #[value_string = "EF_GANBANTEIN"]
    Ganbantein,
    /// `EF_WIND`
    #[value_string = "EF_WIND"]
    Wind,
    /// `EF_VOLCANO`
    #[value_string = "EF_VOLCANO"]
    Volcano,
    /// `EF_GRANDCROSS`
    #[value_string = "EF_GRANDCROSS"]
    Grandcross,
    /// `EF_INTIMIDATE`
    #[value_string = "EF_INTIMIDATE"]
    Intimidate,
    /// `EF_CHOOKGI`
    #[value_string = "EF_CHOOKGI"]
    Chookgi,
    /// `EF_CLOUD`
    #[value_string = "EF_CLOUD"]
    Cloud,
    /// `EF_CLOUD2`
    #[value_string = "EF_CLOUD2"]
    Cloud2,
    /// `EF_MAPPILLAR`
    #[value_string = "EF_MAPPILLAR"]
    Mappillar,
    /// `EF_LINELINK`
    #[value_string = "EF_LINELINK"]
    Linelink,
    /// `EF_CLOUD3`
    #[value_string = "EF_CLOUD3"]
    Cloud3,
    /// `EF_SPELLBREAKER`
    #[value_string = "EF_SPELLBREAKER"]
    Spellbreaker,
    /// `EF_DISPELL`
    #[value_string = "EF_DISPELL"]
    Dispell,
    /// `EF_DELUGE`
    #[value_string = "EF_DELUGE"]
    Deluge,
    /// `EF_VIOLENTGALE`
    #[value_string = "EF_VIOLENTGALE"]
    Violentgale,
    /// `EF_LANDPROTECTOR`
    #[value_string = "EF_LANDPROTECTOR"]
    Landprotector,
    /// `EF_BOTTOM_VO`
    #[value_string = "EF_BOTTOM_VO"]
    BottomVo,
    /// `EF_BOTTOM_DE`
    #[value_string = "EF_BOTTOM_DE"]
    BottomDe,
    /// `EF_BOTTOM_VI`
    #[value_string = "EF_BOTTOM_VI"]
    BottomVi,
    /// `EF_BOTTOM_LA`
    #[value_string = "EF_BOTTOM_LA"]
    BottomLa,
    /// `EF_FASTMOVE`
    #[value_string = "EF_FASTMOVE"]
    Fastmove,
    /// `EF_MAGICROD`
    #[value_string = "EF_MAGICROD"]
    Magicrod,
    /// `EF_HOLYCROSS`
    #[value_string = "EF_HOLYCROSS"]
    Holycross,
    /// `EF_SHIELDCHARGE`
    #[value_string = "EF_SHIELDCHARGE"]
    Shieldcharge,
    /// `EF_MAPPILLAR2`
    #[value_string = "EF_MAPPILLAR2"]
    Mappillar2,
    /// `EF_PROVIDENCE`
    #[value_string = "EF_PROVIDENCE"]
    Providence,
    /// `EF_SHIELDBOOMERANG`
    #[value_string = "EF_SHIELDBOOMERANG"]
    Shieldboomerang,
    /// `EF_SPEARQUICKEN`
    #[value_string = "EF_SPEARQUICKEN"]
    Spearquicken,
    /// `EF_DEVOTION`
    #[value_string = "EF_DEVOTION"]
    Devotion,
    /// `EF_REFLECTSHIELD`
    #[value_string = "EF_REFLECTSHIELD"]
    Reflectshield,
    /// `EF_ABSORBSPIRITS`
    #[value_string = "EF_ABSORBSPIRITS"]
    Absorbspirits,
    /// `EF_STEELBODY`
    #[value_string = "EF_STEELBODY"]
    Steelbody,
    /// `EF_FLAMELAUNCHER`
    #[value_string = "EF_FLAMELAUNCHER"]
    Flamelauncher,
    /// `EF_FROSTWEAPON`
    #[value_string = "EF_FROSTWEAPON"]
    Frostweapon,
    /// `EF_LIGHTNINGLOADER`
    #[value_string = "EF_LIGHTNINGLOADER"]
    Lightningloader,
    /// `EF_SEISMICWEAPON`
    #[value_string = "EF_SEISMICWEAPON"]
    Seismicweapon,
    /// `EF_MAPPILLAR3`
    #[value_string = "EF_MAPPILLAR3"]
    Mappillar3,
    /// `EF_MAPPILLAR4`
    #[value_string = "EF_MAPPILLAR4"]
    Mappillar4,
    /// `EF_GUMGANG2`
    #[value_string = "EF_GUMGANG2"]
    Gumgang2,
    /// `EF_TEIHIT1`
    #[value_string = "EF_TEIHIT1"]
    Teihit1,
    /// `EF_GUMGANG3`
    #[value_string = "EF_GUMGANG3"]
    Gumgang3,
    /// `EF_TEIHIT2`
    #[value_string = "EF_TEIHIT2"]
    Teihit2,
    /// `EF_TANJI`
    #[value_string = "EF_TANJI"]
    Tanji,
    /// `EF_TEIHIT1X`
    #[value_string = "EF_TEIHIT1X"]
    Teihit1x,
    /// `EF_CHIMTO`
    #[value_string = "EF_CHIMTO"]
    Chimto,
    /// `EF_STEALCOIN`
    #[value_string = "EF_STEALCOIN"]
    Stealcoin,
    /// `EF_STRIPWEAPON`
    #[value_string = "EF_STRIPWEAPON"]
    Stripweapon,
    /// `EF_STRIPSHIELD`
    #[value_string = "EF_STRIPSHIELD"]
    Stripshield,
    /// `EF_STRIPARMOR`
    #[value_string = "EF_STRIPARMOR"]
    Striparmor,
    /// `EF_STRIPHELM`
    #[value_string = "EF_STRIPHELM"]
    Striphelm,
    /// `EF_CHAINCOMBO`
    #[value_string = "EF_CHAINCOMBO"]
    Chaincombo,
    /// `EF_RG_COIN`
    #[value_string = "EF_RG_COIN"]
    RgCoin,
    /// `EF_BACKSTAP`
    #[value_string = "EF_BACKSTAP"]
    Backstap,
    /// `EF_TEIHIT3`
    #[value_string = "EF_TEIHIT3"]
    Teihit3,
    /// `EF_BOTTOM_DISSONANCE`
    #[value_string = "EF_BOTTOM_DISSONANCE"]
    BottomDissonance,
    /// `EF_BOTTOM_LULLABY`
    #[value_string = "EF_BOTTOM_LULLABY"]
    BottomLullaby,
    /// `EF_BOTTOM_RICHMANKIM`
    #[value_string = "EF_BOTTOM_RICHMANKIM"]
    BottomRichmankim,
    /// `EF_BOTTOM_ETERNALCHAOS`
    #[value_string = "EF_BOTTOM_ETERNALCHAOS"]
    BottomEternalchaos,
    /// `EF_BOTTOM_DRUMBATTLEFIELD`
    #[value_string = "EF_BOTTOM_DRUMBATTLEFIELD"]
    BottomDrumbattlefield,
    /// `EF_BOTTOM_RINGNIBELUNGEN`
    #[value_string = "EF_BOTTOM_RINGNIBELUNGEN"]
    BottomRingnibelungen,
    /// `EF_BOTTOM_ROKISWEIL`
    #[value_string = "EF_BOTTOM_ROKISWEIL"]
    BottomRokisweil,
    /// `EF_BOTTOM_INTOABYSS`
    #[value_string = "EF_BOTTOM_INTOABYSS"]
    BottomIntoabyss,
    /// `EF_BOTTOM_SIEGFRIED`
    #[value_string = "EF_BOTTOM_SIEGFRIED"]
    BottomSiegfried,
    /// `EF_BOTTOM_WHISTLE`
    #[value_string = "EF_BOTTOM_WHISTLE"]
    BottomWhistle,
    /// `EF_BOTTOM_ASSASSINCROSS`
    #[value_string = "EF_BOTTOM_ASSASSINCROSS"]
    BottomAssassincross,
    /// `EF_BOTTOM_POEMBRAGI`
    #[value_string = "EF_BOTTOM_POEMBRAGI"]
    BottomPoembragi,
    /// `EF_BOTTOM_APPLEIDUN`
    #[value_string = "EF_BOTTOM_APPLEIDUN"]
    BottomAppleidun,
    /// `EF_BOTTOM_UGLYDANCE`
    #[value_string = "EF_BOTTOM_UGLYDANCE"]
    BottomUglydance,
    /// `EF_BOTTOM_HUMMING`
    #[value_string = "EF_BOTTOM_HUMMING"]
    BottomHumming,
    /// `EF_BOTTOM_DONTFORGETME`
    #[value_string = "EF_BOTTOM_DONTFORGETME"]
    BottomDontforgetme,
    /// `EF_BOTTOM_FORTUNEKISS`
    #[value_string = "EF_BOTTOM_FORTUNEKISS"]
    BottomFortunekiss,
    /// `EF_BOTTOM_SERVICEFORYOU`
    #[value_string = "EF_BOTTOM_SERVICEFORYOU"]
    BottomServiceforyou,
    /// `EF_TALK_FROSTJOKE`
    #[value_string = "EF_TALK_FROSTJOKE"]
    TalkFrostjoke,
    /// `EF_TALK_SCREAM`
    #[value_string = "EF_TALK_SCREAM"]
    TalkScream,
    /// `EF_POKJUK`
    #[value_string = "EF_POKJUK"]
    Pokjuk,
    /// `EF_THROWITEM`
    #[value_string = "EF_THROWITEM"]
    Throwitem,
    /// `EF_THROWITEM2`
    #[value_string = "EF_THROWITEM2"]
    Throwitem2,
    /// `EF_CHEMICALPROTECTION`
    #[value_string = "EF_CHEMICALPROTECTION"]
    Chemicalprotection,
    /// `EF_POKJUK_SOUND`
    #[value_string = "EF_POKJUK_SOUND"]
    PokjukSound,
    /// `EF_DEMONSTRATION`
    #[value_string = "EF_DEMONSTRATION"]
    Demonstration,
    /// `EF_CHEMICAL2`
    #[value_string = "EF_CHEMICAL2"]
    Chemical2,
    /// `EF_TELEPORTATION2`
    #[value_string = "EF_TELEPORTATION2"]
    Teleportation2,
    /// `EF_PHARMACY_OK`
    #[value_string = "EF_PHARMACY_OK"]
    PharmacyOk,
    /// `EF_PHARMACY_FAIL`
    #[value_string = "EF_PHARMACY_FAIL"]
    PharmacyFail,
    /// `EF_FORESTLIGHT`
    #[value_string = "EF_FORESTLIGHT"]
    Forestlight,
    /// `EF_THROWITEM3`
    #[value_string = "EF_THROWITEM3"]
    Throwitem3,
    /// `EF_FIRSTAID`
    #[value_string = "EF_FIRSTAID"]
    Firstaid,
    /// `EF_SPRINKLESAND`
    #[value_string = "EF_SPRINKLESAND"]
    Sprinklesand,
    /// `EF_LOUD`
    #[value_string = "EF_LOUD"]
    Loud,
    /// `EF_HEAL`
    #[value_string = "EF_HEAL"]
    Heal,
    /// `EF_HEAL2`
    #[value_string = "EF_HEAL2"]
    Heal2,
    /// `EF_EXIT2`
    #[value_string = "EF_EXIT2"]
    Exit2,
    /// `EF_GLASSWALL2`
    #[value_string = "EF_GLASSWALL2"]
    Glasswall2,
    /// `EF_READYPORTAL2`
    #[value_string = "EF_READYPORTAL2"]
    Readyportal2,
    /// `EF_PORTAL2`
    #[value_string = "EF_PORTAL2"]
    Portal2,
    /// `EF_BOTTOM_MAG`
    #[value_string = "EF_BOTTOM_MAG"]
    BottomMag,
    /// `EF_BOTTOM_SANC`
    #[value_string = "EF_BOTTOM_SANC"]
    BottomSanc,
    /// `EF_HEAL3`
    #[value_string = "EF_HEAL3"]
    Heal3,
    /// `EF_WARPZONE2`
    #[value_string = "EF_WARPZONE2"]
    Warpzone2,
    /// `EF_FORESTLIGHT2`
    #[value_string = "EF_FORESTLIGHT2"]
    Forestlight2,
    /// `EF_FORESTLIGHT3`
    #[value_string = "EF_FORESTLIGHT3"]
    Forestlight3,
    /// `EF_FORESTLIGHT4`
    #[value_string = "EF_FORESTLIGHT4"]
    Forestlight4,
    /// `EF_HEAL4`
    #[value_string = "EF_HEAL4"]
    Heal4,
    /// `EF_FOOT`
    #[value_string = "EF_FOOT"]
    Foot,
    /// `EF_FOOT2`
    #[value_string = "EF_FOOT2"]
    Foot2,
    /// `EF_BEGINASURA`
    #[value_string = "EF_BEGINASURA"]
    Beginasura,
    /// `EF_TRIPLEATTACK`
    #[value_string = "EF_TRIPLEATTACK"]
    Tripleattack,
    /// `EF_HITLINE`
    #[value_string = "EF_HITLINE"]
    Hitline,
    /// `EF_HPTIME`
    #[value_string = "EF_HPTIME"]
    Hptime,
    /// `EF_SPTIME`
    #[value_string = "EF_SPTIME"]
    Sptime,
    /// `EF_MAPLE`
    #[value_string = "EF_MAPLE"]
    Maple,
    /// `EF_BLIND`
    #[value_string = "EF_BLIND"]
    Blind,
    /// `EF_POISON`
    #[value_string = "EF_POISON"]
    Poison,
    /// `EF_GUARD`
    #[value_string = "EF_GUARD"]
    Guard,
    /// `EF_JOBLVUP50`
    #[value_string = "EF_JOBLVUP50"]
    Joblvup50,
    /// `EF_ANGEL2`
    #[value_string = "EF_ANGEL2"]
    Angel2,
    /// `EF_MAGNUM2`
    #[value_string = "EF_MAGNUM2"]
    Magnum2,
    /// `EF_CALLZONE`
    #[value_string = "EF_CALLZONE"]
    Callzone,
    /// `EF_PORTAL3`
    #[value_string = "EF_PORTAL3"]
    Portal3,
    /// `EF_COUPLECASTING`
    #[value_string = "EF_COUPLECASTING"]
    Couplecasting,
    /// `EF_HEARTCASTING`
    #[value_string = "EF_HEARTCASTING"]
    Heartcasting,
    /// `EF_ENTRY2`
    #[value_string = "EF_ENTRY2"]
    Entry2,
    /// `EF_SAINTWING`
    #[value_string = "EF_SAINTWING"]
    Saintwing,
    /// `EF_SPHEREWIND`
    #[value_string = "EF_SPHEREWIND"]
    Spherewind,
    /// `EF_COLORPAPER`
    #[value_string = "EF_COLORPAPER"]
    Colorpaper,
    /// `EF_LIGHTSPHERE`
    #[value_string = "EF_LIGHTSPHERE"]
    Lightsphere,
    /// `EF_WATERFALL`
    #[value_string = "EF_WATERFALL"]
    Waterfall,
    /// `EF_WATERFALL_90`
    #[value_string = "EF_WATERFALL_90"]
    Waterfall90,
    /// `EF_WATERFALL_SMALL`
    #[value_string = "EF_WATERFALL_SMALL"]
    WaterfallSmall,
    /// `EF_WATERFALL_SMALL_90`
    #[value_string = "EF_WATERFALL_SMALL_90"]
    WaterfallSmall90,
    /// `EF_WATERFALL_T2`
    #[value_string = "EF_WATERFALL_T2"]
    WaterfallT2,
    /// `EF_WATERFALL_T2_90`
    #[value_string = "EF_WATERFALL_T2_90"]
    WaterfallT290,
    /// `EF_WATERFALL_SMALL_T2`
    #[value_string = "EF_WATERFALL_SMALL_T2"]
    WaterfallSmallT2,
    /// `EF_WATERFALL_SMALL_T2_90`
    #[value_string = "EF_WATERFALL_SMALL_T2_90"]
    WaterfallSmallT290,
    /// `EF_MINI_TETRIS`
    #[value_string = "EF_MINI_TETRIS"]
    MiniTetris,
    /// `EF_GHOST`
    #[value_string = "EF_GHOST"]
    Ghost,
    /// `EF_BAT`
    #[value_string = "EF_BAT"]
    Bat,
    /// `EF_BAT2`
    #[value_string = "EF_BAT2"]
    Bat2,
    /// `EF_SOULBREAKER`
    #[value_string = "EF_SOULBREAKER"]
    Soulbreaker,
    /// `EF_LEVEL99_4`
    #[value_string = "EF_LEVEL99_4"]
    Level994,
    /// `EF_VALLENTINE`
    #[value_string = "EF_VALLENTINE"]
    Vallentine,
    /// `EF_VALLENTINE2`
    #[value_string = "EF_VALLENTINE2"]
    Vallentine2,
    /// `EF_PRESSURE`
    #[value_string = "EF_PRESSURE"]
    Pressure,
    /// `EF_BASH3D`
    #[value_string = "EF_BASH3D"]
    Bash3d,
    /// `EF_AURABLADE`
    #[value_string = "EF_AURABLADE"]
    Aurablade,
    /// `EF_REDBODY`
    #[value_string = "EF_REDBODY"]
    Redbody,
    /// `EF_LKCONCENTRATION`
    #[value_string = "EF_LKCONCENTRATION"]
    Lkconcentration,
    /// `EF_BOTTOM_GOSPEL`
    #[value_string = "EF_BOTTOM_GOSPEL"]
    BottomGospel,
    /// `EF_ANGEL`
    #[value_string = "EF_ANGEL"]
    Angel,
    /// `EF_DEVIL`
    #[value_string = "EF_DEVIL"]
    Devil,
    /// `EF_DRAGONSMOKE`
    #[value_string = "EF_DRAGONSMOKE"]
    Dragonsmoke,
    /// `EF_BOTTOM_BASILICA`
    #[value_string = "EF_BOTTOM_BASILICA"]
    BottomBasilica,
    /// `EF_ASSUMPTIO`
    #[value_string = "EF_ASSUMPTIO"]
    Assumptio,
    /// `EF_HITLINE2`
    #[value_string = "EF_HITLINE2"]
    Hitline2,
    /// `EF_BASH3D2`
    #[value_string = "EF_BASH3D2"]
    Bash3d2,
    /// `EF_ENERGYDRAIN2`
    #[value_string = "EF_ENERGYDRAIN2"]
    Energydrain2,
    /// `EF_TRANSBLUEBODY`
    #[value_string = "EF_TRANSBLUEBODY"]
    Transbluebody,
    /// `EF_MAGICCRASHER`
    #[value_string = "EF_MAGICCRASHER"]
    Magiccrasher,
    /// `EF_LIGHTSPHERE2`
    #[value_string = "EF_LIGHTSPHERE2"]
    Lightsphere2,
    /// `EF_LIGHTBLADE`
    #[value_string = "EF_LIGHTBLADE"]
    Lightblade,
    /// `EF_ENERGYDRAIN3`
    #[value_string = "EF_ENERGYDRAIN3"]
    Energydrain3,
    /// `EF_LINELINK2`
    #[value_string = "EF_LINELINK2"]
    Linelink2,
    /// `EF_LINKLIGHT`
    #[value_string = "EF_LINKLIGHT"]
    Linklight,
    /// `EF_TRUESIGHT`
    #[value_string = "EF_TRUESIGHT"]
    Truesight,
    /// `EF_FALCONASSAULT`
    #[value_string = "EF_FALCONASSAULT"]
    Falconassault,
    /// `EF_TRIPLEATTACK2`
    #[value_string = "EF_TRIPLEATTACK2"]
    Tripleattack2,
    /// `EF_PORTAL4`
    #[value_string = "EF_PORTAL4"]
    Portal4,
    /// `EF_MELTDOWN`
    #[value_string = "EF_MELTDOWN"]
    Meltdown,
    /// `EF_CARTBOOST`
    #[value_string = "EF_CARTBOOST"]
    Cartboost,
    /// `EF_REJECTSWORD`
    #[value_string = "EF_REJECTSWORD"]
    Rejectsword,
    /// `EF_TRIPLEATTACK3`
    #[value_string = "EF_TRIPLEATTACK3"]
    Tripleattack3,
    /// `EF_SPHEREWIND2`
    #[value_string = "EF_SPHEREWIND2"]
    Spherewind2,
    /// `EF_LINELINK3`
    #[value_string = "EF_LINELINK3"]
    Linelink3,
    /// `EF_PINKBODY`
    #[value_string = "EF_PINKBODY"]
    Pinkbody,
    /// `EF_LEVEL99_5`
    #[value_string = "EF_LEVEL99_5"]
    Level995,
    /// `EF_LEVEL99_6`
    #[value_string = "EF_LEVEL99_6"]
    Level996,
    /// `EF_BASH3D3`
    #[value_string = "EF_BASH3D3"]
    Bash3d3,
    /// `EF_BASH3D4`
    #[value_string = "EF_BASH3D4"]
    Bash3d4,
    /// `EF_NAPALMVALCAN`
    #[value_string = "EF_NAPALMVALCAN"]
    Napalmvalcan,
    /// `EF_PORTAL5`
    #[value_string = "EF_PORTAL5"]
    Portal5,
    /// `EF_MAGICCRASHER2`
    #[value_string = "EF_MAGICCRASHER2"]
    Magiccrasher2,
    /// `EF_BOTTOM_SPIDER`
    #[value_string = "EF_BOTTOM_SPIDER"]
    BottomSpider,
    /// `EF_BOTTOM_FOGWALL`
    #[value_string = "EF_BOTTOM_FOGWALL"]
    BottomFogwall,
    /// `EF_SOULBURN`
    #[value_string = "EF_SOULBURN"]
    Soulburn,
    /// `EF_SOULCHANGE`
    #[value_string = "EF_SOULCHANGE"]
    Soulchange,
    /// `EF_BABY`
    #[value_string = "EF_BABY"]
    Baby,
    /// `EF_SOULBREAKER2`
    #[value_string = "EF_SOULBREAKER2"]
    Soulbreaker2,
    /// `EF_RAINBOW`
    #[value_string = "EF_RAINBOW"]
    Rainbow,
    /// `EF_PEONG`
    #[value_string = "EF_PEONG"]
    Peong,
    /// `EF_TANJI2`
    #[value_string = "EF_TANJI2"]
    Tanji2,
    /// `EF_PRESSEDBODY`
    #[value_string = "EF_PRESSEDBODY"]
    Pressedbody,
    /// `EF_SPINEDBODY`
    #[value_string = "EF_SPINEDBODY"]
    Spinedbody,
    /// `EF_KICKEDBODY`
    #[value_string = "EF_KICKEDBODY"]
    Kickedbody,
    /// `EF_AIRTEXTURE`
    #[value_string = "EF_AIRTEXTURE"]
    Airtexture,
    /// `EF_HITBODY`
    #[value_string = "EF_HITBODY"]
    Hitbody,
    /// `EF_DOUBLEGUMGANG`
    #[value_string = "EF_DOUBLEGUMGANG"]
    Doublegumgang,
    /// `EF_REFLECTBODY`
    #[value_string = "EF_REFLECTBODY"]
    Reflectbody,
    /// `EF_BABYBODY`
    #[value_string = "EF_BABYBODY"]
    Babybody,
    /// `EF_BABYBODY2`
    #[value_string = "EF_BABYBODY2"]
    Babybody2,
    /// `EF_GIANTBODY`
    #[value_string = "EF_GIANTBODY"]
    Giantbody,
    /// `EF_GIANTBODY2`
    #[value_string = "EF_GIANTBODY2"]
    Giantbody2,
    /// `EF_ASURABODY`
    #[value_string = "EF_ASURABODY"]
    Asurabody,
    /// `EF_4WAYBODY`
    #[value_string = "EF_4WAYBODY"]
    Ef4waybody,
    /// `EF_QUAKEBODY`
    #[value_string = "EF_QUAKEBODY"]
    Quakebody,
    /// `EF_ASURABODY_MONSTER`
    #[value_string = "EF_ASURABODY_MONSTER"]
    AsurabodyMonster,
    /// `EF_HITLINE3`
    #[value_string = "EF_HITLINE3"]
    Hitline3,
    /// `EF_HITLINE4`
    #[value_string = "EF_HITLINE4"]
    Hitline4,
    /// `EF_HITLINE5`
    #[value_string = "EF_HITLINE5"]
    Hitline5,
    /// `EF_HITLINE6`
    #[value_string = "EF_HITLINE6"]
    Hitline6,
    /// `EF_ELECTRIC`
    #[value_string = "EF_ELECTRIC"]
    Electric,
    /// `EF_ELECTRIC2`
    #[value_string = "EF_ELECTRIC2"]
    Electric2,
    /// `EF_HITLINE7`
    #[value_string = "EF_HITLINE7"]
    Hitline7,
    /// `EF_STORMKICK`
    #[value_string = "EF_STORMKICK"]
    Stormkick,
    /// `EF_HALFSPHERE`
    #[value_string = "EF_HALFSPHERE"]
    Halfsphere,
    /// `EF_ATTACKENERGY`
    #[value_string = "EF_ATTACKENERGY"]
    Attackenergy,
    /// `EF_ATTACKENERGY2`
    #[value_string = "EF_ATTACKENERGY2"]
    Attackenergy2,
    /// `EF_CHEMICAL3`
    #[value_string = "EF_CHEMICAL3"]
    Chemical3,
    /// `EF_ASSUMPTIO2`
    #[value_string = "EF_ASSUMPTIO2"]
    Assumptio2,
    /// `EF_BLUECASTING`
    #[value_string = "EF_BLUECASTING"]
    Bluecasting,
    /// `EF_RUN`
    #[value_string = "EF_RUN"]
    Run,
    /// `EF_STOPRUN`
    #[value_string = "EF_STOPRUN"]
    Stoprun,
    /// `EF_STOPEFFECT`
    #[value_string = "EF_STOPEFFECT"]
    Stopeffect,
    /// `EF_JUMPBODY`
    #[value_string = "EF_JUMPBODY"]
    Jumpbody,
    /// `EF_LANDBODY`
    #[value_string = "EF_LANDBODY"]
    Landbody,
    /// `EF_FOOT3`
    #[value_string = "EF_FOOT3"]
    Foot3,
    /// `EF_FOOT4`
    #[value_string = "EF_FOOT4"]
    Foot4,
    /// `EF_TAE_READY`
    #[value_string = "EF_TAE_READY"]
    TaeReady,
    /// `EF_GRANDCROSS2`
    #[value_string = "EF_GRANDCROSS2"]
    Grandcross2,
    /// `EF_SOULSTRIKE2`
    #[value_string = "EF_SOULSTRIKE2"]
    Soulstrike2,
    /// `EF_YUFITEL2`
    #[value_string = "EF_YUFITEL2"]
    Yufitel2,
    /// `EF_NPC_STOP`
    #[value_string = "EF_NPC_STOP"]
    NpcStop,
    /// `EF_DARKCASTING`
    #[value_string = "EF_DARKCASTING"]
    Darkcasting,
    /// `EF_GUMGANGNPC`
    #[value_string = "EF_GUMGANGNPC"]
    Gumgangnpc,
    /// `EF_AGIUP`
    #[value_string = "EF_AGIUP"]
    Agiup,
    /// `EF_JUMPKICK`
    #[value_string = "EF_JUMPKICK"]
    Jumpkick,
    /// `EF_QUAKEBODY2`
    #[value_string = "EF_QUAKEBODY2"]
    Quakebody2,
    /// `EF_STORMKICK1`
    #[value_string = "EF_STORMKICK1"]
    Stormkick1,
    /// `EF_STORMKICK2`
    #[value_string = "EF_STORMKICK2"]
    Stormkick2,
    /// `EF_STORMKICK3`
    #[value_string = "EF_STORMKICK3"]
    Stormkick3,
    /// `EF_STORMKICK4`
    #[value_string = "EF_STORMKICK4"]
    Stormkick4,
    /// `EF_STORMKICK5`
    #[value_string = "EF_STORMKICK5"]
    Stormkick5,
    /// `EF_STORMKICK6`
    #[value_string = "EF_STORMKICK6"]
    Stormkick6,
    /// `EF_STORMKICK7`
    #[value_string = "EF_STORMKICK7"]
    Stormkick7,
    /// `EF_SPINEDBODY2`
    #[value_string = "EF_SPINEDBODY2"]
    Spinedbody2,
    /// `EF_BEGINASURA1`
    #[value_string = "EF_BEGINASURA1"]
    Beginasura1,
    /// `EF_BEGINASURA2`
    #[value_string = "EF_BEGINASURA2"]
    Beginasura2,
    /// `EF_BEGINASURA3`
    #[value_string = "EF_BEGINASURA3"]
    Beginasura3,
    /// `EF_BEGINASURA4`
    #[value_string = "EF_BEGINASURA4"]
    Beginasura4,
    /// `EF_BEGINASURA5`
    #[value_string = "EF_BEGINASURA5"]
    Beginasura5,
    /// `EF_BEGINASURA6`
    #[value_string = "EF_BEGINASURA6"]
    Beginasura6,
    /// `EF_BEGINASURA7`
    #[value_string = "EF_BEGINASURA7"]
    Beginasura7,
    /// `EF_AURABLADE2`
    #[value_string = "EF_AURABLADE2"]
    Aurablade2,
    /// `EF_DEVIL1`
    #[value_string = "EF_DEVIL1"]
    Devil1,
    /// `EF_DEVIL2`
    #[value_string = "EF_DEVIL2"]
    Devil2,
    /// `EF_DEVIL3`
    #[value_string = "EF_DEVIL3"]
    Devil3,
    /// `EF_DEVIL4`
    #[value_string = "EF_DEVIL4"]
    Devil4,
    /// `EF_DEVIL5`
    #[value_string = "EF_DEVIL5"]
    Devil5,
    /// `EF_DEVIL6`
    #[value_string = "EF_DEVIL6"]
    Devil6,
    /// `EF_DEVIL7`
    #[value_string = "EF_DEVIL7"]
    Devil7,
    /// `EF_DEVIL8`
    #[value_string = "EF_DEVIL8"]
    Devil8,
    /// `EF_DEVIL9`
    #[value_string = "EF_DEVIL9"]
    Devil9,
    /// `EF_DEVIL10`
    #[value_string = "EF_DEVIL10"]
    Devil10,
    /// `EF_DOUBLEGUMGANG2`
    #[value_string = "EF_DOUBLEGUMGANG2"]
    Doublegumgang2,
    /// `EF_DOUBLEGUMGANG3`
    #[value_string = "EF_DOUBLEGUMGANG3"]
    Doublegumgang3,
    /// `EF_BLACKDEVIL`
    #[value_string = "EF_BLACKDEVIL"]
    Blackdevil,
    /// `EF_FLOWERCAST`
    #[value_string = "EF_FLOWERCAST"]
    Flowercast,
    /// `EF_FLOWERCAST2`
    #[value_string = "EF_FLOWERCAST2"]
    Flowercast2,
    /// `EF_FLOWERCAST3`
    #[value_string = "EF_FLOWERCAST3"]
    Flowercast3,
    /// `EF_MOCHI`
    #[value_string = "EF_MOCHI"]
    Mochi,
    /// `EF_LAMADAN`
    #[value_string = "EF_LAMADAN"]
    Lamadan,
    /// `EF_EDP`
    #[value_string = "EF_EDP"]
    Edp,
    /// `EF_SHIELDBOOMERANG2`
    #[value_string = "EF_SHIELDBOOMERANG2"]
    Shieldboomerang2,
    /// `EF_RG_COIN2`
    #[value_string = "EF_RG_COIN2"]
    RgCoin2,
    /// `EF_GUARD2`
    #[value_string = "EF_GUARD2"]
    Guard2,
    /// `EF_SLIM`
    #[value_string = "EF_SLIM"]
    Slim,
    /// `EF_SLIM2`
    #[value_string = "EF_SLIM2"]
    Slim2,
    /// `EF_SLIM3`
    #[value_string = "EF_SLIM3"]
    Slim3,
    /// `EF_CHEMICALBODY`
    #[value_string = "EF_CHEMICALBODY"]
    Chemicalbody,
    /// `EF_CASTSPIN`
    #[value_string = "EF_CASTSPIN"]
    Castspin,
    /// `EF_PIERCEBODY`
    #[value_string = "EF_PIERCEBODY"]
    Piercebody,
    /// `EF_SOULLINK`
    #[value_string = "EF_SOULLINK"]
    Soullink,
    /// `EF_CHOOKGI2`
    #[value_string = "EF_CHOOKGI2"]
    Chookgi2,
    /// `EF_MEMORIZE`
    #[value_string = "EF_MEMORIZE"]
    Memorize,
    /// `EF_SOULLIGHT`
    #[value_string = "EF_SOULLIGHT"]
    Soullight,
    /// `EF_MAPAE`
    #[value_string = "EF_MAPAE"]
    Mapae,
    /// `EF_ITEMPOKJUK`
    #[value_string = "EF_ITEMPOKJUK"]
    Itempokjuk,
    /// `EF_05VAL`
    #[value_string = "EF_05VAL"]
    Ef05val,
    /// `EF_BEGINASURA11`
    #[value_string = "EF_BEGINASURA11"]
    Beginasura11,
    /// `EF_NIGHT`
    #[value_string = "EF_NIGHT"]
    Night,
    /// `EF_CHEMICAL2DASH`
    #[value_string = "EF_CHEMICAL2DASH"]
    Chemical2dash,
    /// `EF_GROUNDSAMPLE`
    #[value_string = "EF_GROUNDSAMPLE"]
    Groundsample,
    /// `EF_GI_EXPLOSION`
    #[value_string = "EF_GI_EXPLOSION"]
    GiExplosion,
    /// `EF_CLOUD4`
    #[value_string = "EF_CLOUD4"]
    Cloud4,
    /// `EF_CLOUD5`
    #[value_string = "EF_CLOUD5"]
    Cloud5,
    /// `EF_BOTTOM_HERMODE`
    #[value_string = "EF_BOTTOM_HERMODE"]
    BottomHermode,
    /// `EF_CARTTER`
    #[value_string = "EF_CARTTER"]
    Cartter,
    /// `EF_ITEMFAST`
    #[value_string = "EF_ITEMFAST"]
    Itemfast,
    /// `EF_SHIELDBOOMERANG3`
    #[value_string = "EF_SHIELDBOOMERANG3"]
    Shieldboomerang3,
    /// `EF_DOUBLECASTBODY`
    #[value_string = "EF_DOUBLECASTBODY"]
    Doublecastbody,
    /// `EF_GRAVITATION`
    #[value_string = "EF_GRAVITATION"]
    Gravitation,
    /// `EF_TAROTCARD1`
    #[value_string = "EF_TAROTCARD1"]
    Tarotcard1,
    /// `EF_TAROTCARD2`
    #[value_string = "EF_TAROTCARD2"]
    Tarotcard2,
    /// `EF_TAROTCARD3`
    #[value_string = "EF_TAROTCARD3"]
    Tarotcard3,
    /// `EF_TAROTCARD4`
    #[value_string = "EF_TAROTCARD4"]
    Tarotcard4,
    /// `EF_TAROTCARD5`
    #[value_string = "EF_TAROTCARD5"]
    Tarotcard5,
    /// `EF_TAROTCARD6`
    #[value_string = "EF_TAROTCARD6"]
    Tarotcard6,
    /// `EF_TAROTCARD7`
    #[value_string = "EF_TAROTCARD7"]
    Tarotcard7,
    /// `EF_TAROTCARD8`
    #[value_string = "EF_TAROTCARD8"]
    Tarotcard8,
    /// `EF_TAROTCARD9`
    #[value_string = "EF_TAROTCARD9"]
    Tarotcard9,
    /// `EF_TAROTCARD10`
    #[value_string = "EF_TAROTCARD10"]
    Tarotcard10,
    /// `EF_TAROTCARD11`
    #[value_string = "EF_TAROTCARD11"]
    Tarotcard11,
    /// `EF_TAROTCARD12`
    #[value_string = "EF_TAROTCARD12"]
    Tarotcard12,
    /// `EF_TAROTCARD13`
    #[value_string = "EF_TAROTCARD13"]
    Tarotcard13,
    /// `EF_TAROTCARD14`
    #[value_string = "EF_TAROTCARD14"]
    Tarotcard14,
    /// `EF_ACIDDEMON`
    #[value_string = "EF_ACIDDEMON"]
    Aciddemon,
    /// `EF_GREENBODY`
    #[value_string = "EF_GREENBODY"]
    Greenbody,
    /// `EF_THROWITEM4`
    #[value_string = "EF_THROWITEM4"]
    Throwitem4,
    /// `EF_BABYBODY_BACK`
    #[value_string = "EF_BABYBODY_BACK"]
    BabybodyBack,
    /// `EF_THROWITEM5`
    #[value_string = "EF_THROWITEM5"]
    Throwitem5,
    /// `EF_BLUEBODY`
    #[value_string = "EF_BLUEBODY"]
    Bluebody,
    /// `EF_HATED`
    #[value_string = "EF_HATED"]
    Hated,
    /// `EF_REDLIGHTBODY`
    #[value_string = "EF_REDLIGHTBODY"]
    Redlightbody,
    /// `EF_RO2YEAR`
    #[value_string = "EF_RO2YEAR"]
    Ro2year,
    /// `EF_SMA_READY`
    #[value_string = "EF_SMA_READY"]
    SmaReady,
    /// `EF_STIN`
    #[value_string = "EF_STIN"]
    Stin,
    /// `EF_RED_HIT`
    #[value_string = "EF_RED_HIT"]
    RedHit,
    /// `EF_BLUE_HIT`
    #[value_string = "EF_BLUE_HIT"]
    BlueHit,
    /// `EF_QUAKEBODY3`
    #[value_string = "EF_QUAKEBODY3"]
    Quakebody3,
    /// `EF_SMA`
    #[value_string = "EF_SMA"]
    Sma,
    /// `EF_SMA2`
    #[value_string = "EF_SMA2"]
    Sma2,
    /// `EF_STIN2`
    #[value_string = "EF_STIN2"]
    Stin2,
    /// `EF_HITTEXTURE`
    #[value_string = "EF_HITTEXTURE"]
    Hittexture,
    /// `EF_STIN3`
    #[value_string = "EF_STIN3"]
    Stin3,
    /// `EF_SMA3`
    #[value_string = "EF_SMA3"]
    Sma3,
    /// `EF_BLUEFALL`
    #[value_string = "EF_BLUEFALL"]
    Bluefall,
    /// `EF_BLUEFALL_90`
    #[value_string = "EF_BLUEFALL_90"]
    Bluefall90,
    /// `EF_FASTBLUEFALL`
    #[value_string = "EF_FASTBLUEFALL"]
    Fastbluefall,
    /// `EF_FASTBLUEFALL_90`
    #[value_string = "EF_FASTBLUEFALL_90"]
    Fastbluefall90,
    /// `EF_BIG_PORTAL`
    #[value_string = "EF_BIG_PORTAL"]
    BigPortal,
    /// `EF_BIG_PORTAL2`
    #[value_string = "EF_BIG_PORTAL2"]
    BigPortal2,
    /// `EF_SCREEN_QUAKE`
    #[value_string = "EF_SCREEN_QUAKE"]
    ScreenQuake,
    /// `EF_HOMUNCASTING`
    #[value_string = "EF_HOMUNCASTING"]
    Homuncasting,
    /// `EF_HFLIMOON1`
    #[value_string = "EF_HFLIMOON1"]
    Hflimoon1,
    /// `EF_HFLIMOON2`
    #[value_string = "EF_HFLIMOON2"]
    Hflimoon2,
    /// `EF_HFLIMOON3`
    #[value_string = "EF_HFLIMOON3"]
    Hflimoon3,
    /// `EF_HO_UP`
    #[value_string = "EF_HO_UP"]
    HoUp,
    /// `EF_HAMIDEFENCE`
    #[value_string = "EF_HAMIDEFENCE"]
    Hamidefence,
    /// `EF_HAMICASTLE`
    #[value_string = "EF_HAMICASTLE"]
    Hamicastle,
    /// `EF_HAMIBLOOD`
    #[value_string = "EF_HAMIBLOOD"]
    Hamiblood,
    /// `EF_HATED2`
    #[value_string = "EF_HATED2"]
    Hated2,
    /// `EF_TWILIGHT1`
    #[value_string = "EF_TWILIGHT1"]
    Twilight1,
    /// `EF_TWILIGHT2`
    #[value_string = "EF_TWILIGHT2"]
    Twilight2,
    /// `EF_TWILIGHT3`
    #[value_string = "EF_TWILIGHT3"]
    Twilight3,
    /// `EF_ITEM_THUNDER`
    #[value_string = "EF_ITEM_THUNDER"]
    ItemThunder,
    /// `EF_ITEM_CLOUD`
    #[value_string = "EF_ITEM_CLOUD"]
    ItemCloud,
    /// `EF_ITEM_CURSE`
    #[value_string = "EF_ITEM_CURSE"]
    ItemCurse,
    /// `EF_ITEM_ZZZ`
    #[value_string = "EF_ITEM_ZZZ"]
    ItemZzz,
    /// `EF_ITEM_RAIN`
    #[value_string = "EF_ITEM_RAIN"]
    ItemRain,
    /// `EF_ITEM_LIGHT`
    #[value_string = "EF_ITEM_LIGHT"]
    ItemLight,
    /// `EF_ANGEL3`
    #[value_string = "EF_ANGEL3"]
    Angel3,
    /// `EF_M01`
    #[value_string = "EF_M01"]
    M01,
    /// `EF_M02`
    #[value_string = "EF_M02"]
    M02,
    /// `EF_M03`
    #[value_string = "EF_M03"]
    M03,
    /// `EF_M04`
    #[value_string = "EF_M04"]
    M04,
    /// `EF_M05`
    #[value_string = "EF_M05"]
    M05,
    /// `EF_M06`
    #[value_string = "EF_M06"]
    M06,
    /// `EF_M07`
    #[value_string = "EF_M07"]
    M07,
    /// `EF_KAIZEL`
    #[value_string = "EF_KAIZEL"]
    Kaizel,
    /// `EF_KAAHI`
    #[value_string = "EF_KAAHI"]
    Kaahi,
    /// `EF_CLOUD6`
    #[value_string = "EF_CLOUD6"]
    Cloud6,
    /// `EF_FOOD01`
    #[value_string = "EF_FOOD01"]
    Food01,
    /// `EF_FOOD02`
    #[value_string = "EF_FOOD02"]
    Food02,
    /// `EF_FOOD03`
    #[value_string = "EF_FOOD03"]
    Food03,
    /// `EF_FOOD04`
    #[value_string = "EF_FOOD04"]
    Food04,
    /// `EF_FOOD05`
    #[value_string = "EF_FOOD05"]
    Food05,
    /// `EF_FOOD06`
    #[value_string = "EF_FOOD06"]
    Food06,
    /// `EF_SHRINK`
    #[value_string = "EF_SHRINK"]
    Shrink,
    /// `EF_THROWITEM6`
    #[value_string = "EF_THROWITEM6"]
    Throwitem6,
    /// `EF_SIGHT2`
    #[value_string = "EF_SIGHT2"]
    Sight2,
    /// `EF_QUAKEBODY4`
    #[value_string = "EF_QUAKEBODY4"]
    Quakebody4,
    /// `EF_FIREHIT2`
    #[value_string = "EF_FIREHIT2"]
    Firehit2,
    /// `EF_NPC_STOP2`
    #[value_string = "EF_NPC_STOP2"]
    NpcStop2,
    /// `EF_NPC_STOP2_DEL`
    #[value_string = "EF_NPC_STOP2_DEL"]
    NpcStop2Del,
    /// `EF_FVOICE`
    #[value_string = "EF_FVOICE"]
    Fvoice,
    /// `EF_WINK`
    #[value_string = "EF_WINK"]
    Wink,
    /// `EF_COOKING_OK`
    #[value_string = "EF_COOKING_OK"]
    CookingOk,
    /// `EF_COOKING_FAIL`
    #[value_string = "EF_COOKING_FAIL"]
    CookingFail,
    /// `EF_TEMP_OK`
    #[value_string = "EF_TEMP_OK"]
    TempOk,
    /// `EF_TEMP_FAIL`
    #[value_string = "EF_TEMP_FAIL"]
    TempFail,
    /// `EF_HAPGYEOK`
    #[value_string = "EF_HAPGYEOK"]
    Hapgyeok,
    /// `EF_THROWITEM7`
    #[value_string = "EF_THROWITEM7"]
    Throwitem7,
    /// `EF_THROWITEM8`
    #[value_string = "EF_THROWITEM8"]
    Throwitem8,
    /// `EF_THROWITEM9`
    #[value_string = "EF_THROWITEM9"]
    Throwitem9,
    /// `EF_THROWITEM10`
    #[value_string = "EF_THROWITEM10"]
    Throwitem10,
    /// `EF_BUNSINJYUTSU`
    #[value_string = "EF_BUNSINJYUTSU"]
    Bunsinjyutsu,
    /// `EF_KOUENKA`
    #[value_string = "EF_KOUENKA"]
    Kouenka,
    /// `EF_HYOUSENSOU`
    #[value_string = "EF_HYOUSENSOU"]
    Hyousensou,
    /// `EF_BOTTOM_SUITON`
    #[value_string = "EF_BOTTOM_SUITON"]
    BottomSuiton,
    /// `EF_STIN4`
    #[value_string = "EF_STIN4"]
    Stin4,
    /// `EF_THUNDERSTORM2`
    #[value_string = "EF_THUNDERSTORM2"]
    Thunderstorm2,
    /// `EF_CHEMICAL4`
    #[value_string = "EF_CHEMICAL4"]
    Chemical4,
    /// `EF_STIN5`
    #[value_string = "EF_STIN5"]
    Stin5,
    /// `EF_MADNESS_BLUE`
    #[value_string = "EF_MADNESS_BLUE"]
    MadnessBlue,
    /// `EF_MADNESS_RED`
    #[value_string = "EF_MADNESS_RED"]
    MadnessRed,
    /// `EF_RG_COIN3`
    #[value_string = "EF_RG_COIN3"]
    RgCoin3,
    /// `EF_BASH3D5`
    #[value_string = "EF_BASH3D5"]
    Bash3d5,
    /// `EF_CHOOKGI3`
    #[value_string = "EF_CHOOKGI3"]
    Chookgi3,
    /// `EF_KIRIKAGE`
    #[value_string = "EF_KIRIKAGE"]
    Kirikage,
    /// `EF_TATAMI`
    #[value_string = "EF_TATAMI"]
    Tatami,
    /// `EF_KASUMIKIRI`
    #[value_string = "EF_KASUMIKIRI"]
    Kasumikiri,
    /// `EF_ISSEN`
    #[value_string = "EF_ISSEN"]
    Issen,
    /// `EF_KAEN`
    #[value_string = "EF_KAEN"]
    Kaen,
    /// `EF_BAKU`
    #[value_string = "EF_BAKU"]
    Baku,
    /// `EF_HYOUSYOURAKU`
    #[value_string = "EF_HYOUSYOURAKU"]
    Hyousyouraku,
    /// `EF_DESPERADO`
    #[value_string = "EF_DESPERADO"]
    Desperado,
    /// `EF_LIGHTNING_S`
    #[value_string = "EF_LIGHTNING_S"]
    LightningS,
    /// `EF_BLIND_S`
    #[value_string = "EF_BLIND_S"]
    BlindS,
    /// `EF_POISON_S`
    #[value_string = "EF_POISON_S"]
    PoisonS,
    /// `EF_FREEZING_S`
    #[value_string = "EF_FREEZING_S"]
    FreezingS,
    /// `EF_FLARE_S`
    #[value_string = "EF_FLARE_S"]
    FlareS,
    /// `EF_RAPIDSHOWER`
    #[value_string = "EF_RAPIDSHOWER"]
    Rapidshower,
    /// `EF_MAGICALBULLET`
    #[value_string = "EF_MAGICALBULLET"]
    Magicalbullet,
    /// `EF_SPREADATTACK`
    #[value_string = "EF_SPREADATTACK"]
    Spreadattack,
    /// `EF_TRACKCASTING`
    #[value_string = "EF_TRACKCASTING"]
    Trackcasting,
    /// `EF_TRACKING`
    #[value_string = "EF_TRACKING"]
    Tracking,
    /// `EF_TRIPLEACTION`
    #[value_string = "EF_TRIPLEACTION"]
    Tripleaction,
    /// `EF_BULLSEYE`
    #[value_string = "EF_BULLSEYE"]
    Bullseye,
    /// `EF_MAP_MAGICZONE`
    #[value_string = "EF_MAP_MAGICZONE"]
    MapMagiczone,
    /// `EF_MAP_MAGICZONE2`
    #[value_string = "EF_MAP_MAGICZONE2"]
    MapMagiczone2,
    /// `EF_DAMAGE1`
    #[value_string = "EF_DAMAGE1"]
    Damage1,
    /// `EF_DAMAGE1_2`
    #[value_string = "EF_DAMAGE1_2"]
    Damage12,
    /// `EF_DAMAGE1_3`
    #[value_string = "EF_DAMAGE1_3"]
    Damage13,
    /// `EF_UNDEADBODY`
    #[value_string = "EF_UNDEADBODY"]
    Undeadbody,
    /// `EF_UNDEADBODY_DEL`
    #[value_string = "EF_UNDEADBODY_DEL"]
    UndeadbodyDel,
    /// `EF_GREEN_NUMBER`
    #[value_string = "EF_GREEN_NUMBER"]
    GreenNumber,
    /// `EF_BLUE_NUMBER`
    #[value_string = "EF_BLUE_NUMBER"]
    BlueNumber,
    /// `EF_RED_NUMBER`
    #[value_string = "EF_RED_NUMBER"]
    RedNumber,
    /// `EF_PURPLE_NUMBER`
    #[value_string = "EF_PURPLE_NUMBER"]
    PurpleNumber,
    /// `EF_BLACK_NUMBER`
    #[value_string = "EF_BLACK_NUMBER"]
    BlackNumber,
    /// `EF_WHITE_NUMBER`
    #[value_string = "EF_WHITE_NUMBER"]
    WhiteNumber,
    /// `EF_YELLOW_NUMBER`
    #[value_string = "EF_YELLOW_NUMBER"]
    YellowNumber,
    /// `EF_PINK_NUMBER`
    #[value_string = "EF_PINK_NUMBER"]
    PinkNumber,
    /// `EF_BUBBLE_DROP`
    #[value_string = "EF_BUBBLE_DROP"]
    BubbleDrop,
    /// `EF_NPC_EARTHQUAKE`
    #[value_string = "EF_NPC_EARTHQUAKE"]
    NpcEarthquake,
    /// `EF_DA_SPACE`
    #[value_string = "EF_DA_SPACE"]
    DaSpace,
    /// `EF_DRAGONFEAR`
    #[value_string = "EF_DRAGONFEAR"]
    Dragonfear,
    /// `EF_BLEEDING`
    #[value_string = "EF_BLEEDING"]
    Bleeding,
    /// `EF_WIDECONFUSE`
    #[value_string = "EF_WIDECONFUSE"]
    Wideconfuse,
    /// `EF_BOTTOM_RUNNER`
    #[value_string = "EF_BOTTOM_RUNNER"]
    BottomRunner,
    /// `EF_BOTTOM_TRANSFER`
    #[value_string = "EF_BOTTOM_TRANSFER"]
    BottomTransfer,
    /// `EF_CRYSTAL_BLUE`
    #[value_string = "EF_CRYSTAL_BLUE"]
    CrystalBlue,
    /// `EF_BOTTOM_EVILLAND`
    #[value_string = "EF_BOTTOM_EVILLAND"]
    BottomEvilland,
    /// `EF_GUARD3`
    #[value_string = "EF_GUARD3"]
    Guard3,
    /// `EF_NPC_SLOWCAST`
    #[value_string = "EF_NPC_SLOWCAST"]
    NpcSlowcast,
    /// `EF_CRITICALWOUND`
    #[value_string = "EF_CRITICALWOUND"]
    Criticalwound,
    /// `EF_GREEN99_3`
    #[value_string = "EF_GREEN99_3"]
    Green993,
    /// `EF_GREEN99_5`
    #[value_string = "EF_GREEN99_5"]
    Green995,
    /// `EF_GREEN99_6`
    #[value_string = "EF_GREEN99_6"]
    Green996,
    /// `EF_MAPSPHERE`
    #[value_string = "EF_MAPSPHERE"]
    Mapsphere,
    /// `EF_POK_LOVE`
    #[value_string = "EF_POK_LOVE"]
    PokLove,
    /// `EF_POK_WHITE`
    #[value_string = "EF_POK_WHITE"]
    PokWhite,
    /// `EF_POK_VALEN`
    #[value_string = "EF_POK_VALEN"]
    PokValen,
    /// `EF_POK_BIRTH`
    #[value_string = "EF_POK_BIRTH"]
    PokBirth,
    /// `EF_POK_CHRISTMAS`
    #[value_string = "EF_POK_CHRISTMAS"]
    PokChristmas,
    /// `EF_MAP_MAGICZONE3`
    #[value_string = "EF_MAP_MAGICZONE3"]
    MapMagiczone3,
    /// `EF_MAP_MAGICZONE4`
    #[value_string = "EF_MAP_MAGICZONE4"]
    MapMagiczone4,
    /// `EF_DUST`
    #[value_string = "EF_DUST"]
    Dust,
    /// `EF_TORCH_RED`
    #[value_string = "EF_TORCH_RED"]
    TorchRed,
    /// `EF_TORCH_GREEN`
    #[value_string = "EF_TORCH_GREEN"]
    TorchGreen,
    /// `EF_MAP_GHOST`
    #[value_string = "EF_MAP_GHOST"]
    MapGhost,
    /// `EF_GLOW1`
    #[value_string = "EF_GLOW1"]
    Glow1,
    /// `EF_GLOW2`
    #[value_string = "EF_GLOW2"]
    Glow2,
    /// `EF_GLOW4`
    #[value_string = "EF_GLOW4"]
    Glow4,
    /// `EF_TORCH_PURPLE`
    #[value_string = "EF_TORCH_PURPLE"]
    TorchPurple,
    /// `EF_CLOUD7`
    #[value_string = "EF_CLOUD7"]
    Cloud7,
    /// `EF_CLOUD8`
    #[value_string = "EF_CLOUD8"]
    Cloud8,
    /// `EF_FLOWERLEAF`
    #[value_string = "EF_FLOWERLEAF"]
    Flowerleaf,
    /// `EF_MAPSPHERE2`
    #[value_string = "EF_MAPSPHERE2"]
    Mapsphere2,
    /// `EF_GLOW11`
    #[value_string = "EF_GLOW11"]
    Glow11,
    /// `EF_GLOW12`
    #[value_string = "EF_GLOW12"]
    Glow12,
    /// `EF_FOOT5`
    #[value_string = "EF_FOOT5"]
    Foot5,
    /// `EF_FOOT6`
    #[value_string = "EF_FOOT6"]
    Foot6,
    /// `EF_AIRTEXTURE2`
    #[value_string = "EF_AIRTEXTURE2"]
    Airtexture2,
    /// `EF_AIRTEXTURE3`
    #[value_string = "EF_AIRTEXTURE3"]
    Airtexture3,
    /// `EF_AIRTEXTURE4`
    #[value_string = "EF_AIRTEXTURE4"]
    Airtexture4,
    /// `EF_CODE_EFFECT_BEGIN`
    #[value_string = "EF_CODE_EFFECT_BEGIN"]
    #[value = 1000]
    CodeEffectBegin,
    /// `EF_MAKEBLUR3`
    #[value_string = "EF_MAKEBLUR3"]
    Makeblur3,
    /// `EF_MAKEBLUR4`
    #[value_string = "EF_MAKEBLUR4"]
    Makeblur4,
    /// `EF_BLOOD_FLY`
    #[value_string = "EF_BLOOD_FLY"]
    BloodFly,
    /// `EF_HIT7`
    #[value_string = "EF_HIT7"]
    Hit7,
    /// `EF_TEIHIT1REVERSE`
    #[value_string = "EF_TEIHIT1REVERSE"]
    Teihit1reverse,
    /// `EF_TEIHIT2REVERSE`
    #[value_string = "EF_TEIHIT2REVERSE"]
    Teihit2reverse,
    /// `EF_MAKEBLUR5`
    #[value_string = "EF_MAKEBLUR5"]
    Makeblur5,
    /// `EF_ENCHANTPOISON_FLOW`
    #[value_string = "EF_ENCHANTPOISON_FLOW"]
    EnchantpoisonFlow,
    /// `EF_ARROW_YELLOW`
    #[value_string = "EF_ARROW_YELLOW"]
    ArrowYellow,
    /// `EF_ARROW_RED`
    #[value_string = "EF_ARROW_RED"]
    ArrowRed,
    /// `EF_SIGHT3`
    #[value_string = "EF_SIGHT3"]
    Sight3,
    /// `EF_TEIHIT3REVERSE`
    #[value_string = "EF_TEIHIT3REVERSE"]
    Teihit3reverse,
    /// `EF_BEGINSPELLWHITE`
    #[value_string = "EF_BEGINSPELLWHITE"]
    Beginspellwhite,
    /// `EF_BEGINSPELL8`
    #[value_string = "EF_BEGINSPELL8"]
    Beginspell8,
    /// `EF_ENDURE_ZHAN`
    #[value_string = "EF_ENDURE_ZHAN"]
    EndureZhan,
    /// `EF_ENDURE_SOU`
    #[value_string = "EF_ENDURE_SOU"]
    EndureSou,
    /// `EF_ENDURE_SHAN`
    #[value_string = "EF_ENDURE_SHAN"]
    EndureShan,
    /// `EF_ENDURE_JING`
    #[value_string = "EF_ENDURE_JING"]
    EndureJing,
    /// `EF_WIND_BUFF`
    #[value_string = "EF_WIND_BUFF"]
    WindBuff,
    /// `EF_BEGINSPELLRED`
    #[value_string = "EF_BEGINSPELLRED"]
    Beginspellred,
    /// `EF_GREEN_POP`
    #[value_string = "EF_GREEN_POP"]
    GreenPop,
    /// `EF_BEGINSPELL_N`
    #[value_string = "EF_BEGINSPELL_N"]
    BeginspellN,
    /// `EF_ARROW_DOWN`
    #[value_string = "EF_ARROW_DOWN"]
    ArrowDown,
    /// `EF_CODE_EFFECT_END`
    #[value_string = "EF_CODE_EFFECT_END"]
    CodeEffectEnd,
    /// `EF_PROCESS2_BEGIN`
    #[value_string = "EF_PROCESS2_BEGIN"]
    #[value = 1030]
    Process2Begin,
    /// `EF_TEXTURE_FALLING`
    #[value_string = "EF_TEXTURE_FALLING"]
    TextureFalling,
    /// `EF_SPHEREWIND3`
    #[value_string = "EF_SPHEREWIND3"]
    Spherewind3,
    /// `EF_PROCESS2_END`
    #[value_string = "EF_PROCESS2_END"]
    Process2End,
    /// `EF_FILE_EFFECT_BEGIN`
    #[value_string = "EF_FILE_EFFECT_BEGIN"]
    #[value = 1100]
    FileEffectBegin,
    /// `EF_SHAKE`
    #[value_string = "EF_SHAKE"]
    Shake,
    /// `EF_LEVELUP`
    #[value_string = "EF_LEVELUP"]
    Levelup,
    /// `EF_JOBLEVELUP`
    #[value_string = "EF_JOBLEVELUP"]
    Joblevelup,
    /// `EF_NPCDEAD`
    #[value_string = "EF_NPCDEAD"]
    Npcdead,
    /// `EF_CLAW_ATK`
    #[value_string = "EF_CLAW_ATK"]
    ClawAtk,
    /// `EF_SWORD_LIGHT`
    #[value_string = "EF_SWORD_LIGHT"]
    SwordLight,
    /// `EF_RING4`
    #[value_string = "EF_RING4"]
    Ring4,
    /// `EF_HIT8`
    #[value_string = "EF_HIT8"]
    Hit8,
    /// `EF_CAST_MAGIC_RED`
    #[value_string = "EF_CAST_MAGIC_RED"]
    CastMagicRed,
    /// `EF_CAST_MAGIC_RED2`
    #[value_string = "EF_CAST_MAGIC_RED2"]
    CastMagicRed2,
    /// `EF_CAST_MAGIC_BLUE`
    #[value_string = "EF_CAST_MAGIC_BLUE"]
    CastMagicBlue,
    /// `EF_CAST_MAGIC_BLUE2`
    #[value_string = "EF_CAST_MAGIC_BLUE2"]
    CastMagicBlue2,
    /// `EF_CAST_MAGIC_WHITE`
    #[value_string = "EF_CAST_MAGIC_WHITE"]
    CastMagicWhite,
    /// `EF_CAST_MAGIC_WHITE2`
    #[value_string = "EF_CAST_MAGIC_WHITE2"]
    CastMagicWhite2,
    /// `EF_CAST_MAGIC_YELLOW`
    #[value_string = "EF_CAST_MAGIC_YELLOW"]
    CastMagicYellow,
    /// `EF_CAST_MAGIC_YELLOW2`
    #[value_string = "EF_CAST_MAGIC_YELLOW2"]
    CastMagicYellow2,
    /// `EF_FLAMMULE`
    #[value_string = "EF_FLAMMULE"]
    Flammule,
    /// `EF_BLINGLINE`
    #[value_string = "EF_BLINGLINE"]
    Blingline,
    /// `EF_BLINGLINE2`
    #[value_string = "EF_BLINGLINE2"]
    Blingline2,
    /// `EF_GROUNDIMAGE`
    #[value_string = "EF_GROUNDIMAGE"]
    Groundimage,
    /// `EF_GROUNDIMAGE3`
    #[value_string = "EF_GROUNDIMAGE3"]
    Groundimage3,
    /// `EF_GROUNDIMAGE5`
    #[value_string = "EF_GROUNDIMAGE5"]
    Groundimage5,
    /// `EF_GROUNDIMAGE7`
    #[value_string = "EF_GROUNDIMAGE7"]
    Groundimage7,
    /// `EF_GROUNDIMAGE9`
    #[value_string = "EF_GROUNDIMAGE9"]
    Groundimage9,
    /// `EF_CODE2_EFFECT_BEGIN`
    #[value_string = "EF_CODE2_EFFECT_BEGIN"]
    #[value = 1200]
    Code2EffectBegin,
    /// `EF_CASTFLOWER`
    #[value_string = "EF_CASTFLOWER"]
    Castflower,
    /// `EF_ROTATEFLOWER`
    #[value_string = "EF_ROTATEFLOWER"]
    Rotateflower,
    /// `EF_FLYUP`
    #[value_string = "EF_FLYUP"]
    Flyup,
    /// `EF_ACTOR_COLOR`
    #[value_string = "EF_ACTOR_COLOR"]
    ActorColor,
    /// `EF_LIGHT_SWORD`
    #[value_string = "EF_LIGHT_SWORD"]
    LightSword,
    /// `EF_LIGHT_BODY`
    #[value_string = "EF_LIGHT_BODY"]
    LightBody,
    /// `EF_LIGHT_RIDE`
    #[value_string = "EF_LIGHT_RIDE"]
    LightRide,
    /// `EF_PRINT_FOOT`
    #[value_string = "EF_PRINT_FOOT"]
    PrintFoot,
    /// `EF_COLOR_SWORD`
    #[value_string = "EF_COLOR_SWORD"]
    ColorSword,
    /// `EF_COLOR_BODY`
    #[value_string = "EF_COLOR_BODY"]
    ColorBody,
    /// `EF_COLOR_RIDE`
    #[value_string = "EF_COLOR_RIDE"]
    ColorRide,
    /// `EF_MOVE_TO_SPRITE`
    #[value_string = "EF_MOVE_TO_SPRITE"]
    MoveToSprite,
    /// `EF_GET_ITEM`
    #[value_string = "EF_GET_ITEM"]
    GetItem,
    /// `EF_LIGHT_ROLESHIELD`
    #[value_string = "EF_LIGHT_ROLESHIELD"]
    LightRoleshield,
    /// `EF_LIGHT_HEAD1`
    #[value_string = "EF_LIGHT_HEAD1"]
    LightHead1,
    /// `EF_LIGHT_HEAD2`
    #[value_string = "EF_LIGHT_HEAD2"]
    LightHead2,
    /// `EF_LIGHT_HEAD3`
    #[value_string = "EF_LIGHT_HEAD3"]
    LightHead3,
    /// `EF_COLOR_HEAD1`
    #[value_string = "EF_COLOR_HEAD1"]
    ColorHead1,
    /// `EF_COLOR_HEAD2`
    #[value_string = "EF_COLOR_HEAD2"]
    ColorHead2,
    /// `EF_COLOR_HEAD3`
    #[value_string = "EF_COLOR_HEAD3"]
    ColorHead3,
    /// `EF_CODE_EFFECT_BEGIN2`
    #[value_string = "EF_CODE_EFFECT_BEGIN2"]
    #[value = 1300]
    CodeEffectBegin2,
    /// `EF_RIPPLE_YELLOW`
    #[value_string = "EF_RIPPLE_YELLOW"]
    RippleYellow,
    /// `EF_RIPPLE_BLACKK`
    #[value_string = "EF_RIPPLE_BLACKK"]
    RippleBlackk,
    /// `EF_RIPPLE_WHITE`
    #[value_string = "EF_RIPPLE_WHITE"]
    RippleWhite,
    /// `EF_RIPPLE_RED`
    #[value_string = "EF_RIPPLE_RED"]
    RippleRed,
    /// `EF_RIPPLE_PURPLE`
    #[value_string = "EF_RIPPLE_PURPLE"]
    RipplePurple,
    /// `EF_AGGREGATION_YELLOW`
    #[value_string = "EF_AGGREGATION_YELLOW"]
    AggregationYellow,
    /// `EF_AGGREGATION_BLACKK`
    #[value_string = "EF_AGGREGATION_BLACKK"]
    AggregationBlackk,
    /// `EF_AGGREGATION_WHITE`
    #[value_string = "EF_AGGREGATION_WHITE"]
    AggregationWhite,
    /// `EF_AGGREGATION_RED`
    #[value_string = "EF_AGGREGATION_RED"]
    AggregationRed,
    /// `EF_AGGREGATION_PURPLE`
    #[value_string = "EF_AGGREGATION_PURPLE"]
    AggregationPurple,
    /// `EF_CODE_EFFECT_END2`
    #[value_string = "EF_CODE_EFFECT_END2"]
    CodeEffectEnd2,
    /// `EF_TEST_EFFECT_BEGIN`
    #[value_string = "EF_TEST_EFFECT_BEGIN"]
    #[value = 2000]
    TestEffectBegin,
    /// `EF_SELECTRING`
    #[value_string = "EF_SELECTRING"]
    Selectring,
    /// `EF_TESTEFFECT`
    #[value_string = "EF_TESTEFFECT"]
    Testeffect,
    /// `EF_TESTBODYLIGHT`
    #[value_string = "EF_TESTBODYLIGHT"]
    Testbodylight,
    /// `EF_ZOOM_IN`
    #[value_string = "EF_ZOOM_IN"]
    ZoomIn,
    /// `EF_ZOOM_OUT`
    #[value_string = "EF_ZOOM_OUT"]
    ZoomOut,
    /// `EF_BLOW_LINE`
    #[value_string = "EF_BLOW_LINE"]
    BlowLine,
    /// `EF_LIGHT_SHIELD`
    #[value_string = "EF_LIGHT_SHIELD"]
    LightShield,
    /// `EF_TYPING`
    #[value_string = "EF_TYPING"]
    Typing,
    /// `EF_SMATK1`
    #[value_string = "EF_SMATK1"]
    Smatk1,
    /// `EF_SMATK2`
    #[value_string = "EF_SMATK2"]
    Smatk2,
    /// `EF_SMATK3`
    #[value_string = "EF_SMATK3"]
    Smatk3,
    /// `EF_SMATK4`
    #[value_string = "EF_SMATK4"]
    Smatk4,
    /// `EF_SMDEF`
    #[value_string = "EF_SMDEF"]
    Smdef,
    /// `EF_MGATTACK1`
    #[value_string = "EF_MGATTACK1"]
    Mgattack1,
    /// `EF_MGATTACK2`
    #[value_string = "EF_MGATTACK2"]
    Mgattack2,
    /// `EF_ALATTACK1`
    #[value_string = "EF_ALATTACK1"]
    Alattack1,
    /// `EF_ALATTACK2`
    #[value_string = "EF_ALATTACK2"]
    Alattack2,
    /// `EF_ALATTACK3`
    #[value_string = "EF_ALATTACK3"]
    Alattack3,
    /// `EF_ALATTACK4`
    #[value_string = "EF_ALATTACK4"]
    Alattack4,
    /// `EF_ALDEF2`
    #[value_string = "EF_ALDEF2"]
    Aldef2,
    /// `EF_ALDEF3`
    #[value_string = "EF_ALDEF3"]
    Aldef3,
    /// `EF_MGDEF1`
    #[value_string = "EF_MGDEF1"]
    Mgdef1,
    /// `EF_MGDEF2`
    #[value_string = "EF_MGDEF2"]
    Mgdef2,
    /// `EF_MGDEF3`
    #[value_string = "EF_MGDEF3"]
    Mgdef3,
    /// `EF_MGDEF4`
    #[value_string = "EF_MGDEF4"]
    Mgdef4,
    /// `EF_DEVIL_RED`
    #[value_string = "EF_DEVIL_RED"]
    DevilRed,
    /// `EF_DECAGILITYBUF`
    #[value_string = "EF_DECAGILITYBUF"]
    Decagilitybuf,
}


impl EffectId {
    pub fn from_skill(skill_id: SkillEnum) -> Option<Self> {
        Some(match skill_id {
            SkillEnum::SmBash => EffectId::Bash,
            SkillEnum::SmProvoke => EffectId::Invenom,
            SkillEnum::SmMagnum => EffectId::Magnumbreak,
            SkillEnum::SmEndure => EffectId::Endure,
            SkillEnum::MgSight => EffectId::Sight,
            SkillEnum::MgSafetywall => EffectId::Glasswall2,
            SkillEnum::MgSoulstrike => EffectId::Soulstrike,
            SkillEnum::MgColdbolt => EffectId::Firesplashhit,
            SkillEnum::MgFrostdiver => EffectId::Frostdiver,
            SkillEnum::MgStonecurse => EffectId::Stonecurse,
            SkillEnum::MgFireball => EffectId::Fireball,
            SkillEnum::MgFirewall => EffectId::Firewall,
            SkillEnum::MgFirebolt => EffectId::Spraypond,
            SkillEnum::MgLightningbolt => EffectId::Lightbolt,
            SkillEnum::MgThunderstorm => EffectId::Thunderstorm,
            SkillEnum::AlRuwach => EffectId::Ruwach,
            SkillEnum::AlPneuma => EffectId::Firepillaron,
            SkillEnum::AlWarp => EffectId::Readyportal,
            SkillEnum::AlHeal => EffectId::Heal,
            SkillEnum::AlIncagi => EffectId::Incagility,
            SkillEnum::AlDecagi => EffectId::Decagility,
            SkillEnum::AlHolywater => EffectId::Aqua,
            SkillEnum::AlCrucis => EffectId::Signum,
            SkillEnum::AlAngelus => EffectId::Angelus,
            SkillEnum::AlBlessing => EffectId::Blessing,
            SkillEnum::AlCure => EffectId::Arrowshot,
            SkillEnum::AcConcentration => EffectId::Spearstabself,
            SkillEnum::AcDouble => EffectId::Bash,
            SkillEnum::TfSteal => EffectId::Steal,
            SkillEnum::TfPoison => EffectId::Pattack,
            SkillEnum::TfDetoxify => EffectId::Poisonattack,
            SkillEnum::KnPierce => EffectId::Shockwave,
            SkillEnum::KnBrandishspear => EffectId::Mvp,
            SkillEnum::KnSpearstab => EffectId::Earthhit,
            SkillEnum::KnSpearboomerang => EffectId::Pierceself,
            SkillEnum::KnTwohandquicken => EffectId::Poisonreact2,
            SkillEnum::KnAutocounter => EffectId::Splasher,
            SkillEnum::KnBowlingbash => EffectId::Shockwavehit,
            SkillEnum::PrImpositio => EffectId::Turnundead,
            SkillEnum::PrSuffragium => EffectId::Lexaeterna,
            SkillEnum::PrAspersio => EffectId::Sanctuary,
            SkillEnum::PrBenedictio => EffectId::Stormgust,
            SkillEnum::PrSanctuary => EffectId::Pierce,
            SkillEnum::PrSlowpoison => EffectId::Freeze,
            SkillEnum::PrStrecovery => EffectId::Magnificat,
            SkillEnum::PrKyrie => EffectId::Gaspush,
            SkillEnum::PrMagnificat => EffectId::Icewall,
            SkillEnum::PrGloria => EffectId::Bowlingbash,
            SkillEnum::PrLexdivina => EffectId::Impositio,
            SkillEnum::PrTurnundead => EffectId::Bowlingself,
            SkillEnum::WzFirepillar => EffectId::Yufitelhit,
            SkillEnum::WzSightrasher => EffectId::Lockon,
            SkillEnum::WzMeteor => EffectId::Lord,
            SkillEnum::WzJupitel => EffectId::Benedictio,
            SkillEnum::WzVermilion => EffectId::Suffragium,
            SkillEnum::WzWaterball => EffectId::Magnus,
            SkillEnum::WzIcewall => EffectId::Sphere,
            SkillEnum::WzFrostnova => EffectId::Frostdiver2,
            SkillEnum::WzStormgust => EffectId::Lexdivina,
            SkillEnum::WzEarthspike => EffectId::Resurrection,
            SkillEnum::WzHeavendrive => EffectId::Sandman,
            SkillEnum::WzQuagmire => EffectId::Yufitel,
            SkillEnum::BsRepairweapon => EffectId::Flasher,
            SkillEnum::BsHammerfall => EffectId::Removetrap,
            SkillEnum::BsAdrenaline => EffectId::Firepillar,
            SkillEnum::BsWeaponperfect => EffectId::Repairweapon,
            SkillEnum::BsOverthrust => EffectId::Enchantpoison,
            SkillEnum::BsMaximize => EffectId::Crashearth,
            SkillEnum::HtSkidtrap => EffectId::Provoke,
            SkillEnum::HtShockwave => EffectId::Heavensdrive,
            SkillEnum::HtSandman => EffectId::Slowpoison,
            SkillEnum::HtFlasher => EffectId::Firepillarbomb,
            SkillEnum::HtFreezingtrap => EffectId::Blastminebomb,
            SkillEnum::HtBlastmine => EffectId::Maxpower,
            SkillEnum::HtClaymoretrap => EffectId::Blastmine,
            SkillEnum::HtRemovetrap => EffectId::Hasteup,
            SkillEnum::HtBlitzbeat => EffectId::Kyrie,
            SkillEnum::HtDetecting => EffectId::Waterball,
            SkillEnum::HtSpringtrap => EffectId::Bubble,
            SkillEnum::AsCloaking => EffectId::Waterball2,
            SkillEnum::AsSonicblow => EffectId::Revive,
            SkillEnum::AsGrimtooth => EffectId::Cloaking,
            SkillEnum::AsEnchantpoison => EffectId::Pattack,
            SkillEnum::AsPoisonreact => EffectId::Grimtooth,
            SkillEnum::AsVenomdust => EffectId::Sonicblow,
            SkillEnum::AsSplasher => EffectId::Poisonreact,
            SkillEnum::NvFirstaid => EffectId::Firstaid,
            SkillEnum::TfSprinklesand => EffectId::Sprinklesand,
            SkillEnum::TfThrowstone => EffectId::Forestlight,
            SkillEnum::McCartrevolution => EffectId::Tamingfailed,
            SkillEnum::McLoud => EffectId::Loud,
            SkillEnum::AlHolylight => EffectId::Bowlingself,
            SkillEnum::MgEnergycoat => EffectId::Tamingsuccess,
            SkillEnum::NpcPiercingatt => EffectId::Shockwave,
            SkillEnum::NpcChangewater => EffectId::Changecold,
            SkillEnum::NpcChangeground => EffectId::Changeearth,
            SkillEnum::NpcChangefire => EffectId::Changefire,
            SkillEnum::NpcChangewind => EffectId::Changewind,
            SkillEnum::NpcChangeholy => EffectId::Chaingeholy,
            SkillEnum::NpcChangedarkness => EffectId::Changedark,
            SkillEnum::NpcCriticalslash => EffectId::Bash,
            SkillEnum::NpcGuidedattack => EffectId::Comboattack3,
            SkillEnum::NpcSelfdestruction => EffectId::Hitdark,
            SkillEnum::NpcSuicide => EffectId::Magicalatthit,
            SkillEnum::NpcPoison => EffectId::Comboattack4,
            SkillEnum::NpcSilenceattack => EffectId::Comboattack5,
            SkillEnum::NpcStunattack => EffectId::Guidedattack,
            SkillEnum::NpcSleepattack => EffectId::Silenceattack,
            SkillEnum::NpcWaterattack => EffectId::Firesplashhit,
            SkillEnum::NpcGroundattack => EffectId::Brandish2,
            SkillEnum::NpcFireattack => EffectId::Spraypond,
            SkillEnum::NpcPoisonattack => EffectId::Coldhit,
            SkillEnum::NpcHolyattack => EffectId::Bowlingself,
            SkillEnum::NpcDarknessattack => EffectId::Mentalbreak,
            SkillEnum::NpcTelekinesisattack => EffectId::Stunattack,
            SkillEnum::NpcMagicalattack => EffectId::Changepoison,
            SkillEnum::NpcProvocation => EffectId::Invenom,
            SkillEnum::NpcKeeping => EffectId::Keeping,
            SkillEnum::NpcDarkbreath => EffectId::Darkbreath,
            SkillEnum::NpcDefender => EffectId::Deffender,
            SkillEnum::RgBackstap => EffectId::Stripshield,
            SkillEnum::RgRaid => EffectId::Striparmor,
            SkillEnum::RgStripweapon => EffectId::Stealcoin,
            SkillEnum::RgIntimidate => EffectId::Grandcross,
            SkillEnum::AmDemonstration => EffectId::PokjukSound,
            SkillEnum::AmAcidterror => EffectId::Pokjuk,
            SkillEnum::AmPotionpitcher => EffectId::Throwitem,
            SkillEnum::AmCpWeapon => EffectId::Throwitem2,
            SkillEnum::AmCpShield => EffectId::Throwitem2,
            SkillEnum::AmCpArmor => EffectId::Throwitem2,
            SkillEnum::AmCpHelm => EffectId::Throwitem2,
            SkillEnum::CrAutoguard => EffectId::Blind,
            SkillEnum::CrShieldcharge => EffectId::Holycross,
            SkillEnum::CrReflectshield => EffectId::Providence,
            SkillEnum::CrHolycross => EffectId::Magicrod,
            SkillEnum::CrGrandcross => EffectId::Volcano,
            SkillEnum::CrDefender => EffectId::Potionpillar,
            SkillEnum::MoTripleattack => EffectId::Foot2,
            SkillEnum::MoInvestigate => EffectId::Teihit1x,
            SkillEnum::MoFingeroffensive => EffectId::Teihit2,
            SkillEnum::MoChaincombo => EffectId::Teihit1,
            SkillEnum::SaMagicrod => EffectId::Fastmove,
            SkillEnum::SaSpellbreaker => EffectId::Cloud3,
            SkillEnum::SaFlamelauncher => EffectId::Steelbody,
            SkillEnum::SaFrostweapon => EffectId::Reflectshield,
            SkillEnum::SaSeismicweapon => EffectId::Flamelauncher,
            SkillEnum::SaVolcano => EffectId::Landprotector,
            SkillEnum::SaDeluge => EffectId::BottomVo,
            SkillEnum::SaViolentgale => EffectId::BottomDe,
            SkillEnum::SaLandprotector => EffectId::BottomVi,
            SkillEnum::SaDispell => EffectId::Spellbreaker,
            SkillEnum::BdLullaby => EffectId::Chaincombo,
            SkillEnum::BdRichmankim => EffectId::RgCoin,
            SkillEnum::BdEternalchaos => EffectId::Backstap,
            SkillEnum::BdDrumbattlefield => EffectId::Teihit3,
            SkillEnum::BdRingnibelungen => EffectId::BottomDissonance,
            SkillEnum::BdRokisweil => EffectId::BottomLullaby,
            SkillEnum::BdIntoabyss => EffectId::BottomDrumbattlefield,
            SkillEnum::BdSiegfried => EffectId::BottomRingnibelungen,
            SkillEnum::BaWhistle => EffectId::BottomRokisweil,
            SkillEnum::BaAssassincross => EffectId::BottomIntoabyss,
            SkillEnum::BaPoembragi => EffectId::BottomSiegfried,
            SkillEnum::BaAppleidun => EffectId::BottomWhistle,
            SkillEnum::DcHumming => EffectId::BottomPoembragi,
            SkillEnum::DcDontforgetme => EffectId::BottomAppleidun,
            SkillEnum::DcFortunekiss => EffectId::BottomUglydance,
            SkillEnum::DcServiceforyou => EffectId::BottomFortunekiss,
            SkillEnum::ItmTomahawk => EffectId::Mochi,
            SkillEnum::NpcDarkcross => EffectId::Foot3,
            SkillEnum::NpcDarkstrike => EffectId::Foot4,
            SkillEnum::NpcDarkthunder => EffectId::Benedictio,
            SkillEnum::NpcStop => EffectId::Grandcross2,
            SkillEnum::NpcPowerup => EffectId::NpcStop,
            SkillEnum::LkAurablade => EffectId::Vallentine2,
            SkillEnum::LkParrying => EffectId::Blind,
            SkillEnum::HpAssumptio => EffectId::Attackenergy,
            SkillEnum::HpBasilica => EffectId::BottomGospel,
            SkillEnum::HwMagiccrasher => EffectId::Bash3d2,
            SkillEnum::HwMagicpower => EffectId::Bash,
            SkillEnum::PaPressure => EffectId::Level994,
            SkillEnum::PaSacrifice => EffectId::Vallentine,
            SkillEnum::PaGospel => EffectId::Bash3d,
            SkillEnum::ChTigerfist => EffectId::Teihit1,
            SkillEnum::ChChaincrush => EffectId::Ef05val,
            SkillEnum::PfHpconversion => EffectId::Magiccrasher,
            SkillEnum::PfSoulchange => EffectId::Lightsphere2,
            SkillEnum::PfSoulburn => EffectId::Magiccrasher2,
            SkillEnum::AscEdp => EffectId::Flowercast3,
            SkillEnum::AscBreaker => EffectId::Ghost,
            SkillEnum::SnSight => EffectId::Energydrain3,
            SkillEnum::SnFalconassault => EffectId::Linelink2,
            SkillEnum::SnSharpshooting => EffectId::Linklight,
            SkillEnum::SnWindwalk => EffectId::Truesight,
            SkillEnum::WsMeltdown => EffectId::Falconassault,
            SkillEnum::WsCartboost => EffectId::Tripleattack2,
            SkillEnum::StChasewalk => EffectId::Slim2,
            SkillEnum::StRejectsword => EffectId::Portal4,
            SkillEnum::CgArrowvulcan => EffectId::Meltdown,
            SkillEnum::CgMoonlit => EffectId::Cartboost,
            SkillEnum::CgMarionette => EffectId::Rejectsword,
            SkillEnum::LkSpiralpierce => EffectId::Joblvup50,
            SkillEnum::LkHeadcrush => EffectId::Pinkbody,
            SkillEnum::LkJointbeat => EffectId::Level995,
            SkillEnum::HwNapalmvulcan => EffectId::Level996,
            SkillEnum::ChSoulcollect => EffectId::Bash3d3,
            SkillEnum::PfMindbreaker => EffectId::Bash3d4,
            SkillEnum::PfMemorize => EffectId::Piercebody,
            SkillEnum::PfSpiderweb => EffectId::Napalmvalcan,
            SkillEnum::AscMeteorassault => EffectId::Soulburn,
            SkillEnum::WeBaby => EffectId::BottomFogwall,
            SkillEnum::TkRun => EffectId::Assumptio2,
            SkillEnum::TkStormkick => EffectId::Electric,
            SkillEnum::TkDownkick => EffectId::Rainbow,
            SkillEnum::TkTurnkick => EffectId::Peong,
            SkillEnum::TkCounter => EffectId::Tanji2,
            SkillEnum::TkJumpkick => EffectId::Halfsphere,
            SkillEnum::TkHighjump => EffectId::Baby,
            SkillEnum::SgFeel => EffectId::Hitline4,
            SkillEnum::SgSunWarm => EffectId::Doublegumgang2,
            SkillEnum::SgMoonWarm => EffectId::Doublegumgang2,
            SkillEnum::SgStarWarm => EffectId::Doublegumgang2,
            SkillEnum::SgSunComfort => EffectId::Attackenergy2,
            SkillEnum::SgMoonComfort => EffectId::Attackenergy2,
            SkillEnum::SgStarComfort => EffectId::Attackenergy2,
            SkillEnum::SgHate => EffectId::Devil10,
            SkillEnum::AmBerserkpitcher => EffectId::Potion,
            SkillEnum::SlKaizel => EffectId::M04,
            SkillEnum::SlKaahi => EffectId::Throwitem4,
            SkillEnum::SlKaupe => EffectId::Bluebody,
            SkillEnum::SlKaite => EffectId::Airtexture,
            SkillEnum::SlStin => EffectId::Hated,
            SkillEnum::SlStun => EffectId::Sma,
            SkillEnum::SlSma => EffectId::BlueHit,
            SkillEnum::SlSwoo => EffectId::M03,
            SkillEnum::SlSke => EffectId::Asurabody,
            SkillEnum::StPreserve => EffectId::Edp,
            SkillEnum::StFullstrip => EffectId::Lamadan,
            SkillEnum::PfDoublecasting => EffectId::Cartter,
            SkillEnum::HwGanbantein => EffectId::Defender,
            SkillEnum::WsCarttermination => EffectId::Cloud4,
            SkillEnum::CgLongingfreedom => EffectId::Slim,
            SkillEnum::CgHermode => EffectId::GiExplosion,
            SkillEnum::CgTarotcard => EffectId::Slim,
            SkillEnum::CrAciddemonstration => EffectId::Tarotcard11,
            SkillEnum::AmTwilight1 => EffectId::Shieldboomerang2,
            SkillEnum::AmTwilight2 => EffectId::RgCoin2,
            SkillEnum::AmTwilight3 => EffectId::Guard2,
            SkillEnum::GsTripleaction => EffectId::Magicalbullet,
            SkillEnum::GsBullseye => EffectId::Spreadattack,
            SkillEnum::GsMadnesscancel => EffectId::Stin4,
            SkillEnum::GsAdjustment => EffectId::Thunderstorm2,
            SkillEnum::GsIncreasing => EffectId::NpcStop,
            SkillEnum::GsMagicalbullet => EffectId::PoisonS,
            SkillEnum::GsTracking => EffectId::FlareS,
            SkillEnum::GsDisarm => EffectId::Chemical4,
            SkillEnum::GsRapidshower => EffectId::BlindS,
            SkillEnum::GsDesperado => EffectId::Issen,
            SkillEnum::GsGatlingfever => EffectId::Thunderstorm2,
            SkillEnum::GsDust => EffectId::Stin5,
            SkillEnum::GsFullbuster => EffectId::ItemRain,
            SkillEnum::GsSpreadattack => EffectId::FreezingS,
            SkillEnum::NjSyuriken => EffectId::CookingFail,
            SkillEnum::NjKunai => EffectId::TempOk,
            SkillEnum::NjHuuma => EffectId::TempFail,
            SkillEnum::NjZenynage => EffectId::Hapgyeok,
            SkillEnum::NjTatamigaeshi => EffectId::RgCoin3,
            SkillEnum::NjKasumikiri => EffectId::Bash3d5,
            SkillEnum::NjKirikage => EffectId::MadnessRed,
            SkillEnum::NjBunsinjyutsu => EffectId::Throwitem7,
            SkillEnum::NjKouenka => EffectId::Throwitem8,
            SkillEnum::NjKaensin => EffectId::Kirikage,
            SkillEnum::NjBakuenryu => EffectId::Tatami,
            SkillEnum::NjHyousensou => EffectId::Throwitem9,
            SkillEnum::NjSuiton => EffectId::Throwitem10,
            SkillEnum::NjHyousyouraku => EffectId::Kasumikiri,
            SkillEnum::NjHuujin => EffectId::Bunsinjyutsu,
            SkillEnum::NjRaigekisai => EffectId::Kouenka,
            SkillEnum::NjIssen => EffectId::Chookgi3,
            SkillEnum::NpcEarthquake => EffectId::WhiteNumber,
            SkillEnum::NpcFirebreath => EffectId::Spraypond,
            SkillEnum::NpcDragonfear => EffectId::PinkNumber,
            SkillEnum::NpcPulsestrike => EffectId::Soulburn,
            SkillEnum::NpcWidefreeze => EffectId::Lexdivina,
            SkillEnum::NpcWidebleeding => EffectId::BubbleDrop,
            SkillEnum::NpcEvilland => EffectId::Wideconfuse,
            SkillEnum::NpcCriticalwound => EffectId::CrystalBlue,
            SkillEnum::AllWewish => EffectId::Teihit1reverse,
            SkillEnum::CrShrink => EffectId::Food03,
            SkillEnum::AsVenomknife => EffectId::Food04,
            SkillEnum::RgCloseconfine => EffectId::Food06,
            SkillEnum::WzSightblaster => EffectId::Food05,
            SkillEnum::SaElementwater => EffectId::Reflectshield,
            SkillEnum::BaPangvoice => EffectId::Quakebody4,
            SkillEnum::DcWinkcharm => EffectId::Firehit2,
            SkillEnum::MoBalkyoung => EffectId::Night,
            SkillEnum::SaElementground => EffectId::Flamelauncher,
            SkillEnum::SaElementfire => EffectId::Steelbody,
            SkillEnum::HlifChange => EffectId::Piercebody,
            SkillEnum::HamiCastle => EffectId::Hflimoon2,
            SkillEnum::HamiDefence => EffectId::Hflimoon1,
            SkillEnum::HamiBloodlust => EffectId::Hflimoon3,
            SkillEnum::HfliFleet => EffectId::Fastbluefall90,
            SkillEnum::HfliSpeed => EffectId::Fastbluefall90,
            SkillEnum::HvanExplosion => EffectId::Hitdark,
            _ => return None,
        })
    }
}
