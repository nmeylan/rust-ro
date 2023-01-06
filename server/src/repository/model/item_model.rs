use sqlx::{Decode, Postgres};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::TypeInfo;
use enums::item::{EquipmentLocation, ItemType};
use enums::weapon::WeaponType;

#[derive(sqlx::FromRow, Debug)]
pub struct ItemModel {
    #[sqlx(default)]
    pub id: Option<i32>,
    #[sqlx(default)]
    pub name_aegis: Option<String>,
    #[sqlx(default)]
    pub name_english: Option<String>,
    #[sqlx(rename = "type")]
    #[sqlx(default)]
    pub item_type: Option<String>,
    #[sqlx(default)]
    pub subtype: Option<String>,
    #[sqlx(default)]
    pub price_buy: Option<i32>,
    #[sqlx(default)]
    pub price_sell: Option<i32>,
    #[sqlx(default)]
    pub weight: Option<i32>,
    #[sqlx(default)]
    pub attack: Option<i16>,
    #[sqlx(default)]
    pub defense: Option<i16>,
    #[sqlx(default)]
    pub range: Option<i16>,
    #[sqlx(default)]
    pub slots: Option<i16>,
    #[sqlx(default)]
    pub job_all: Option<i16>,
    #[sqlx(default)]
    pub job_acolyte: Option<i16>,
    #[sqlx(default)]
    pub job_alchemist: Option<i16>,
    #[sqlx(default)]
    pub job_archer: Option<i16>,
    #[sqlx(default)]
    pub job_assassin: Option<i16>,
    #[sqlx(default)]
    pub job_barddancer: Option<i16>,
    #[sqlx(default)]
    pub job_blacksmith: Option<i16>,
    #[sqlx(default)]
    pub job_crusader: Option<i16>,
    #[sqlx(default)]
    pub job_gunslinger: Option<i16>,
    #[sqlx(default)]
    pub job_hunter: Option<i16>,
    #[sqlx(default)]
    pub job_knight: Option<i16>,
    #[sqlx(default)]
    pub job_mage: Option<i16>,
    #[sqlx(default)]
    pub job_merchant: Option<i16>,
    #[sqlx(default)]
    pub job_monk: Option<i16>,
    #[sqlx(default)]
    pub job_ninja: Option<i16>,
    #[sqlx(default)]
    pub job_novice: Option<i16>,
    #[sqlx(default)]
    pub job_priest: Option<i16>,
    #[sqlx(default)]
    pub job_rogue: Option<i16>,
    #[sqlx(default)]
    pub job_sage: Option<i16>,
    #[sqlx(default)]
    pub job_soullinker: Option<i16>,
    #[sqlx(default)]
    pub job_stargladiator: Option<i16>,
    #[sqlx(default)]
    pub job_supernovice: Option<i16>,
    #[sqlx(default)]
    pub job_swordman: Option<i16>,
    #[sqlx(default)]
    pub job_taekwon: Option<i16>,
    #[sqlx(default)]
    pub job_thief: Option<i16>,
    #[sqlx(default)]
    pub job_wizard: Option<i16>,
    #[sqlx(default)]
    pub class_all: Option<i16>,
    #[sqlx(default)]
    pub class_normal: Option<i16>,
    #[sqlx(default)]
    pub class_upper: Option<i16>,
    #[sqlx(default)]
    pub class_baby: Option<i16>,
    #[sqlx(default)]
    pub gender: Option<String>,
    #[sqlx(default)]
    pub location_head_top: Option<i16>,
    #[sqlx(default)]
    pub location_head_mid: Option<i16>,
    #[sqlx(default)]
    pub location_head_low: Option<i16>,
    #[sqlx(default)]
    pub location_armor: Option<i16>,
    #[sqlx(default)]
    pub location_right_hand: Option<i16>,
    #[sqlx(default)]
    pub location_left_hand: Option<i16>,
    #[sqlx(default)]
    pub location_garment: Option<i16>,
    #[sqlx(default)]
    pub location_shoes: Option<i16>,
    #[sqlx(default)]
    pub location_right_accessory: Option<i16>,
    #[sqlx(default)]
    pub location_left_accessory: Option<i16>,
    #[sqlx(default)]
    pub location_costume_head_top: Option<i16>,
    #[sqlx(default)]
    pub location_costume_head_mid: Option<i16>,
    #[sqlx(default)]
    pub location_costume_head_low: Option<i16>,
    #[sqlx(default)]
    pub location_costume_garment: Option<i16>,
    #[sqlx(default)]
    pub location_ammo: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_armor: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_weapon: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_shield: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_shoes: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_right_accessory: Option<i16>,
    #[sqlx(default)]
    pub location_shadow_left_accessory: Option<i16>,
    #[sqlx(default)]
    pub weapon_level: Option<i16>,
    #[sqlx(default)]
    pub armor_level: Option<i16>,
    #[sqlx(default)]
    pub equip_level_min: Option<i16>,
    #[sqlx(default)]
    pub equip_level_max: Option<i16>,
    #[sqlx(default)]
    pub refineable: Option<i16>,
    #[sqlx(default)]
    pub view: Option<i32>,
    #[sqlx(default)]
    pub alias_name: Option<String>,
    #[sqlx(default)]
    pub flag_buyingstore: Option<i16>,
    #[sqlx(default)]
    pub flag_deadbranch: Option<i16>,
    #[sqlx(default)]
    pub flag_container: Option<i16>,
    #[sqlx(default)]
    pub flag_uniqueid: Option<i16>,
    #[sqlx(default)]
    pub flag_bindonequip: Option<i16>,
    #[sqlx(default)]
    pub flag_dropannounce: Option<i16>,
    #[sqlx(default)]
    pub flag_noconsume: Option<i16>,
    #[sqlx(default)]
    pub flag_dropeffect: Option<String>,
    #[sqlx(default)]
    pub delay_duration: Option<i64>,
    #[sqlx(default)]
    pub delay_status: Option<String>,
    #[sqlx(default)]
    pub stack_amount: Option<i16>,
    #[sqlx(default)]
    pub stack_inventory: Option<i16>,
    #[sqlx(default)]
    pub stack_cart: Option<i16>,
    #[sqlx(default)]
    pub stack_storage: Option<i16>,
    #[sqlx(default)]
    pub stack_guildstorage: Option<i16>,
    #[sqlx(default)]
    pub nouse_override: Option<i16>,
    #[sqlx(default)]
    pub nouse_sitting: Option<i16>,
    #[sqlx(default)]
    pub trade_override: Option<i32>,
    #[sqlx(default)]
    pub trade_nodrop: Option<i16>,
    #[sqlx(default)]
    pub trade_notrade: Option<i16>,
    #[sqlx(default)]
    pub trade_tradepartner: Option<i16>,
    #[sqlx(default)]
    pub trade_nosell: Option<i16>,
    #[sqlx(default)]
    pub trade_nocart: Option<i16>,
    #[sqlx(default)]
    pub trade_nostorage: Option<i16>,
    #[sqlx(default)]
    pub trade_noguildstorage: Option<i16>,
    #[sqlx(default)]
    pub trade_nomail: Option<i16>,
    #[sqlx(default)]
    pub trade_noauction: Option<i16>,
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
    pub location: i32,
}


pub type ItemId = i32;
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct InventoryItemModel {
    // Come from inventory table
    pub id: i32,
    pub unique_id: i64,
    #[sqlx(rename = "nameid")]
    pub item_id: ItemId,
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
    pub location: i32,
}

pub struct EquippedItem {
    pub item_id: i32,
    pub location: i32,
    pub index: usize,
}