use std::collections::HashMap;
use std::sync::{Arc, Once, RwLock};
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use crate::repository::model::item_model::InventoryItemModel;
use enums::item::ItemType;
use packets::packets::PacketZcUseItemAck2;
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::game_event::{CharacterAddItems, CharacterUseItem, GameEvent};
use crate::server::events::persistence_event::{DeleteItems, PersistenceEvent};
use crate::server::script::PlayerScriptHandler;
use crate::server::Server;
use crate::server::state::character::Character;
use crate::util::cell::{MyRef, MyUnsafeCell};


static mut SERVICE_INSTANCE: Option<ItemService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct ItemService {
    item_script_cache: MyUnsafeCell<HashMap<u32, ClassFile>>,
}

impl ItemService {
    pub fn instance() -> &'static ItemService {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ItemService::new());
        });
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    fn new() -> Self {
        Self {
            item_script_cache: Default::default(),
        }
    }
    pub fn schedule_get_items(&self, char_id: u32, server: &Server, runtime: &Runtime, item_ids_amounts: Vec<(i32, i16)>, buy: bool) {
        let mut items = runtime.block_on(async { server.repository.get_items(item_ids_amounts.iter().map(|(id, _)| *id as i32).collect()).await }).unwrap();
        items.iter_mut().for_each(|item| item.amount = item_ids_amounts.iter().find(|(id, _amount)| item.id == *id).unwrap().1);
        server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems {
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
                is_identified: true,
                refine: 0,
                is_damaged: false,
                card0: 0,
                card1: 0,
                card2: 0,
                equip: 0,
                card3: 0,
            }).collect(),
        }));
    }

    pub fn get_item_script(&self, item_id: i32, server: &Server, runtime: &Runtime) -> Option<MyRef<ClassFile>> {
        if !self.item_script_cache.borrow().contains_key(&(item_id as u32)) {
            if let Ok(script) = runtime.block_on(async { server.repository.get_item_script(item_id).await }) {
                let compilation_result = Compiler::compile_script(format!("item_script_{}", item_id), script.as_str(), "native_functions_list.txt", DebugFlag::None.value());
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
        if self.item_script_cache.borrow().contains_key(&(item_id as u32)) {
            return Some(MyRef::map(self.item_script_cache.borrow(), |scripts| scripts.get(&(item_id as u32)).unwrap()));
        }
        None
    }

    #[metrics::elapsed]
    pub fn use_item(&self, server_ref: Arc<Server>, runtime: &Runtime, persistence_event_sender: &SyncSender<PersistenceEvent>, character_user_item: CharacterUseItem, character: &mut Character) {
        if let Some(item) = character.get_item_from_inventory(character_user_item.index) {
            if item.item_type.is_consumable() {
                // TODO check if char can use (class restriction, level restriction)
                let maybe_script_ref = ItemService::instance().get_item_script(item.item_id, server_ref.as_ref(), runtime);
                if maybe_script_ref.is_some() {
                    let script = maybe_script_ref.as_ref().unwrap();
                    let (tx, rx) = mpsc::channel(1);
                    let session = server_ref.get_session(character.account_id);
                    session.set_script_handler_channel_sender(tx);
                    let script_result = Vm::repl(server_ref.vm.clone(), script,
                                                 Box::new(&PlayerScriptHandler {
                                                     client_notification_channel: server_ref.client_notification_sender.clone(),
                                                     npc_id: 0,
                                                     server: server_ref.clone(),
                                                     player_action_receiver: RwLock::new(rx),
                                                     runtime: Runtime::new().unwrap(),
                                                     session,
                                                 }));
                    let mut packet_zc_use_item_ack = PacketZcUseItemAck2::new();
                    packet_zc_use_item_ack.set_aid(character_user_item.char_id);
                    packet_zc_use_item_ack.set_index(character_user_item.index as u16);
                    if script_result.is_ok() {
                        let item_inventory_id = item.id;
                        let item_unique_id = item.unique_id;
                        drop(item); // Drop here is to indicate to compiler we don't use item anymore so we can borrow character mutably to perform del_item_from_inventory
                        let remaining_item = character.del_item_from_inventory(character_user_item.index, 1);
                        persistence_event_sender.send(PersistenceEvent::DeleteItemsFromInventory(DeleteItems {
                            char_id: character.char_id as i32,
                            item_inventory_id,
                            unique_id: item_unique_id,
                            amount: remaining_item as i16,
                        })).expect("Failed to send delete item event");
                        packet_zc_use_item_ack.set_count(remaining_item as i16);
                        packet_zc_use_item_ack.set_result(true);
                    } else {
                        error!("Fail to execute script for item: {} reason: {}", item.item_id, script_result.err().unwrap());
                        packet_zc_use_item_ack.set_result(false);
                    }
                    packet_zc_use_item_ack.fill_raw();
                    server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_use_item_ack.raw))).expect("Fail to send client notification");
                }
            }
        }
        // check if can use
        // check if potion has been created by famous (ranked) alch/creator, bonus + 50%
    }
}
