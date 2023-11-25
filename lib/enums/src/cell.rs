use crate::*;

#[derive(WithMaskValueU16)]
pub enum CellType {
    Walkable,
    Shootable,
    Water,
    Boot,
    Basilica,
    LandProtector,
    NoVending,
    NoChat,
    Icewall,
    NoIceWall,
    NoSkill,
    Warp,
    Mob,
}
