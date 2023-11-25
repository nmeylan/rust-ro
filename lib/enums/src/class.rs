use crate::*;

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
    #[value = 14]
    Crusader,
    Monk,
    Sage,
    Rogue,
    Alchemist,
    Bard,
    Dancer,
    Wedding,
    #[value = 23]
    #[value_string = "Super Novice"]
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
    BabySwordsman,
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
    #[value = 4037]
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
    #[value = 4049]
    SoulLinker,
}

impl JobName {
    pub fn is_rebirth(&self) -> bool {
        matches!(
            self,
            JobName::NoviceHigh
                | JobName::SwordsmanHigh
                | JobName::MageHigh
                | JobName::ArcherHigh
                | JobName::AcolyteHigh
                | JobName::MerchantHigh
                | JobName::ThiefHigh
                | JobName::LordKnight
                | JobName::HighPriest
                | JobName::HighWizard
                | JobName::Whitesmith
                | JobName::Sniper
                | JobName::AssassinCross
                | JobName::Paladin
                | JobName::Champion
                | JobName::Professor
                | JobName::Stalker
                | JobName::Creator
                | JobName::Clown
                | JobName::Gypsy
        )
    }
    pub fn is_novice(&self) -> bool {
        matches!(
            self,
            JobName::NoviceHigh | JobName::Novice | JobName::BabyNovice
        )
    }
    pub fn is_supernovice(&self) -> bool {
        matches!(self, JobName::SuperNovice | JobName::SuperBaby)
    }
    pub fn is_first_class(&self) -> bool {
        matches!(
            self,
            JobName::Swordsman
                | JobName::Mage
                | JobName::Archer
                | JobName::Acolyte
                | JobName::Merchant
                | JobName::Thief
                | JobName::BabySwordsman
                | JobName::BabyMage
                | JobName::BabyArcher
                | JobName::BabyAcolyte
                | JobName::BabyMerchant
                | JobName::BabyThief
                | JobName::SwordsmanHigh
                | JobName::MageHigh
                | JobName::ArcherHigh
                | JobName::AcolyteHigh
                | JobName::MerchantHigh
                | JobName::ThiefHigh
        )
    }
    pub fn is_second_class(&self) -> bool {
        matches!(
            self,
            JobName::Knight
                | JobName::Priest
                | JobName::Wizard
                | JobName::Blacksmith
                | JobName::Hunter
                | JobName::Assassin
                | JobName::Crusader
                | JobName::Monk
                | JobName::Sage
                | JobName::Rogue
                | JobName::Alchemist
                | JobName::Bard
                | JobName::Dancer
                | JobName::BabyKnight
                | JobName::BabyPriest
                | JobName::BabyWizard
                | JobName::BabyBlacksmith
                | JobName::BabyHunter
                | JobName::BabyAssassin
                | JobName::BabyCrusader
                | JobName::BabyMonk
                | JobName::BabySage
                | JobName::BabyRogue
                | JobName::BabyAlchemist
                | JobName::BabyBard
                | JobName::BabyDancer
                | JobName::LordKnight
                | JobName::HighPriest
                | JobName::HighWizard
                | JobName::Whitesmith
                | JobName::Sniper
                | JobName::AssassinCross
                | JobName::Paladin
                | JobName::Champion
                | JobName::Professor
                | JobName::Stalker
                | JobName::Creator
                | JobName::Clown
                | JobName::Gypsy
        )
    }
    pub fn is_gunslinger_ninja(&self) -> bool {
        matches!(self, JobName::Gunslinger | JobName::Ninja)
    }
    pub fn is_taekwon(&self) -> bool {
        matches!(
            self,
            JobName::Taekwon | JobName::StarGladiator | JobName::SoulLinker
        )
    }

