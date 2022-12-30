use std::collections::HashMap;
use std::sync::Once;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use tokio::runtime::Runtime;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::enums::item::ItemType;
use crate::server::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::Server;
use crate::util::cell::{MyRef, MyUnsafeCell};


static mut ITEM_SERVICE_INSTANCE: Option<ItemService> = None;
static ITEM_SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct ItemService {
    item_script_cache: MyUnsafeCell<HashMap<u32, ClassFile>>,
}

impl ItemService {
    pub fn instance() -> &'static ItemService {
        ITEM_SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            ITEM_SERVICE_INSTANCE = Some(ItemService::new());
        });
        unsafe { ITEM_SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn new() -> Self {
        ItemService {
            item_script_cache: Default::default(),
        }
    }
    pub fn get_items(&self, char_id: u32, server: &Server, runtime: &Runtime, item_ids_amounts: Vec<(i32, i16)>, buy: bool) {
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

    pub fn get_item_script(&self, item_id: i32, server: &Server, runtime: &Runtime) -> Option<MyRef<ClassFile>> {
        if !self.item_script_cache.borrow().contains_key(&(item_id as u32)) {
            if let Some(script) = runtime.block_on(async { server.repository.get_item_script(item_id).await }).ok() {
                let mut compilation_result = Compiler::compile_script(format!("item_script_{}", item_id), script.as_str(), "native_functions_list.txt", DebugFlag::None.value());
                if compilation_result.is_err() {
                    error!("Failed to compile item script for item id: {}, due to", item_id);
                    compilation_result.err().unwrap().iter().for_each(|e| {
                        error!("{}", e)
                    });
                   return None;
                }
                self.item_script_cache.borrow_mut().insert(item_id as u32, compilation_result.unwrap().pop().unwrap());
            }
        }
        if !self.item_script_cache.borrow().contains_key(&(item_id as u32)) {
            return Some(MyRef::map(self.item_script_cache.borrow(), |scripts| scripts.get(&(item_id as u32)).unwrap()));
        }
        None
    }
}
