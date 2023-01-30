#![allow(dead_code)]
use crate::*;

#[derive(WithMaskValueU64, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapPropertyFlags {
    // PARTY - Show attack cursor on non-party members (PvP)
    IsParty,
    // GUILD - Show attack cursor on non-guild members (GvG)
    IsGuild,
    // SIEGE - Show emblem over characters heads when in GvG (WoE castle)
    IsSiege,
    // USE_SIMPLE_EFFECT - Automatically enable /mineffect
    UseSimpleEffect,
    // DISABLE_LOCKON - Only allow attacks on other players with shift key or /ns active
    IsNoLockOn,
    // COUNT_PK - Show the PvP counter
    CountPk,
    // NO_PARTY_FORMATION - Prevents party creation/modification (Might be used for instance dungeons)
    PartyLock,
    // BATTLEFIELD - Unknown (Does something for battlegrounds areas)
    IsBattleground,
    // DISABLE_COSTUMEITEM - Disable costume sprites
    IsNoCostum,
    // USECART - Allow opening cart inventory (Well force it to always allow it)
    IsUseCart,
    // SUNMOONSTAR_MIRACLE - Blocks Star Gladiator's Miracle from activating
    IsSummonstarMiracle,
    // Unused bits. 1 - 10 is 0x1 length and 11 is 0x15 length. May be used for future settings.
    Unused
}