    pub fn mask(&self) -> u64 {
        let base_mask = match self {
            // 1-1
            JobName::Novice | JobName::SuperNovice | JobName::NoviceHigh => 1,
            JobName::Swordsman
            | JobName::SwordsmanHigh
            | JobName::Knight
            | JobName::Crusader
            | JobName::BabyKnight
            | JobName::BabyCrusader
            | JobName::LordKnight
            | JobName::Paladin => 2,
            JobName::Mage
            | JobName::MageHigh
            | JobName::Wizard
            | JobName::Sage
            | JobName::BabyWizard
            | JobName::BabySage
            | JobName::HighWizard
            | JobName::Professor => 4,
            JobName::Archer
            | JobName::ArcherHigh
            | JobName::Hunter
            | JobName::Bard
            | JobName::Dancer
            | JobName::BabyHunter
            | JobName::BabyBard
            | JobName::BabyDancer
            | JobName::Sniper
            | JobName::Clown
            | JobName::Gypsy => 8,
            JobName::Acolyte
            | JobName::AcolyteHigh
            | JobName::Priest
            | JobName::Monk
            | JobName::BabyPriest
            | JobName::BabyMonk
            | JobName::HighPriest
            | JobName::Champion => 16,
            JobName::Merchant
            | JobName::MerchantHigh
            | JobName::Blacksmith
            | JobName::Alchemist
            | JobName::BabyBlacksmith
            | JobName::BabyAlchemist
            | JobName::Creator
            | JobName::Whitesmith => 32,
            JobName::Thief
            | JobName::ThiefHigh
            | JobName::Assassin
            | JobName::Rogue
            | JobName::BabyAssassin
            | JobName::BabyRogue
            | JobName::AssassinCross
            | JobName::Stalker => 64,
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
            | JobName::BabySwordsman
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
            | JobName::BabyBard
            | JobName::BabyDancer
            | JobName::BabyMonk
            | JobName::BabyAlchemist
            | JobName::BabyRogue => JOB_BABY_MASK | JOB_2_2_MASK,

            JobName::Wedding => JOB_OTHER | 1,
            JobName::Gunslinger => JOB_OTHER | 2,
            JobName::Ninja => JOB_OTHER | 4,
            JobName::Xmas => JOB_OTHER | 8,
            JobName::Summer => JOB_OTHER | 16,
            JobName::SuperBaby => JOB_OTHER | 32,
            _ => 0,
        };

        base_mask | class_mask
    }

