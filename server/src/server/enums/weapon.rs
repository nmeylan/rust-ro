#![allow(dead_code)]

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
pub enum WeaponType {
    Fist,
    Dagger,
    Sword1H,
    Sword2H,
    Spear1H,
    Spear2H,
    Axe1H,
    Axe2H,
    Mace,
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

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
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