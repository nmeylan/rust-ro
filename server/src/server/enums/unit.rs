#![allow(dead_code)]

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum UnitTargetFlag {
    #[value = 0]
    NoOne,
    #[value = 4096]
    // Self
    MySelf,
    #[value = 131072]
    Enemy,
    #[value = 262144]
    Party,
    #[value = 524288]
    Guildally,

    #[value = 1048576]
    Neutral,

    #[value = 2097152]
    // Guildmates, No Guild Allies
    Sameguild,

    #[value = 4128768]
    All,

    #[value = 4194304]
// Except self
    Wos,
    #[value = 2621440]
// SameGuild|GuildAlly
    Guild,
    #[value = 1507328]
// All&~Guild
    NoGuild,
    #[value = 3866624]
// All&~Party
    NoParty,
    #[value = 3997696]
// All&~Enemy
    NoEnemy,
    #[value = 2883584]
// Party|Guild
    Ally,
    #[value = 3997696]
    // NoEnemy
    Friend,
}