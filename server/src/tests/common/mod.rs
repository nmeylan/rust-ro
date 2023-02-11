pub mod character_helper;
pub mod map_instance_helper;
pub mod mob_helper;
#[macro_use]
pub mod assert_helper;
pub mod mocked_repository;
pub mod server_helper;
pub mod item_helper;
pub mod sync_helper;

use std::collections::HashMap;
use std::{fs, thread};
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use packets::packets::Packet;
use packets::packets_parser::parse;
use crate::repository::model::item_model::{ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::model::configuration::Config;

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::tests::common::mocked_repository::MockedRepository;
use crate::tests::common::sync_helper::CountDownLatch;


pub static mut CONFIGS: Option<Config> = None;
static INIT: Once = Once::new();

pub struct TestContext {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    received_notification: Arc<Mutex<Vec<Notification>>>,
    received_persistence_events: Arc<Mutex<Vec<PersistenceEvent>>>,
    countdown_latch: CountDownLatch,
}

impl TestContext {
    pub fn new(client_notification_sender: SyncSender<Notification>, client_notification_receiver: Receiver<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, persistence_event_receiver: Receiver<PersistenceEvent>, countdown_latch: CountDownLatch) -> Self {
        let received_notification = Arc::new(Mutex::new(vec![]));
        let received_persistence_events = Arc::new(Mutex::new(vec![]));
        let received_notification_cloned = received_notification.clone();
        let received_persistence_events_cloned = received_persistence_events.clone();
        let count_down_latch_clone = countdown_latch.clone();
        thread::Builder::new().name("client_notification_thread".to_string()).spawn(move || {
            for notification in client_notification_receiver.iter() {
                println!("Received notification {:?}", notification);
                received_notification_cloned.lock().unwrap().push(notification);
                count_down_latch_clone.countdown();
            }
            println!("client_notification_thread exit");
        }).unwrap();
        let count_down_latch_clone = countdown_latch.clone();
        thread::Builder::new().name("persistence_event_thread".to_string()).spawn(move || {
            for notification in persistence_event_receiver.iter() {
                received_persistence_events_cloned.lock().unwrap().push(notification);
                count_down_latch_clone.countdown();
            }
            println!("persistence_event_thread exit");
        }).unwrap();
        Self {
            client_notification_sender,
            persistence_event_sender,
            received_notification,
            received_persistence_events,
            countdown_latch
        }
    }
    pub fn client_notification_sender(&self) -> SyncSender<Notification> {
        self.client_notification_sender.clone()
    }

    pub fn persistence_event_sender(&self) -> SyncSender<PersistenceEvent> {
        self.persistence_event_sender.clone()
    }

    pub fn received_notification(&self) -> Arc<Mutex<Vec<Notification>>> {
        self.received_notification.clone()
    }
    pub fn received_persistence_events(&self) -> Arc<Mutex<Vec<PersistenceEvent>>> {
        self.received_persistence_events.clone()
    }
    pub fn countdown_latch(&self) -> &CountDownLatch {
        &self.countdown_latch
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
            let config: Config = serde_json::from_str(&fs::read_to_string("../config.template.json").unwrap()).unwrap();
            CONFIGS = Some(config);
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

pub fn mocked_repository() -> Arc<MockedRepository> {
    Arc::new(MockedRepository::default())
}
