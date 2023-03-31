use crate::*;
use crate::look::LookType::Job;

pub const JOB_BASE_MASK: u64 = 0xff;
pub const JOB_UPPER_MASK: u64 = 0xfff;
pub const JOB_2_1_MASK: u64 = 256;
pub const JOB_2_2_MASK: u64 = 512;
pub const JOB_TRANS_MASK: u64 = 4096;
pub const JOB_TRANS_2_1_MASK: u64 = JOB_TRANS_MASK | JOB_2_1_MASK;
pub const JOB_TRANS_2_2_MASK: u64 = JOB_TRANS_MASK | JOB_2_2_MASK;
pub const JOB_BABY_MASK: u64 = 8192;
pub const JOB_OTHER: u64 = 16384;

#[derive(WithNumberValue, WithStringValue, Debug, PartialEq, Clone)]
pub enum JobName {
    #[value = 0]
    Novice,
    Swordsman,
    Mage,
    Archer,
    Acolyte,
    Merchant,
    Thief,
    Knight,
    Priest,
    Wizard,
    Blacksmith,
    Hunter,
    Assassin,
    Crusader,
    Monk,
    Sage,
    Rogue,
    Alchemist,
    Bard,
    Dancer,
    Wedding,
    #[value = 23]
    SuperNovice,
    Gunslinger,
    Ninja,
    Xmas,
    Summer,
    #[value_string = "Novice High"]
    #[value = 4001]
    NoviceHigh,
    #[value_string = "Swordsman High"]
    SwordsmanHigh,
    #[value_string = "Mage High"]
    MageHigh,
    #[value_string = "Archer High"]
    ArcherHigh,
    #[value_string = "Acolyte High"]
    AcolyteHigh,
    #[value_string = "Merchant High"]
    MerchantHigh,
    #[value_string = "Thief High"]
    ThiefHigh,
    LordKnight,
    HighPriest,
    HighWizard,
    Whitesmith,
    Sniper,
    AssassinCross,
    #[value = 4015]
    Paladin,
    Champion,
    Professor,
    Stalker,
    Creator,
    Clown,
    Gypsy,
    #[value = 4023]
    BabyNovice,
    BabySwordman,
    BabyMage,
    BabyArcher,
    BabyAcolyte,
    BabyMerchant,
    BabyThief,
    BabyKnight,
    BabyPriest,
    BabyWizard,
    BabyBlacksmith,
    BabyHunter,
    BabyAssassin,
    BabyCrusader,
    BabyMonk,
    BabySage,
    BabyRogue,
    BabyAlchemist,
    BabyBard,
    BabyDancer,
    #[value = 4045]
    SuperBaby,
    Taekwon,
    StarGladiator,
    SoulLinker,
}

