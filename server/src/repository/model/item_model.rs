
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use crate::util::serde_helper::{*};


use sqlx::postgres::PgRow;

use enums::class::EquipClassFlag;
use enums::{EnumWithMaskValueU64, EnumWithStringValue};
use enums::item::{EquipmentLocation, ItemClass, ItemFlag, ItemTradeFlag, ItemType};
use enums::weapon::{AmmoType, WeaponType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModels {
    items: Vec<ItemModel>
}
impl From<Vec<ItemModel>> for ItemModels {
    fn from(items: Vec<ItemModel>) -> Self {
        ItemModels {
            items
        }
    }
}
impl From<ItemModels> for Vec<ItemModel> {
    fn from(item_models: ItemModels) -> Self {
        item_models.items
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemModel {
    pub id: i32,
    pub name_aegis: String,
    pub name_english: String,
    #[serde(serialize_with = "serialize_string_enum", deserialize_with = "deserialize_string_enum")]
    pub item_type: ItemType,
    #[serde(serialize_with = "serialize_optional_string_enum", deserialize_with = "deserialize_optional_string_enum", default)]
    pub weapon_type: Option<WeaponType>,
    #[serde(serialize_with = "serialize_optional_string_enum", deserialize_with = "deserialize_optional_string_enum", default)]
    pub ammo_type: Option<AmmoType>,
    pub price_buy: Option<i32>,
    pub price_sell: Option<i32>,
    pub weight: i32,
    pub attack: Option<i16>,
    pub defense: Option<i16>,
    pub range: Option<i16>,
    pub slots: Option<i16>,
    pub job_flags: u64,
    pub class_flags: u64,
    pub location: u64,
    pub gender: Option<String>,
    pub weapon_level: Option<i16>,
    pub armor_level: Option<i16>,
    pub equip_level_min: Option<i16>,
    pub equip_level_max: Option<i16>,
    pub refineable: Option<i16>,
    pub view: Option<i32>,
    pub alias_name: Option<String>,
    pub flags: u64,
    pub delay_duration: Option<i32>,
    pub delay_status: Option<String>,
    pub stack_amount: Option<i32>,
    pub stack_inventory: Option<i16>,
    pub stack_cart: Option<i16>,
    pub stack_storage: Option<i16>,
    pub stack_guildstorage: Option<i16>,
    pub nouse_override: Option<i32>,
    pub nouse_sitting: Option<i16>,
    pub trade_override: Option<i32>,
    pub trade_flags: u64,
}

impl<'r> FromRow<'r, PgRow> for ItemModel {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id: i32 = row.get("id");
        let name_aegis: String = row.get("name_aegis");
        let name_english: String = row.get("name_english");
        let item_type = ItemType::from_string(row.get("type"));
        let mut weapon_type: Option<WeaponType> = None;
        if item_type == ItemType::Weapon {
            weapon_type = row.try_get("subtype").or_else(Self::map_error::<Option<String>>())?.map(|s| WeaponType::from_string_ignore_case(s.as_str()));
        }

        let mut ammo_type: Option<AmmoType> = None;
        if item_type == ItemType::Ammo {
            ammo_type = row.try_get("subtype").or_else(Self::map_error::<Option<String>>())?.map(|s| AmmoType::from_string_ignore_case(s.as_str()));
        }

        let price_buy: Option<i32> = row.try_get("price_buy").or_else(Self::map_error())?;
        let price_sell: Option<i32> = row.try_get("price_sell").or_else(Self::map_error())?;
        let weight: i32 = row.try_get::<Option<i32>, _>("weight").map(|v: Option<i32>| v.unwrap_or(0)).or_else(Self::map_error())?;
        let attack: Option<i16> = row.try_get("attack").or_else(Self::map_error())?;
        let defense: Option<i16> = row.try_get("defense").or_else(Self::map_error())?;
        let range: Option<i16> = row.try_get("range").or_else(Self::map_error())?;
        let slots: Option<i16> = row.try_get("slots").or_else(Self::map_error())?;
        let mut job_flags = vec![];
        row.try_get::<'r, Option<i16>, _>("job_all").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::All) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_acolyte").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Acolyte) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_alchemist").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Alchemist) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_archer").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Archer) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_assassin").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Assassin) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_barddancer").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Barddancer) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_blacksmith").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Blacksmith) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_crusader").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Crusader) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_hunter").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Hunter) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_gunslinger").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Gunslinger) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_knight").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Knight) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_mage").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Mage) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_merchant").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Merchant) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_monk").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Monk) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_ninja").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Ninja) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_novice").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Novice) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_priest").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Priest) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_rogue").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Rogue) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_sage").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Sage) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_soullinker").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Soullinker) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_stargladiator").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Stargladiator) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_supernovice").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Supernovice) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_swordman").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Swordman) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_taekwon").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Taekwon) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_thief").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Thief) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("job_wizard").map(|v| if v.is_some() && v.unwrap() != 0 { job_flags.push(EquipClassFlag::Wizard) }).or_else(Self::map_error())?;
        let job_flags = Self::enum_flags_into_u64(&job_flags);
        let mut class_flags = vec![];
        row.try_get::<'r, Option<i16>, _>("class_all").map(|v| if v.is_some() && v.unwrap() != 0 { class_flags.push(ItemClass::All) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("class_normal").map(|v| if v.is_some() && v.unwrap() != 0 { class_flags.push(ItemClass::Normal) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("class_upper").map(|v| if v.is_some() && v.unwrap() != 0 { class_flags.push(ItemClass::Upper) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("class_baby").map(|v| if v.is_some() && v.unwrap() != 0 { class_flags.push(ItemClass::Baby) }).or_else(Self::map_error())?;
        let class_flags = Self::enum_flags_into_u64(&class_flags);
        let gender: Option<String> = row.try_get("gender").or_else(Self::map_error())?;

        let mut location_flags = vec![];
        row.try_get::<'r, Option<i16>, _>("location_head_top").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::HeadTop) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_head_mid").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::HeadMid) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_head_low").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::HeadLow) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_armor").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::Armor) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_right_hand").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::HandRight) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_left_hand").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::HandLeft) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_garment").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::Garment) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shoes").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::Shoes) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_right_accessory").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::AccessoryRight) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_left_accessory").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::AccessoryLeft) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_costume_head_top").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::CostumeHeadTop) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_costume_head_mid").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::CostumeHeadMid) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_costume_head_low").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::CostumeHeadLow) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_costume_garment").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::CostumeGarment) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_ammo").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::Ammo) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shadow_weapon").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::ShadowWeapon) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shadow_shield").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::ShadowShield) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shadow_shoes").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::ShadowShoes) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shadow_right_accessory").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::ShadowAccR) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("location_shadow_left_accessory").map(|v| if v.is_some() && v.unwrap() != 0 { location_flags.push(EquipmentLocation::ShadowAccL) }).or_else(Self::map_error())?;
        let location_flags = Self::enum_flags_into_u64(&location_flags);

        let weapon_level: Option<i16> = row.try_get("weapon_level").or_else(Self::map_error())?;
        let armor_level: Option<i16> = row.try_get("armor_level").or_else(Self::map_error())?;
        let equip_level_min: Option<i16> = row.try_get("equip_level_min").or_else(Self::map_error())?;
        let equip_level_max: Option<i16> = row.try_get("equip_level_max").or_else(Self::map_error())?;
        let refineable: Option<i16> = row.try_get("refineable").or_else(Self::map_error())?;
        let view: Option<i32> = row.try_get("view").or_else(Self::map_error())?;
        let alias_name: Option<String> = row.try_get("alias_name").or_else(Self::map_error())?;

        let mut flags = vec![];
        row.try_get::<'r, Option<i16>, _>("flag_buyingstore").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::BuyingStore) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_deadbranch").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::DeadBranch) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_container").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::Container) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_uniqueid").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::UniqueId) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_bindonequip").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::BindOnEquip) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_dropannounce").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::DropAnnounce) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_noconsume").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::NoConsume) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("flag_dropeffect").map(|v| if v.is_some() && v.unwrap() != 0 { flags.push(ItemFlag::DropEffect) }).or_else(Self::map_error())?;
        let flags = Self::enum_flags_into_u64(&flags);

        let delay_duration: Option<i32> = row.try_get("delay_duration").or_else(Self::map_error())?;
        let delay_status: Option<String> = row.try_get("delay_status").or_else(Self::map_error())?;
        let stack_amount: Option<i32> = row.try_get("stack_amount").or_else(Self::map_error())?;
        let stack_inventory: Option<i16> = row.try_get("stack_inventory").or_else(Self::map_error())?;
        let stack_cart: Option<i16> = row.try_get("stack_cart").or_else(Self::map_error())?;
        let stack_storage: Option<i16> = row.try_get("stack_storage").or_else(Self::map_error())?;
        let stack_guildstorage: Option<i16> = row.try_get("stack_guildstorage").or_else(Self::map_error())?;
        let nouse_override: Option<i32> = row.try_get("nouse_override").or_else(Self::map_error())?;
        let nouse_sitting: Option<i16> = row.try_get("nouse_sitting").or_else(Self::map_error())?;
        let trade_override: Option<i32> = row.try_get("trade_override").or_else(Self::map_error())?;

        let mut trade_flags = vec![];
        row.try_get::<'r, Option<i16>, _>("trade_nodrop").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoDrop) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_notrade").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoTrade) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_tradepartner").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::TradePartner) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_nosell").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoSell) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_nocart").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoCart) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_nostorage").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoStorage) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_noguildstorage").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoGuildStorage) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_noauction").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoAuction) }).or_else(Self::map_error())?;
        row.try_get::<'r, Option<i16>, _>("trade_nomail").map(|v| if v.is_some() && v.unwrap() != 0 { trade_flags.push(ItemTradeFlag::NoMail) }).or_else(Self::map_error())?;
        let trade_flags = Self::enum_flags_into_u64(&trade_flags);

        Ok(ItemModel {
            id,
            name_aegis,
            name_english,
            item_type,
            weapon_type,
            ammo_type,
            price_buy,
            price_sell,
            weight,
            attack,
            defense,
            range,
            slots,
            job_flags,
            gender,
            location: location_flags,
            weapon_level,
            armor_level,
            equip_level_min,
            equip_level_max,
            refineable,
            view,
            alias_name,
            flags,
            delay_duration,
            delay_status,
            stack_amount,
            stack_inventory,
            stack_cart,
            stack_storage,
            stack_guildstorage,
            nouse_override,
            nouse_sitting,
            trade_override,
            trade_flags,
            class_flags,
        })
    }
}

impl ItemModel {
    fn map_error<T: Default>() -> fn(Error) -> Result<T, Error> {
        |e| match e {
            ::sqlx::Error::ColumnNotFound(_) => ::std::result::Result::Ok(Default::default()),
            e => ::std::result::Result::Err(e),
        }
    }

    fn enum_flags_into_u64<T: EnumWithMaskValueU64>(flags: &Vec<T>) -> u64 {
        flags.iter().fold(0, |acc, enum_flag| {
            acc | enum_flag.as_flag() as u64
        })
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct ItemBuySellModel {
    pub id: Option<i32>,
    #[sqlx(rename = "type")]
    pub item_type: String,
    #[sqlx(default)]
    pub price_buy: Option<i32>,
    #[sqlx(default)]
    pub price_sell: Option<i32>,
    #[sqlx(default)]
    pub stack_amount: Option<i16>,
    #[sqlx(default)]
    pub weight: Option<i32>,
    #[sqlx(default)]
    pub name_english: Option<String>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct GetItemModel {
    pub id: i32,
    #[sqlx(rename = "type")]
    pub item_type: String,
    #[sqlx(default)]
    pub amount: i16,
    pub weight: i32,
    #[sqlx(default)]
    pub name_english: String,
    #[sqlx(default)]
    pub name_aegis: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct InventoryItemModel {
    // Come from inventory table
    pub id: i32,
    pub unique_id: i64,
    #[sqlx(rename = "nameid")]
    pub item_id: i32,
    #[sqlx(rename = "type")]
    pub item_type: ItemType,
    pub amount: i16,
    pub refine: i16,
    #[sqlx(rename = "identified")]
    pub is_identified: bool,
    pub equip: i32,
    #[sqlx(rename = "damaged")]
    pub is_damaged: bool,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    // Come from itemdb table
    #[sqlx(default)]
    pub name_english: String,
    pub weight: i32,
}
