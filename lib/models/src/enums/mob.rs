use enum_macro::{WithNumberValue, WithStringValue};
use crate::enums::{EnumWithNumberValue, EnumWithStringValue};

#[derive(WithStringValue, WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MobClass {
    #[value = 0]
    #[value_string = "Class_Normal"]
    Normal,
    #[value_string = "Class_Boss"]
    Boss,
    #[value_string = "Class_Guardian"]
    Guardian,
    #[value_string = "Class_All"]
    All,
}

#[derive(WithStringValue, WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MobRace {
    #[value_string = "RC_All"]
    #[value_string = "All"]
    #[value = 0]
    All,
    #[value_string = "RC_Angel"]
    #[value_string = "Angel"]
    Angel,
    #[value_string = "RC_Brute"]
    #[value_string = "Brute"]
    Brute,
    #[value_string = "RC_DemiHuman"]
    #[value_string = "DemiHuman"]
    #[value_string = "Demihuman"]
    DemiHuman,
    #[value_string = "RC_Demon"]
    #[value_string = "Demon"]
    Demon,
    #[value_string = "RC_Dragon"]
    #[value_string = "Dragon"]
    Dragon,
    #[value_string = "RC_Fish"]
    #[value_string = "Fish"]
    Fish,
    #[value_string = "RC_Formless"]
    #[value_string = "Formless"]
    Formless,
    #[value_string = "RC_Insect"]
    #[value_string = "Insect"]
    Insect,
    #[value_string = "RC_Plant"]
    #[value_string = "Plant"]
    Plant,
    #[value_string = "RC_Player_Human"]
    PlayerHuman,
    #[value_string = "RC_Player_Doram"]
    PlayerDoram,
    #[value_string = "RC_Undead"]
    #[value_string = "Undead"]
    RUndead,

}

#[derive(WithStringValue, WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MobGroup {
    #[value_string = "RC2_Goblin"]
    #[value = 0]
    Goblin,
    #[value_string = "RC2_Kobold"]
    Kobold,
    #[value_string = "RC2_Golem"]
    Golem,
    #[value_string = "RC2_Orc"]
    Orc,
    #[value_string = "RC2_Guardian"]
    Guardian,
    #[value_string = "RC2_Ninja"]
    Ninja,
    #[value_string = "RC2_GVG"]
    GVG,
    #[value_string = "RC2_Battlefield"]
    Battlefield,
    #[value_string = "RC2_Treasure"]
    Treasure,
    #[value_string = "RC2_BioLab"]
    BioLab,
    #[value_string = "RC2_Manuk"]
    Manuk,
    #[value_string = "RC2_Splendide"]
    Splendide,
    #[value_string = "RC2_Scaraba"]
    Scaraba,
    #[value_string = "RC2_Clocktower"]
    Clocktower,
    #[value_string = "RC2_Thanatos"]
    Thanatos,
    #[value_string = "RC2_Faceworm"]
    Faceworm,
    #[value_string = "RC2_Hearthunter"]
    Hearthunter,
    #[value_string = "RC2_Rockridge"]
    Rockridge,
    #[value_string = "RC2_Werner_Lab"]
    WernerLab,
    #[value_string = "RC2_Temple_Demon"]
    TempleDemon,
    #[value_string = "RC2_Illusion_Vampire"]
    IllusionVampire,
    #[value_string = "RC2_Malangdo"]
    Malangdo,
    #[value_string = "RC2_Rachel_Sanctuary"]
    RachelSanctuary,

}