use std::collections::HashMap;
use std::sync::{Arc, Once};
use std::sync::mpsc::SyncSender;
use base64::Engine;
use base64::engine::general_purpose;
use rathena_script_lang_interpreter::lang::chunk::{ClassFile};
use rathena_script_lang_interpreter::lang::chunk::OpCode::{CallNative, LoadConstant, LoadValue, LoadGlobal};
use rathena_script_lang_interpreter::lang::compiler::{Compiler, DebugFlag};
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use models::enums::bonus::BonusType;


use packets::packets::PacketZcUseItemAck2;
use crate::repository::ItemRepository;
use crate::repository::model::item_model::{ItemModel};

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterUseItem};
use crate::server::model::events::persistence_event::{DeleteItems, PersistenceEvent};
use crate::server::script::{PlayerScriptHandler};
use crate::server::Server;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::util::cell::{MyRef, MyUnsafeCell};


#[allow(dead_code)]
pub struct ItemService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn ItemRepository>,
    configuration_service: &'static GlobalConfigService,
    item_script_cache: MyUnsafeCell<HashMap<u32, ClassFile>>,
    item_script_vm: Arc<Vm>,
}

impl ItemService {
    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn ItemRepository>, item_script_vm: Arc<Vm>, configuration_service: &'static GlobalConfigService) -> Self {
        ItemService { client_notification_sender, persistence_event_sender, repository, configuration_service, item_script_cache: Default::default(), item_script_vm }
    }

    pub fn get_item_script(&self, item_id: i32, runtime: &Runtime) -> Option<MyRef<ClassFile>> {
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
    pub fn use_item(&self, server_ref: &Server, runtime: &Runtime, character_user_item: CharacterUseItem, character: &mut Character) {
        if let Some(item) = character.get_item_from_inventory(character_user_item.index) {
            if item.item_type().is_consumable() {
                // TODO check if char can use (class restriction, level restriction)
                //TODO rework, this is deprecated, items script are now compiled. In addition it is not efficient to instantiate PlayerScriptHandler each time
                let maybe_script_ref = self.get_item_script(item.item_id, runtime);
                if maybe_script_ref.is_some() {
                    let script = maybe_script_ref.as_ref().unwrap();
                    let (tx, _rx) = mpsc::channel(1);
                    let session = server_ref.state().get_session(character.account_id);
                    session.set_script_handler_channel_sender(tx);
                    let script_result = Vm::repl(self.item_script_vm.clone(), script,
                                                 Box::new(PlayerScriptHandler::instance()), vec![0, character.char_id, character.account_id]);
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

    pub fn convert_script_into_bonuses(items: &mut Vec<ItemModel>, native_function_file_path: &str) -> (i32, i32) {
        let vm = Arc::new(Vm::new(native_function_file_path, rathena_script_lang_interpreter::lang::vm::DebugFlag::None.value()));
        let script_handler = crate::server::script::bonus::BonusScriptHandler::new();
        let mut count_script_executed = 0;
        let mut count_script_skipped = 0;
        for item in items.iter_mut() {
            if let Some(script_compilation) = &item.script_compilation {
                let script = general_purpose::STANDARD.decode(script_compilation).unwrap();
                let maybe_class = Compiler::from_binary(&script).unwrap().pop();
                let class_file = maybe_class.as_ref().unwrap();
                let script_main = class_file.functions().iter().find(|f| f.name == "_main").map(|f| f.chunk.clone()).unwrap();
                let mut complex_script = false;
                for op_code in script_main.op_codes.borrow().iter() {
                    match op_code {
                        LoadConstant(_) | LoadValue | LoadGlobal => {}
                        CallNative { reference, .. } => {
                            let native = vm.get_from_native_pool(*reference).unwrap();
                            if !(native.name == "bonus" || native.name == "bonus2" || native.name == "bonus3" || native.name == "bonus4" || native.name == "bonus5" || native.name == "skill") {
                                complex_script = true;
                                break;
                            }
                        }
                        // When script contains those op code, it means script need to be executed each time the item is used
                        // E.g Gibbet_card          Rybio_Card
                        // if (getrefine()<6)       bonus2 bAddEffWhenHit,Eff_Stun,300+600*(readparam(bDex)>=77);
                        //    bonus bMdef,5;
                        _ => {
                            complex_script = true;
                            break;
                        }
                    }
                }

                let _ = Vm::repl(vm.clone(), class_file, Box::new(&script_handler), vec![]);
                item.bonuses = script_handler.drain();

                item.bonuses.iter().filter_map(|b| {
                    if let BonusType::ElementWeapon(element) = b { Some(*element) } else { None }
                }).for_each(|element| { item.element = Some(element); });
                if !complex_script {
                    item.script_compilation = None;
                    count_script_executed += 1;
                } else {
                    count_script_skipped += 1;
                    item.item_bonuses_are_dynamic = true;
                }
            }
        }
        (count_script_executed, count_script_skipped)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use models::enums::bonus::BonusType;
    use crate::repository::model::item_model::ItemModels;
    use crate::server::service::item_service::ItemService;

    #[test]
    fn test_initialize_items_with_static_bonus() {
        // Given
        let mut item_models = serde_json::from_str::<ItemModels>(&fs::read_to_string("../config/items.json").unwrap()).unwrap().items;
        // When
        ItemService::convert_script_into_bonuses(&mut item_models, "../native_functions_list.txt");
        // Then
        let mantis_card = item_models.iter().find(|i| i.name_aegis == "Mantis_Card").unwrap();
        assert!(matches!(mantis_card.bonuses[0], BonusType::Str(3)));
    }
}