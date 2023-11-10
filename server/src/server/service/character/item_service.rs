use std::collections::HashMap;
use std::sync::{Arc, Once, RwLock};
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;


use packets::packets::PacketZcUseItemAck2;
use crate::repository::{ItemRepository, Repository};

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterUseItem};
use crate::server::model::events::persistence_event::{DeleteItems, PersistenceEvent};
use crate::server::script::{PlayerScriptHandler};
use crate::server::Server;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::util::cell::{MyRef, MyUnsafeCell};


static mut SERVICE_INSTANCE: Option<ItemService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct ItemService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn ItemRepository>,
    configuration_service: &'static GlobalConfigService,
    item_script_cache: MyUnsafeCell<HashMap<u32, ClassFile>>,
}

impl ItemService {
    pub fn instance() -> &'static ItemService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }
    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<Repository>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ItemService{ client_notification_sender, persistence_event_sender, repository, configuration_service, item_script_cache: Default::default() });
        });
    }

    pub fn get_item_script(&self, item_id: i32,runtime: &Runtime) -> Option<MyRef<ClassFile>> {
        if !self.item_script_cache.borrow().contains_key(&(item_id as u32)) {
            if let Ok(script) = runtime.block_on(async { self.repository.get_item_script(item_id).await }) {
                let compilation_result = Compiler::compile_script(format!("item_script_{item_id}"), script.as_str(), "native_functions_list.txt", DebugFlag::None.value());
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
    pub fn use_item(&self, server_ref: Arc<Server>, runtime: &Runtime, character_user_item: CharacterUseItem, character: &mut Character) {
        if let Some(item) = character.get_item_from_inventory(character_user_item.index) {
            if item.item_type().is_consumable() {
                // TODO check if char can use (class restriction, level restriction)
                let maybe_script_ref = ItemService::instance().get_item_script(item.item_id, runtime);
                if maybe_script_ref.is_some() {
                    let script = maybe_script_ref.as_ref().unwrap();
                    let (tx, rx) = mpsc::channel(1);
                    let session = server_ref.state().get_session(character.account_id);
                    session.set_script_handler_channel_sender(tx);
                    let script_result = Vm::repl(server_ref.vm.clone(), script,
                                                 Box::new(&PlayerScriptHandler {
                                                     client_notification_channel: self.client_notification_sender.clone(),
                                                     npc_id: 0,
                                                     server: server_ref.clone(),
                                                     player_action_receiver: RwLock::new(rx),
                                                     runtime: Runtime::new().unwrap(),
                                                     session,
                                                     configuration_service: self.configuration_service
                                                 }));
                    let mut packet_zc_use_item_ack = PacketZcUseItemAck2::new(self.configuration_service.packetver());
                    packet_zc_use_item_ack.set_aid(character_user_item.char_id);
                    packet_zc_use_item_ack.set_index(character_user_item.index as u16);
                    let item_inventory_id = item.id;
                    let item_unique_id = item.unique_id;
                    if script_result.is_ok() {
                        // TODO call delete item from inventory service
                        let remaining_item = character.del_item_from_inventory(character_user_item.index, 1);
                        self.persistence_event_sender.send(PersistenceEvent::DeleteItemsFromInventory(DeleteItems {
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
                    self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_use_item_ack.raw))).expect("Fail to send client notification");
                }
            }
        }
        // check if can use
        // check if potion has been created by famous (ranked) alch/creator, bonus + 50%
    }
}