impl JobName {
    pub fn is_rebirth(&self) -> bool {
        match self {
            JobName::NoviceHigh | JobName::SwordsmanHigh | JobName::MageHigh | JobName::ArcherHigh
            | JobName::AcolyteHigh | JobName::MerchantHigh | JobName::ThiefHigh | JobName::LordKnight | JobName::HighPriest
            | JobName::HighWizard | JobName::Whitesmith | JobName::Sniper | JobName::AssassinCross | JobName::Paladin
            | JobName::Champion | JobName::Professor | JobName::Stalker | JobName::Creator | JobName::Clown | JobName::Gypsy => true,
            _ => false,
        }
    }
    pub fn is_novice(&self) -> bool {
        match self {
            JobName::NoviceHigh | JobName::Novice | JobName::SuperNovice => true,
            _ => false
        }
    }
    pub fn is_first_class(&self) -> bool {
        match self {
            JobName::Swordsman | JobName::Mage | JobName::Archer | JobName::Acolyte
            | JobName::Merchant | JobName::Thief | JobName::BabySwordman | JobName::BabyMage
            | JobName::BabyArcher | JobName::BabyAcolyte | JobName::BabyMerchant | JobName::BabyThief
            | JobName::SwordsmanHigh | JobName::MageHigh | JobName::ArcherHigh
            | JobName::AcolyteHigh | JobName::MerchantHigh | JobName::ThiefHigh => true,
            _ => false,
        }
    }
    pub fn is_second_class(&self) -> bool {
        match self {
            JobName::Knight | JobName::Priest | JobName::Wizard
            | JobName::Blacksmith | JobName::Hunter | JobName::Assassin | JobName::Crusader | JobName::Monk
            | JobName::Sage | JobName::Rogue | JobName::Alchemist | JobName::Bard | JobName::Dancer
            | JobName::BabyKnight | JobName::BabyPriest | JobName::BabyWizard | JobName::BabyBlacksmith
            | JobName::BabyHunter | JobName::BabyAssassin | JobName::BabyCrusader | JobName::BabyMonk
            | JobName::BabySage | JobName::BabyRogue | JobName::BabyAlchemist
            | JobName::LordKnight | JobName::HighPriest
            | JobName::HighWizard | JobName::Whitesmith | JobName::Sniper | JobName::AssassinCross | JobName::Paladin
            | JobName::Champion | JobName::Professor | JobName::Stalker | JobName::Creator | JobName::Clown | JobName::Gypsy => true,
            _ => false,
        }
    }
    pub fn is_gunslinger_ninja(&self) -> bool {
        match self {
            JobName::Gunslinger | JobName::Ninja => true,
            _ => false,
        }
    }
    pub fn is_taekwon(&self) -> bool {
        match self {
            JobName::Taekwon | JobName::StarGladiator | JobName::SoulLinker => true,
            _ => false,
        }
    }

    pub fn mask(&self) -> u64 {
        let base_mask = match self {
            // 1-1
            JobName::Novice | JobName::SuperNovice => 1,
            JobName::Swordsman | JobName::Knight | JobName::Crusader | JobName::BabyKnight | JobName::BabyCrusader | JobName::LordKnight | JobName::Paladin => 2,
            JobName::Mage | JobName::Wizard | JobName::Sage | JobName::BabyWizard | JobName::BabySage | JobName::HighWizard | JobName::Professor => 4,
            JobName::Archer | JobName::Hunter | JobName::Bard | JobName::Dancer | JobName::BabyHunter | JobName::BabyBard | JobName::BabyDancer | JobName::Sniper | JobName::Clown | JobName::Gypsy => 8,
            JobName::Acolyte | JobName::Priest | JobName::Monk | JobName::BabyPriest | JobName::BabyMonk | JobName::HighPriest | JobName::Champion => 16,
            JobName::Merchant | JobName::Blacksmith | JobName::Alchemist | JobName::BabyBlacksmith | JobName::BabyAlchemist | JobName::Creator | JobName::Whitesmith => 32,
            JobName::Thief | JobName::Assassin | JobName::Rogue | JobName::BabyAssassin | JobName::BabyRogue | JobName::AssassinCross | JobName::Stalker => 64,
            JobName::Taekwon | JobName::StarGladiator | JobName::SoulLinker => 128,
            _ => 0,
        };

        let class_mask = match self {
            // 2-1
            JobName::SuperNovice
            | JobName::Knight
            | JobName::Wizard
            | JobName::Hunter
            | JobName::Priest
            | JobName::Blacksmith
            | JobName::Assassin
            | JobName::StarGladiator => JOB_2_1_MASK,

            // 2-2
            JobName::Crusader
            | JobName::Sage
            | JobName::Bard
            | JobName::Dancer
            | JobName::Monk
            | JobName::Alchemist
            | JobName::Rogue
            | JobName::SoulLinker => JOB_2_2_MASK,

            // trans 1-1
            JobName::NoviceHigh
            | JobName::SwordsmanHigh
            | JobName::MageHigh
            | JobName::ArcherHigh
            | JobName::AcolyteHigh
            | JobName::MerchantHigh
            | JobName::ThiefHigh => JOB_TRANS_MASK,

            // trans 2-1
            JobName::LordKnight
            | JobName::HighWizard
            | JobName::Sniper
            | JobName::HighPriest
            | JobName::Whitesmith
            | JobName::AssassinCross => JOB_TRANS_2_1_MASK,

            // trans 2-2
            JobName::Paladin
            | JobName::Professor
            | JobName::Clown
            | JobName::Gypsy
            | JobName::Champion
            | JobName::Creator
            | JobName::Stalker => JOB_TRANS_2_2_MASK,

            // Baby
            JobName::BabyNovice
            | JobName::BabySwordman
            | JobName::BabyMage
            | JobName::BabyArcher
            | JobName::BabyAcolyte
            | JobName::BabyMerchant
            | JobName::BabyThief => JOB_BABY_MASK,
            // Baby 2-1
            JobName::BabyKnight
            | JobName::BabyWizard
            | JobName::BabyHunter
            | JobName::BabyPriest
            | JobName::BabyBlacksmith
            | JobName::BabyAssassin => JOB_BABY_MASK | JOB_2_1_MASK,
            // Baby 2-2
            JobName::BabyCrusader
            | JobName::BabySage
            | JobName::BabyBard | JobName::BabyDancer
            | JobName::BabyMonk
            | JobName::BabyAlchemist
            | JobName::BabyRogue => JOB_BABY_MASK | JOB_2_2_MASK,

            JobName::Wedding => JOB_OTHER | 1,
            JobName::Gunslinger => JOB_OTHER | 2,
            JobName::Ninja => JOB_OTHER | 4,
            JobName::Xmas => JOB_OTHER | 8,
            JobName::Summer => JOB_OTHER | 16,
            JobName::SuperBaby => JOB_OTHER | 32,
            _ => 0
        };

        base_mask | class_mask
    }

