use tokio::runtime::Runtime;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::enums::item::ItemType;
use crate::server::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::Server;

pub fn get_items(char_id: u32, server: &Server, runtime: &Runtime, item_ids_amounts: Vec<(i32, i16)>, buy: bool) {
    let mut items = runtime.block_on(async { server.repository.get_items(item_ids_amounts.iter().map(|(id, _)| *id as i32).collect()).await }).unwrap();
    items.iter_mut().for_each(|item| item.amount = item_ids_amounts.iter().find(|(id, _amount)| item.id == *id).unwrap().1);
    server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems{
        char_id,
        should_perform_check: true,
        buy,
        items: items.iter().map(|item| InventoryItemModel {
            id: 0,
            unique_id: 0,
            item_id: item.id,
            item_type: ItemType::from_string(item.item_type.as_str()),
            amount: item.amount,
            weight: item.weight,
            name_english: item.name_english.clone(),
            is_identified:  true,
            refine: 0,
            is_damaged: false,
            card0: 0,
            card1: 0,
            card2: 0,
            equip: 0,
            card3: 0
        }).collect()
    }));
}