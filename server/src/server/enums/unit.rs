#![allow(dead_code)]
use r#enum::*;

#[derive(WithMaskValue, WithStringValue, Debug, Copy, Clone, PartialEq)]
pub enum UnitTargetType {
    #[mask_value = 0]
    NoOne,
    #[mask_value = 4096]
    #[value_string = "Self"]
    MySelf,
    #[mask_value = 131072]
    Enemy,
    #[mask_value = 262144]
    Party,
    #[mask_value = 524288]
    Guildally,

    #[mask_value = 1048576]
    Neutral,

    #[mask_value = 2097152]
    // Guildmates, No Guild Allies
    SameGuild,

    #[mask_value = 4128768]
    All,

    #[mask_value = 4194304]
// Except self
    Wos,
    #[mask_value = 2621440]
// SameGuild|GuildAlly
    Guild,
    #[mask_value = 1507328]
// All&~Guild
    NoGuild,
    #[mask_value = 3866624]
// All&~Party
    NoParty,
    #[mask_value = 3997696]
// All&~Enemy
    NoEnemy,
    #[mask_value = 2883584]
// Party|Guild
    Ally,
    #[mask_value = 3997696]
    // NoEnemy
    Friend,
}