    pub fn from_mask(mask: u64, is_male: bool) -> Option<Self> {
        match mask {
            1 => Some(JobName::Novice),
            2 => Some(JobName::Swordsman),
            4 => Some(JobName::Mage),
            8 => Some(JobName::Archer),
            16 => Some(JobName::Acolyte),
            32 => Some(JobName::Merchant),
            64 => Some(JobName::Thief),
            128 => Some(JobName::Taekwon),

            (JOB_2_1_MASK | 1) => Some(JobName::SuperNovice),
            (JOB_2_1_MASK | 2) => Some(JobName::Knight),
            (JOB_2_1_MASK | 4) => Some(JobName::Wizard),
            (JOB_2_1_MASK | 8) => Some(JobName::Hunter),
            (JOB_2_1_MASK | 16) => Some(JobName::Priest),
            (JOB_2_1_MASK | 32) => Some(JobName::Blacksmith),
            (JOB_2_1_MASK | 64) => Some(JobName::Assassin),
            (JOB_2_1_MASK | 128) => Some(JobName::StarGladiator),

            (JOB_2_2_MASK | 2) => Some(JobName::Crusader),
            (JOB_2_2_MASK | 4) => Some(JobName::Sage),
            (JOB_2_2_MASK | 8) => { if is_male { Some(JobName::Bard) } else { Some(JobName::Dancer) } }
            (JOB_2_2_MASK | 16) => Some(JobName::Monk),
            (JOB_2_2_MASK | 32) => Some(JobName::Alchemist),
            (JOB_2_2_MASK | 64) => Some(JobName::Rogue),
            (JOB_2_2_MASK | 128) => Some(JobName::SoulLinker),

            (JOB_TRANS_MASK | 1) => Some(JobName::NoviceHigh),
            (JOB_TRANS_MASK | 2) => Some(JobName::SwordsmanHigh),
            (JOB_TRANS_MASK | 4) => Some(JobName::MageHigh),
            (JOB_TRANS_MASK | 8) => Some(JobName::ArcherHigh),
            (JOB_TRANS_MASK | 16) => Some(JobName::AcolyteHigh),
            (JOB_TRANS_MASK | 32) => Some(JobName::MerchantHigh),
            (JOB_TRANS_MASK | 64) => Some(JobName::ThiefHigh),

            (JOB_TRANS_2_1_MASK | 2) => Some(JobName::LordKnight),
            (JOB_TRANS_2_1_MASK | 4) => Some(JobName::HighWizard),
            (JOB_TRANS_2_1_MASK | 8) => Some(JobName::Sniper),
            (JOB_TRANS_2_1_MASK | 16) => Some(JobName::HighPriest),
            (JOB_TRANS_2_1_MASK | 32) => Some(JobName::Whitesmith),
            (JOB_TRANS_2_1_MASK | 64) => Some(JobName::AssassinCross),

            (JOB_TRANS_2_2_MASK | 2) => Some(JobName::Paladin),
            (JOB_TRANS_2_2_MASK | 4) => Some(JobName::Professor),
            (JOB_TRANS_2_2_MASK | 8) => { if is_male { Some(JobName::Clown) } else { Some(JobName::Gypsy) } }
            (JOB_TRANS_2_2_MASK | 16) => Some(JobName::Champion),
            (JOB_TRANS_2_2_MASK | 32) => Some(JobName::Creator),
            (JOB_TRANS_2_2_MASK | 64) => Some(JobName::Stalker),

            (JOB_BABY_MASK | 1) => Some(JobName::BabyNovice),
            (JOB_BABY_MASK | 2) => Some(JobName::BabySwordman),
            (JOB_BABY_MASK | 4) => Some(JobName::BabyMage),
            (JOB_BABY_MASK | 8) => Some(JobName::BabyArcher),
            (JOB_BABY_MASK | 16) => Some(JobName::BabyAcolyte),
            (JOB_BABY_MASK | 32) => Some(JobName::BabyMerchant),
            (JOB_BABY_MASK | 64) => Some(JobName::BabyThief),

            (JOB_BABY_MASK | JOB_2_1_MASK | 2) => Some(JobName::BabyKnight),
            (JOB_BABY_MASK | JOB_2_1_MASK | 4) => Some(JobName::BabyWizard),
            (JOB_BABY_MASK | JOB_2_1_MASK | 8) => Some(JobName::BabyHunter),
            (JOB_BABY_MASK | JOB_2_1_MASK | 16) => Some(JobName::BabyPriest),
            (JOB_BABY_MASK | JOB_2_1_MASK | 32) => Some(JobName::BabyBlacksmith),
            (JOB_BABY_MASK | JOB_2_1_MASK | 64) => Some(JobName::BabyAssassin),

            (JOB_BABY_MASK | JOB_2_2_MASK | 2) => Some(JobName::BabyCrusader),
            (JOB_BABY_MASK | JOB_2_2_MASK | 4) => Some(JobName::BabySage),
            (JOB_BABY_MASK | JOB_2_2_MASK | 8) => { if is_male { Some(JobName::BabyBard) } else { Some(JobName::BabyDancer) } }
            (JOB_BABY_MASK | JOB_2_2_MASK | 16) => Some(JobName::BabyMonk),
            (JOB_BABY_MASK | JOB_2_2_MASK | 32) => Some(JobName::BabyAlchemist),
            (JOB_BABY_MASK | JOB_2_2_MASK | 64) => Some(JobName::BabyRogue),

            (JOB_OTHER | 1) => Some(JobName::Wedding),
            (JOB_OTHER | 2) => Some(JobName::Gunslinger),
            (JOB_OTHER | 4) => Some(JobName::Ninja),
            (JOB_OTHER | 8) => Some(JobName::Xmas),
            (JOB_OTHER | 16) => Some(JobName::Summer),
            (JOB_OTHER | 32) => Some(JobName::SuperBaby),
            _ => None
        }
    }
}