    pub fn from_mask(mask: u64, is_male: bool) -> Option<Self> {
        if mask == 1 {
            return Some(JobName::Novice);
        }
        if mask == 2 {
            return Some(JobName::Swordsman);
        }
        if mask == 4 {
            return Some(JobName::Mage);
        }
        if mask == 8 {
            return Some(JobName::Archer);
        }
        if mask == 16 {
            return Some(JobName::Acolyte);
        }
        if mask == 32 {
            return Some(JobName::Merchant);
        }
        if mask == 64 {
            return Some(JobName::Thief);
        }
        if mask == 128 {
            return Some(JobName::Taekwon);
        }

        if mask == JOB_2_1_MASK | 1 {
            return Some(JobName::SuperNovice);
        }
        if mask == JOB_2_1_MASK | 2 {
            return Some(JobName::Knight);
        }
        if mask == JOB_2_1_MASK | 4 {
            return Some(JobName::Wizard);
        }
        if mask == JOB_2_1_MASK | 8 {
            return Some(JobName::Hunter);
        }
        if mask == JOB_2_1_MASK | 16 {
            return Some(JobName::Priest);
        }
        if mask == JOB_2_1_MASK | 32 {
            return Some(JobName::Blacksmith);
        }
        if mask == JOB_2_1_MASK | 64 {
            return Some(JobName::Assassin);
        }
        if mask == JOB_2_1_MASK | 128 {
            return Some(JobName::StarGladiator);
        }

        if mask == JOB_2_2_MASK | 2 {
            return Some(JobName::Crusader);
        }
        if mask == JOB_2_2_MASK | 4 {
            return Some(JobName::Sage);
        }
        if mask == JOB_2_2_MASK | 8 {
            return if is_male {
                Some(JobName::Bard)
            } else {
                Some(JobName::Dancer)
            };
        }
        if mask == JOB_2_2_MASK | 16 {
            return Some(JobName::Monk);
        }
        if mask == JOB_2_2_MASK | 32 {
            return Some(JobName::Alchemist);
        }
        if mask == JOB_2_2_MASK | 64 {
            return Some(JobName::Rogue);
        }
        if mask == JOB_2_2_MASK | 128 {
            return Some(JobName::SoulLinker);
        }

        if mask == JOB_TRANS_MASK | 1 {
            return Some(JobName::NoviceHigh);
        }
        if mask == JOB_TRANS_MASK | 2 {
            return Some(JobName::SwordsmanHigh);
        }
        if mask == JOB_TRANS_MASK | 4 {
            return Some(JobName::MageHigh);
        }
        if mask == JOB_TRANS_MASK | 8 {
            return Some(JobName::ArcherHigh);
        }
        if mask == JOB_TRANS_MASK | 16 {
            return Some(JobName::AcolyteHigh);
        }
        if mask == JOB_TRANS_MASK | 32 {
            return Some(JobName::MerchantHigh);
        }
        if mask == JOB_TRANS_MASK | 64 {
            return Some(JobName::ThiefHigh);
        }

        if mask == JOB_TRANS_2_1_MASK | 2 {
            return Some(JobName::LordKnight);
        }
        if mask == JOB_TRANS_2_1_MASK | 4 {
            return Some(JobName::HighWizard);
        }
        if mask == JOB_TRANS_2_1_MASK | 8 {
            return Some(JobName::Sniper);
        }
        if mask == JOB_TRANS_2_1_MASK | 16 {
            return Some(JobName::HighPriest);
        }
        if mask == JOB_TRANS_2_1_MASK | 32 {
            return Some(JobName::Whitesmith);
        }
        if mask == JOB_TRANS_2_1_MASK | 64 {
            return Some(JobName::AssassinCross);
        }

        if mask == JOB_TRANS_2_2_MASK | 2 {
            return Some(JobName::Paladin);
        }
        if mask == JOB_TRANS_2_2_MASK | 4 {
            return Some(JobName::Professor);
        }
        if mask == JOB_TRANS_2_2_MASK | 8 {
            return if is_male {
                Some(JobName::Clown)
            } else {
                Some(JobName::Gypsy)
            };
        }
        if mask == JOB_TRANS_2_2_MASK | 16 {
            return Some(JobName::Champion);
        }
        if mask == JOB_TRANS_2_2_MASK | 32 {
            return Some(JobName::Creator);
        }
        if mask == JOB_TRANS_2_2_MASK | 64 {
            return Some(JobName::Stalker);
        }

        if mask == JOB_BABY_MASK | 1 {
            return Some(JobName::BabyNovice);
        }
        if mask == JOB_BABY_MASK | 2 {
            return Some(JobName::BabySwordsman);
        }
        if mask == JOB_BABY_MASK | 4 {
            return Some(JobName::BabyMage);
        }
        if mask == JOB_BABY_MASK | 8 {
            return Some(JobName::BabyArcher);
        }
        if mask == JOB_BABY_MASK | 16 {
            return Some(JobName::BabyAcolyte);
        }
        if mask == JOB_BABY_MASK | 32 {
            return Some(JobName::BabyMerchant);
        }
        if mask == JOB_BABY_MASK | 64 {
            return Some(JobName::BabyThief);
        }

        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 2 {
            return Some(JobName::BabyKnight);
        }
        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 4 {
            return Some(JobName::BabyWizard);
        }
        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 8 {
            return Some(JobName::BabyHunter);
        }
        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 16 {
            return Some(JobName::BabyPriest);
        }
        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 32 {
            return Some(JobName::BabyBlacksmith);
        }
        if mask == JOB_BABY_MASK | JOB_2_1_MASK | 64 {
            return Some(JobName::BabyAssassin);
        }

        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 2 {
            return Some(JobName::BabyCrusader);
        }
        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 4 {
            return Some(JobName::BabySage);
        }
        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 8 {
            return if is_male {
                Some(JobName::BabyBard)
            } else {
                Some(JobName::BabyDancer)
            };
        }
        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 16 {
            return Some(JobName::BabyMonk);
        }
        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 32 {
            return Some(JobName::BabyAlchemist);
        }
        if mask == JOB_BABY_MASK | JOB_2_2_MASK | 64 {
            return Some(JobName::BabyRogue);
        }

        if mask == JOB_OTHER | 1 {
            return Some(JobName::Wedding);
        }
        if mask == JOB_OTHER | 2 {
            return Some(JobName::Gunslinger);
        }
        if mask == JOB_OTHER | 4 {
            return Some(JobName::Ninja);
        }
        if mask == JOB_OTHER | 8 {
            return Some(JobName::Xmas);
        }
        if mask == JOB_OTHER | 16 {
            return Some(JobName::Summer);
        }
        if mask == JOB_OTHER | 32 {
            return Some(JobName::SuperBaby);
        }
        None
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
            JobName::Assassin => {
                EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag()
            }
            JobName::Rogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Archer => EquipClassFlag::Archer.as_flag(),
            JobName::Hunter => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Bard => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Dancer => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Swordsman => EquipClassFlag::Swordsman.as_flag(),
            JobName::Crusader => {
                EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::Knight => {
                EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::Merchant => EquipClassFlag::Merchant.as_flag(),
            JobName::Alchemist => {
                EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
            JobName::Blacksmith => {
                EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
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
            JobName::HighPriest => {
                EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag()
            }
            JobName::HighWizard => {
                EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag()
            }
            JobName::Whitesmith => {
                EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
            JobName::Sniper => EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::AssassinCross => {
                EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag()
            }
            JobName::Champion => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::Professor => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::Stalker => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::Creator => {
                EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
            JobName::Clown => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Gypsy => EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::Paladin => {
                EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::LordKnight => {
                EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::BabyNovice => EquipClassFlag::Novice.as_flag(),
            JobName::BabySwordsman => EquipClassFlag::Swordsman.as_flag(),
            JobName::BabyMage => EquipClassFlag::Mage.as_flag(),
            JobName::BabyArcher => EquipClassFlag::Archer.as_flag(),
            JobName::BabyAcolyte => EquipClassFlag::Acolyte.as_flag(),
            JobName::BabyMerchant => EquipClassFlag::Merchant.as_flag(),
            JobName::BabyThief => EquipClassFlag::Thief.as_flag(),
            JobName::BabyKnight => {
                EquipClassFlag::Knight.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::BabyPriest => {
                EquipClassFlag::Priest.as_flag() | EquipClassFlag::Acolyte.as_flag()
            }
            JobName::BabyWizard => {
                EquipClassFlag::Wizard.as_flag() | EquipClassFlag::Mage.as_flag()
            }
            JobName::BabyBlacksmith => {
                EquipClassFlag::Blacksmith.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
            JobName::BabyHunter => {
                EquipClassFlag::Hunter.as_flag() | EquipClassFlag::Archer.as_flag()
            }
            JobName::BabyAssassin => {
                EquipClassFlag::Assassin.as_flag() | EquipClassFlag::Thief.as_flag()
            }
            JobName::BabyCrusader => {
                EquipClassFlag::Crusader.as_flag() | EquipClassFlag::Swordsman.as_flag()
            }
            JobName::BabyMonk => EquipClassFlag::Monk.as_flag() | EquipClassFlag::Acolyte.as_flag(),
            JobName::BabySage => EquipClassFlag::Sage.as_flag() | EquipClassFlag::Mage.as_flag(),
            JobName::BabyRogue => EquipClassFlag::Rogue.as_flag() | EquipClassFlag::Thief.as_flag(),
            JobName::BabyAlchemist => {
                EquipClassFlag::Alchemist.as_flag() | EquipClassFlag::Merchant.as_flag()
            }
            JobName::SuperBaby => 0,
            JobName::BabyBard => EquipClassFlag::Bard.as_flag() | EquipClassFlag::Archer.as_flag(),
            JobName::BabyDancer => {
                EquipClassFlag::Dancer.as_flag() | EquipClassFlag::Archer.as_flag()
            }
        }
    }
}
