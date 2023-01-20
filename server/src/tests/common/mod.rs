pub mod character_helper;

use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Mutex, Once};
use crate::repository::model::item_model::ItemModel;
use crate::server::core::configuration::Config;
use crate::server::core::map_instance::MapInstanceKey;
use crate::server::events::client_notification::Notification;
use crate::server::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;

pub static mut CONFIGS: Option<Config> = None;
pub static mut TEST_CONTEXT: Option<TestContext> = None;
static INIT: Once = Once::new();

pub struct TestContext {
    pub(crate) client_notification_sender: SyncSender<Notification>,
    pub(crate) client_notification_receiver: Receiver<Notification>,
    pub(crate) persistence_event_receiver: Receiver<PersistenceEvent>,
    pub(crate) persistence_event_sender: SyncSender<PersistenceEvent>,
}

impl TestContext {
    pub fn client_notification_sender(&self) -> SyncSender<Notification> {
        self.client_notification_sender.clone()
    }

    pub fn persistence_event_sender(&self) -> SyncSender<PersistenceEvent> {
        self.persistence_event_sender.clone()
    }
}

pub fn create_mpsc<T>() -> (SyncSender<T>, Receiver<T>) {
    std::sync::mpsc::sync_channel::<T>(0)
}

pub fn before_all() {
    INIT.call_once(|| {
        let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
        let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
        unsafe {
            let config: Config = toml::from_str(&fs::read_to_string("../config.toml").unwrap()).unwrap();
            CONFIGS = Some(config);
            TEST_CONTEXT = Some(TestContext { client_notification_sender, persistence_event_sender, client_notification_receiver, persistence_event_receiver});
        }
        let skills_config = Config::load_skills_config("..").unwrap();
        let mut skill_configs_id_name: HashMap<String, u32> = Default::default();
        skills_config.values().for_each(|skill_config| {
            skill_configs_id_name.insert(skill_config.name.clone(), skill_config.id);
        });
        let mut items_id_name: HashMap<String, u32> = Default::default();
        let items: Vec<ItemModel> = vec![];
        items.iter().for_each(|item| {
            items_id_name.insert(item.name_aegis.clone(), item.id as u32);
        });
        let job_configs = Config::load_jobs_config("..").unwrap();
        let configs = unsafe { CONFIGS.as_ref().unwrap() };
        crate::GlobalConfigService::init(&configs,
                                         items.into_iter().map(|item| (item.id as u32, item)).collect(),
                                         items_id_name,
                                         job_configs,
                                         skills_config,
                                         skill_configs_id_name);
    });
}
