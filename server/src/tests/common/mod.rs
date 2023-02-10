pub mod character_helper;
pub mod map_instance_helper;
pub mod mob_helper;
#[macro_use]
pub mod assert_helper;
pub mod mocked_repository;
pub mod server_helper;
pub mod item_helper;

use std::collections::HashMap;
use std::{fs, thread};
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
use crate::repository::model::item_model::{ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::model::configuration::Config;

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::tests::common::mocked_repository::MockedRepository;


pub static mut CONFIGS: Option<Config> = None;
static INIT: Once = Once::new();

pub struct TestContext {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    received_notification: Arc<Mutex<Vec<Notification>>>,
    received_persistence_events: Arc<Mutex<Vec<PersistenceEvent>>>,
}

impl TestContext {
    pub fn new(client_notification_sender: SyncSender<Notification>, client_notification_receiver: Receiver<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, persistence_event_receiver: Receiver<PersistenceEvent>) -> Self {
        let received_notification = Arc::new(Mutex::new(vec![]));
        let received_persistence_events = Arc::new(Mutex::new(vec![]));
        let received_notification_cloned = received_notification.clone();
        let received_persistence_events_cloned = received_persistence_events.clone();
        thread::Builder::new().name("client_notification_thread".to_string()).spawn(move || {
            for notification in client_notification_receiver.iter() {
                println!("Received notification {:?}", notification);
                received_notification_cloned.lock().unwrap().push(notification);
            }
            println!("client_notification_thread exit");
        }).unwrap();
        thread::Builder::new().name("persistence_event_thread".to_string()).spawn(move || {
            for notification in persistence_event_receiver.iter() {
                received_persistence_events_cloned.lock().unwrap().push(notification);
            }
            println!("persistence_event_thread exit");
        }).unwrap();
        Self {
            client_notification_sender,
            persistence_event_sender,
            received_notification,
            received_persistence_events,
        }
    }
    pub fn client_notification_sender(&self) -> SyncSender<Notification> {
        self.client_notification_sender.clone()
    }

    pub fn persistence_event_sender(&self) -> SyncSender<PersistenceEvent> {
        self.persistence_event_sender.clone()
    }

    pub fn has_sent_notification(&self, notification: Notification) -> bool {
        // Need a better synchronisation mechanism, like a countdown latch with a timeout
        thread::sleep(Duration::from_millis(20));
        let notifications_guard = self.received_notification.lock().unwrap();
        notifications_guard.iter().find(|sent_notification| {
            match sent_notification {
                Notification::Char(sent_char_notification) => {
                    match &notification {
                        Notification::Char(char_notification) => {sent_char_notification.char_id() == char_notification.char_id()}
                        Notification::Area(area_notification) => {false}
                    }
                }
                Notification::Area(sent_area_notification) => {
                    match &notification {
                        Notification::Char(char_notification) => {false}
                        Notification::Area(area_notification) => {sent_area_notification.map_name == area_notification.map_name
                            && sent_area_notification.map_instance_id == area_notification.map_instance_id
                            && sent_area_notification.range_type == area_notification.range_type }
                    }
                }
            }

        }).is_some()
    }

    pub fn has_sent_persistence_event(&self, persistence_event: PersistenceEvent) -> bool {
        // Need a better synchronisation mechanism, like a countdown latch with a timeout
        thread::sleep(Duration::from_millis(20));
        let notifications_guard = self.received_persistence_events.lock().unwrap();
        notifications_guard.iter().find(|sent_persistence_event| if matches!(&persistence_event, sent_persistence_event) { persistence_event == **sent_persistence_event } else { false }).is_some()
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
