use std::sync::Mutex;
use rand::RngCore;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::status::Status;

pub fn create_character() -> Character {
    Character {
        name: "Walkiry".to_string(),
        status: Status {
            job: 1,
            hp: 0,
            sp: 0,
            max_hp: 0,
            max_sp: 0,
            str: 1,
            agi: 1,
            vit: 1,
            int: 1,
            dex: 1,
            luk: 1,
            base_atk: 0,
            matk_min: 0,
            matk_max: 0,
            speed: 0,
            hit: 0,
            flee: 0,
            crit: 0,
            def: 0,
            mdef: 0,
            look: None,
            zeny: 0,
            base_level: 0,
            job_level: 0
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
        inventory: vec![],
        map_view: Default::default(),
        script_variable_store: Mutex::new(Default::default())
    }
}


//
// Warning: this method is not safe, if an item is already equipped at the given item location, character will have more than 1 item equipped to this location.
// Character equip given item.
pub fn equip_item(character: &mut Character, aegis_name: &str) -> usize {
    let mut rng = rand::thread_rng();
    let item = GlobalConfigService::instance().get_item_by_name(aegis_name);
    let inventory_item = InventoryItemModel {
        id: rng.next_u32() as i32,
        unique_id: rng.next_u32() as i64,
        item_id: item.id,
        item_type: item.item_type,
        amount: 1,
        refine: 0,
        is_identified: false,
        equip: item.location as i32,
        is_damaged: false,
        card0: 0,
        card1: 0,
        card2: 0,
        card3: 0,
        name_english: item.name_english.clone(),
        weight: item.weight
    };
    character.add_in_inventory(inventory_item)
}