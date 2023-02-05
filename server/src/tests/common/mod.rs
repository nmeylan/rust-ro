pub mod character_helper;
pub mod map_instance_helper;
pub mod mob_helper;
#[macro_use]
pub mod assert_helper;
pub mod mocked_repository;
pub mod server_helper;

use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Once};
use crate::repository::model::item_model::{ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::model::configuration::Config;

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;



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
    std::sync::mpsc::sync_channel::<T>(20480)
}

pub fn before_all() {
    INIT.call_once(|| {
        let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
        let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
        unsafe {
            let config: Config = serde_json::from_str(&fs::read_to_string("../config.template.json").unwrap()).unwrap();
            CONFIGS = Some(config);
            TEST_CONTEXT = Some(TestContext { client_notification_sender, persistence_event_sender, client_notification_receiver, persistence_event_receiver});
        }
        let skills_config = Config::load_skills_config("..").unwrap();
        let mut skill_configs_id_name: HashMap<String, u32> = Default::default();
        skills_config.values().for_each(|skill_config| {
            skill_configs_id_name.insert(skill_config.name.clone(), skill_config.id);
        });
        let mut items_id_name: HashMap<String, u32> = Default::default();
        let item_models = serde_json::from_str::<ItemModels>(&fs::read_to_string("../config/items.json").unwrap());
        let items: Vec<ItemModel> = item_models.unwrap().into();
        items.iter().for_each(|item| {
            items_id_name.insert(item.name_aegis.clone(), item.id as u32);
        });

        let mut mobs_id_name: HashMap<String, u32> = Default::default();
        let mob_models = serde_json::from_str::<MobModels>(&fs::read_to_string("../config/mobs.json").unwrap());
        let mobs: Vec<MobModel> = mob_models.unwrap().into();
        mobs.iter().for_each(|mob| {
            mobs_id_name.insert(mob.name.clone(), mob.id as u32);
        });

        let job_configs = Config::load_jobs_config("..").unwrap();
        let configs = unsafe { CONFIGS.as_ref().unwrap() };
        crate::GlobalConfigService::init(configs,
                                         items.into_iter().map(|item| (item.id as u32, item)).collect(),
                                         items_id_name,
                                         mobs.into_iter().map(|mob| (mob.id as u32, mob)).collect(),
                                         mobs_id_name,
                                         job_configs,
                                         skills_config,
                                         skill_configs_id_name, Default::default());
    });
}
