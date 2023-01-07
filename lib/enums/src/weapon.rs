#![allow(dead_code)]

use sqlx::{Decode, Postgres};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::TypeInfo;
use crate::*;

#[derive(WithNumberValue, WithMaskValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
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
    Shuriken,
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

impl WeaponType {
    pub fn is_ranged(&self) -> bool {
        matches!(self, Self::Bow | Self::Rifle | Self::Shotgun | Self::Gatling | Self::Revolver | Self::Grenade)
    }
}

#[derive(WithNumberValue, WithMaskValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
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

impl<'r> Decode<'r, Postgres> for WeaponType {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        Ok(WeaponType::from_string_ignore_case(value))
    }
}

impl sqlx::Type<Postgres> for WeaponType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <&str as sqlx::Type<sqlx::Postgres>>::type_info()
    }


    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "VARCHAR"
    }
}