#[derive(WithStringValue, WithMaskValueU64)]
pub enum EquipClassFlag {
    #[mask_value = 1]
    Novice,
    Acolyte,
    Priest,
    Monk,
    Mage,
    Wizard,
    Sage,
    Thief,
    Assassin,
    Rogue,
    Archer,
    Hunter,
    Bard,
    Dancer,
    Swordsman,
    Crusader,
    Knight,
    Merchant,
    Alchemist,
    Blacksmith,
    Gunslinger,
    Ninja,
    SoulLinker,
    StarGladiator,
    SuperNovice,
    Taekwon,
    #[mask_all]
    All,
}

impl EquipClassFlag {
    pub fn flag_from_job_name(job_name: JobName) -> u64 {
        match job_name {
            JobName::Novice => EquipClassFlag::Novice.as_flag(),
            JobName::Acolyte => EquipClassFlag::Acolyte.as_flag(),
            JobName::Priest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Monk => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Mage => EquipClassFlag::Mage.as_flag(),
            JobName::Wizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Sage => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Thief => EquipClassFlag::Thief.as_flag(),
            JobName::Assassin => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Rogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Archer => EquipClassFlag::Archer.as_flag(),
            JobName::Hunter => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Bard => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Dancer => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Swordsman => EquipClassFlag::Swordsman.as_flag(),
            JobName::Crusader => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::Knight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::Merchant => EquipClassFlag::Merchant.as_flag(),
            JobName::Alchemist => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Blacksmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Gunslinger => EquipClassFlag::Gunslinger.as_flag(),
            JobName::Ninja => EquipClassFlag::Ninja.as_flag(),
            JobName::SoulLinker => EquipClassFlag::SoulLinker.as_flag(),
            JobName::StarGladiator => EquipClassFlag::StarGladiator.as_flag(),
            JobName::SuperNovice => EquipClassFlag::SuperNovice.as_flag(),
            JobName::Taekwon => EquipClassFlag::Taekwon.as_flag(),
            JobName::Wedding => 0,
            JobName::Xmas => 0,
            JobName::Summer => 0,
            JobName::NoviceHigh => EquipClassFlag::Novice.as_flag(),
            JobName::SwordsmanHigh => EquipClassFlag::Swordsman.as_flag(),
            JobName::MageHigh => EquipClassFlag::Mage.as_flag(),
            JobName::ArcherHigh => EquipClassFlag::Archer.as_flag(),
            JobName::AcolyteHigh => EquipClassFlag::Acolyte.as_flag(),
            JobName::MerchantHigh => EquipClassFlag::Merchant.as_flag(),
            JobName::ThiefHigh => EquipClassFlag::Thief.as_flag(),
            JobName::HighPriest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::HighWizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Whitesmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Sniper => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::AssassinCross => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Champion => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Professor => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Stalker => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Creator => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::Clown => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Gypsy => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Paladin => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::LordKnight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyNovice => EquipClassFlag::Novice.as_flag(),
            JobName::BabySwordman => EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyMage => EquipClassFlag::Mage.as_flag(),
            JobName::BabyArcher => EquipClassFlag::Archer.as_flag(),
            JobName::BabyAcolyte => EquipClassFlag::Acolyte.as_flag(),
            JobName::BabyMerchant => EquipClassFlag::Merchant.as_flag(),
            JobName::BabyThief => EquipClassFlag::Thief.as_flag(),
            JobName::BabyKnight => EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyPriest => EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::BabyWizard => EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::BabyBlacksmith => EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::BabyHunter => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::BabyAssassin => EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::BabyCrusader => EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyMonk => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::BabySage => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::BabyRogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::BabyAlchemist => EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag(),
            JobName::SuperBaby => 0,
            JobName::BabyBard => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::BabyDancer => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
        }
    }
}