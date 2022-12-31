#![allow(dead_code)]
use r#enum::*;

#[derive(WithNumberValue, WithMaskValue, WithStringValue, Debug, Copy, Clone, PartialEq)]
pub enum WeaponType {
    Fist,
    Dagger,
    #[value_string = "1hSword"]
    Sword1H,
    #[value_string = "2hSword"]
    Sword2H,
    #[value_string = "1hSpear"]
    Spear1H,
    #[value_string = "2hSpear"]
    Spear2H,
    #[value_string = "1hAxe"]
    Axe1H,
    #[value_string = "2hAxe"]
    Axe2H,
    Mace,
    #[value_string = "2hMace"]
    Mace2H,
    Staff,
    Bow,
    Knuckle,
    Musical,
    Whip,
    Book,
    Katar,
    Revolver,
    Rifle,
    Gatling,
    Shotgun,
    Grenade,
    Huuma,
    #[value_string = "2hStaff"]
    Staff2H,
    MaxWeaponType,
    // Double dagger
    DoubleDd,
    // Double sword
    DoubleSs,
    // Double axe
    DoubleAa,
    // dagger + sword
    DoubleDs,
    // dagger + axe
    DoubleDa,
    // sword + axe
    DoubleSa,
    MaxWeaponTypeAll,
}

#[derive(WithNumberValue, WithMaskValue, WithStringValue, Debug, Copy, Clone, PartialEq)]
pub enum AmmoType {
    None,
    Arrow,
    Dagger,
    Bullet,
    Shell,
    Grenade,
    Shuriken,
    Kunai,
    Cannonball,
    Throwweapon,
    MaxType,
}