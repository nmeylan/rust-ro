use crate::*;

#[derive(WithNumberValue, WithStringValue, Debug, PartialEq)]
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
    #[value = 4045]
    SuperBaby,
    Taekwon,
    StarGladiator,
    SoulLinker,
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