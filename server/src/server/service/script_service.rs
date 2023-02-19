use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};
use tokio::runtime::Runtime;

use crate::repository::{ItemRepository};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::script::Value;
use crate::server::service::global_config_service::GlobalConfigService;

static mut SERVICE_INSTANCE: Option<ScriptService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct ScriptService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    repository: Arc<dyn ItemRepository>,
    server_task_queue: Arc<TasksQueue<GameEvent>>
}

impl ScriptService {
    pub fn instance() -> &'static ScriptService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, repository: Arc<dyn ItemRepository>, server_task_queue: Arc<TasksQueue<GameEvent>>) -> Self {
        ScriptService { client_notification_sender, configuration_service, repository, server_task_queue }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, repository: Arc<dyn ItemRepository>, server_task_queue: Arc<TasksQueue<GameEvent>>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ScriptService::new(client_notification_sender, configuration_service, repository, server_task_queue));
        });
    }

    pub fn schedule_get_items(&self, char_id: u32, runtime: &Runtime, item_ids_amounts: Vec<(Value, i16)>, buy: bool) {
        let mut items = runtime.block_on(async { self.repository.get_items(item_ids_amounts.iter().map(|(v, _)| v.clone()).collect()).await }).unwrap();
        items.iter_mut().for_each(|item| item.amount = item_ids_amounts.iter().find(|(id, _amount)|
            match id {
                Value::Number(v) => *v == item.id,
                Value::String(v) => v.to_lowercase() == item.name_aegis.to_lowercase(),
            }
        ).unwrap().1);
        self.server_task_queue.add_to_first_index(GameEvent::CharacterAddItems(CharacterAddItems {
            char_id,
            should_perform_check: true,
            buy,
            items: items.iter().map(|item| InventoryItemModel::from_item_model(self.configuration_service.get_item(item.id), item.amount, true)).collect(),
        }));
    }
}