use enums::weapon::WeaponType;

pub struct Weapon {
    pub attack: u32,
    pub level: u8,
    pub weapon_type: WeaponType,
    pub refine: i16,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
}