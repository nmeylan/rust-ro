use std::sync::Mutex;
use rand::RngCore;
use models::enums::class::JobName;
use models::enums::EnumWithMaskValueU64;
use models::enums::item::{EquipmentLocation};


use models::status::{Look, Status};
use crate::repository::model::item_model::{DBItemType, InventoryItemModel, ItemModel};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use models::enums::EnumWithNumberValue;
use models::status_bonus::{StatusBonuses, TemporaryStatusBonuses};

pub fn create_character() -> Character {
    Character {
        name: "Walkiry".to_string(),
        status: Status {
            job: JobName::Novice.value() as u32,
            hp: 0,
            sp: 0,
            str: 1,
            agi: 1,
            vit: 1,
            int: 1,
            dex: 1,
            luk: 1,
            speed: 0,
            look: Some(Look::default()),
            zeny: 0,
            base_level: 1,
            job_level: 1,
            status_point: 48,
            skill_point: 0,
            base_exp: 0,
            job_exp: 0,
            state: 0,
            size: Default::default(),
            is_male: true,
            weapons: vec![],
            equipments: vec![],
            ammo: None,
            known_skills: vec![],
            effects: vec![],
            equipment_bonuses: StatusBonuses::default(),
            temporary_bonuses: TemporaryStatusBonuses::default(),
        },
        char_id: 150000,
        account_id: 2000000,
        map_instance_key: MapInstanceKey::new("Prontera".to_string(), 0),
        loaded_from_client_side: false,
        x: 156,
        y: 179,
        dir: 0,
        movements: vec![],
        attack: None,
        skill_in_use: None,
        inventory: vec![],
        map_view: Default::default(),
        script_variable_store: Mutex::new(Default::default()),
        last_moved_at: 0,
    }
}


pub fn equip_item_from_name(character: &mut Character, aegis_name: &str) -> usize {
    let item = GlobalConfigService::instance().get_item_by_name(aegis_name);
    equip_item(character, item)
}
pub fn equip_item_from_id(character: &mut Character, id: u32) -> usize {
    let item = GlobalConfigService::instance().get_item(id as i32);
    equip_item(character, item)
}
pub fn equip_item_from_id_with_cards(character: &mut Character, id: u32, cards: Vec<i16>) -> usize {
    let item = GlobalConfigService::instance().get_item(id as i32);
    equip_item_with_cards_and_refinement(character, item, cards, 0)
}
pub fn equip_item_from_name_with_cards(character: &mut Character, name: &str, cards: Vec<i16>) -> usize {
    let item = GlobalConfigService::instance().get_item_by_name(name);
    equip_item_with_cards_and_refinement(character, item, cards, 0)
}

//
// Warning: this method is not safe, if an item is already equipped at the given item location, character will have more than 1 item equipped to this location.
// Character equip given item.
pub fn equip_item(character: &mut Character, item: &ItemModel) -> usize {
    equip_item_with_cards_and_refinement(character, item, vec![], 0)
}//
// Warning: this method is not safe, if an item is already equipped at the given item location, character will have more than 1 item equipped to this location.
// Character equip given item.
pub fn equip_item_with_cards_and_refinement(character: &mut Character, item: &ItemModel, cards: Vec<i16>, refinement: u8) -> usize {
    let mut rng = rand::thread_rng();
    let inventory_item = InventoryItemModel {
        id: rng.next_u32() as i32,
        unique_id: rng.next_u32() as i64,
        item_id: item.id,
        item_type: DBItemType::from_type(item.item_type),
        amount: 1,
        refine: refinement as i16,
        is_identified: false,
        equip: item.location as i32,
        is_damaged: false,
        card0: cards.first().cloned().unwrap_or_default(),
        card1: cards.get(1).cloned().unwrap_or_default(),
        card2: cards.get(2).cloned().unwrap_or_default(),
        card3: cards.get(3).cloned().unwrap_or_default(),
        name_english: item.name_english.clone(),
        weight: item.weight,
    };
    let index = character.add_in_inventory(inventory_item);
    character.wear_equip_item(index, item.location, item);
    index
}

pub fn takeoff_weapon(character: &mut Character) {
    character.takeoff_equip_item(EquipmentLocation::HandRight.as_flag() as usize);
    character.takeoff_equip_item(EquipmentLocation::HandLeft.as_flag() as usize);
}

pub fn add_item_in_inventory(character: &mut Character, aegis_name: &str) -> usize {
    let mut rng = rand::thread_rng();
    let item = GlobalConfigService::instance().get_item_by_name(aegis_name);
    let inventory_item = InventoryItemModel {
        id: rng.next_u32() as i32,
        unique_id: rng.next_u32() as i64,
        item_id: item.id,
        item_type: DBItemType::from_type(item.item_type),
        amount: 1,
        refine: 0,
        is_identified: false,
        equip: 0,
        is_damaged: false,
        card0: 0,
        card1: 0,
        card2: 0,
        card3: 0,
        name_english: item.name_english.clone(),
        weight: item.weight,
    };
    character.add_in_inventory(inventory_item)
}

pub fn add_items_in_inventory(character: &mut Character, aegis_name: &str, amount: i16) -> usize {
    let mut rng = rand::thread_rng();
    let item = GlobalConfigService::instance().get_item_by_name(aegis_name);
    let inventory_item = InventoryItemModel {
        id: rng.next_u32() as i32,
        unique_id: rng.next_u32() as i64,
        item_id: item.id,
        item_type: DBItemType::from_type(item.item_type),
        amount,
        refine: 0,
        is_identified: false,
        equip: 0,
        is_damaged: false,
        card0: 0,
        card1: 0,
        card2: 0,
        card3: 0,
        name_english: item.name_english.clone(),
        weight: item.weight,
    };
    character.add_in_inventory(inventory_item)
}