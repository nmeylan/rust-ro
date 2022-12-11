use tokio::runtime::Runtime;
use crate::server::core::inventory_item::InventoryItem;
use crate::server::enums::item::ItemType;
use crate::server::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::Server;

pub fn get_items(char_id: u32, server: &Server, runtime: &Runtime, item_ids_amounts: Vec<(i32, i16)>) {
    let mut items = runtime.block_on(async { server.repository.get_items(item_ids_amounts.iter().map(|(id, _)| *id).collect()).await }).unwrap();
    items.iter_mut().for_each(|item| item.amount = item_ids_amounts.iter().find(|(id, amount)| item.id == *id).unwrap().1);
    server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems{
        char_id,
        should_perform_check: true,
        items: items.iter().map(|item| InventoryItem{
            item_id: item.id as u16,
            item_type: ItemType::from_string(item.item_type.as_str()),
            amount: item.amount as u16,
            weight: item.weight as u16,
            name_english: item.name_english.clone(),
            is_identified:  true
        }).collect()
    }));
}