use tokio::runtime::Runtime;
use crate::server::core::inventory_item::InventoryItem;
use crate::server::enums::item::ItemType;
use crate::server::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::Server;

pub fn get_items(char_id: u32, server: &Server, runtime: &Runtime, item_ids: Vec<i32>, item_amounts: Vec<i16>) {
    let items = runtime.block_on(async { server.repository.get_items(item_ids, item_amounts).await }).unwrap();
    server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems{
        char_id,
        should_perform_check: true,
        items: items.iter().map(|item| InventoryItem{
            item_id: item.id as u32,
            item_type: ItemType::from_string(item.item_type.as_str()),
            amount: item.amount as u16,
            weight: item.weight as u16,
            name_english: item.name_english.clone()
        }).collect()
    }));
}