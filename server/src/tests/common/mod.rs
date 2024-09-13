#![allow(dead_code)]

pub mod character_helper;
pub mod map_instance_helper;
pub mod mob_helper;
#[macro_use]
pub mod assert_helper;
pub mod mocked_repository;
pub mod server_helper;
pub mod item_helper;
pub mod sync_helper;
pub mod fixtures;
#[cfg(feature = "integration_tests")]
pub mod integration_test;


use std::{fs, thread};
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, Once};
use log::{LevelFilter};
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use simple_logger::SimpleLogger;

use crate::repository::model::item_model::{ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use configuration::configuration::Config;
use packets::packets::Packet;
use crate::repository::Repository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::item_service::ItemService;
use crate::server::state::server::ServerState;
use crate::tests::common::mocked_repository::MockedRepository;
use crate::tests::common::sync_helper::{CountDownLatch, IncrementLatch};
use crate::util::cell::MyUnsafeCell;

static mut CONFIGS: Option<Config> = None;
static INIT: Once = Once::new();

pub struct TestContext {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    received_notification: Arc<Mutex<Vec<Notification>>>,
    received_persistence_events: Arc<Mutex<Vec<PersistenceEvent>>>,
    countdown_latch: CountDownLatch,
    increment_latch: IncrementLatch,
}

impl TestContext {
    pub fn new(client_notification_sender: SyncSender<Notification>, client_notification_receiver: Receiver<Notification>,
               persistence_event_sender: SyncSender<PersistenceEvent>, persistence_event_receiver: Receiver<PersistenceEvent>, countdown_latch: CountDownLatch) -> Self {
        let received_notification = Arc::new(Mutex::new(vec![]));
        let received_persistence_events = Arc::new(Mutex::new(vec![]));
        let received_notification_cloned = received_notification.clone();
        let received_persistence_events_cloned = received_persistence_events.clone();
        let count_down_latch_clone = countdown_latch.clone();
        let increment_latch = IncrementLatch::new();
        let increment_latch_clone = increment_latch.clone();
        thread::Builder::new().name("client_notification_thread".to_string()).spawn(move || {
            for notification in client_notification_receiver.iter() {
                // println!("Sent client notification {:?}", notification);
                received_notification_cloned.lock().unwrap().push(notification);
                count_down_latch_clone.countdown();
                increment_latch_clone.increment();
            }
        }).unwrap();
        let count_down_latch_clone = countdown_latch.clone();
        let increment_latch_clone = increment_latch.clone();
        thread::Builder::new().name("persistence_event_thread".to_string()).spawn(move || {
            for notification in persistence_event_receiver.iter() {
                // println!("Sent persistence event {:?}", notification);
                received_persistence_events_cloned.lock().unwrap().push(notification);
                count_down_latch_clone.countdown();
                increment_latch_clone.increment();
            }
        }).unwrap();
        Self {
            client_notification_sender,
            persistence_event_sender,
            received_notification,
            received_persistence_events,
            countdown_latch,
            increment_latch
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

    pub fn increment_latch(&self) -> &IncrementLatch {
        &self.increment_latch
    }

    pub fn reset_increment_latch(&mut self) {
        self.increment_latch.reset();
    }

    pub fn clear_sent_packet(&self) {
        *self.received_notification.lock().unwrap() = vec![];
    }

    pub fn get_sent_packet(&self, packet_ids: Vec<&str>, packetver: u32) -> Vec<Box<dyn Packet>> {
        let notification_guard = self.received_notification.lock().unwrap();
        let mut packets_found = vec![];
        for notification in notification_guard.iter() {
            let packet_sent = notification.packet(packetver);
            if packet_ids.contains(&packet_sent.id(packetver)) {
                packets_found.push(packet_sent);
            }
        }
        packets_found
    }
}

pub fn create_mpsc<T>() -> (SyncSender<T>, Receiver<T>) {
    std::sync::mpsc::sync_channel::<T>(0)
}

pub fn before_all() {
    INIT.call_once(|| {
        SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

        unsafe {
            let mut config: Config = serde_json::from_str(&fs::read_to_string("../config.template.json").unwrap()).unwrap();
            let file_path = "../config/status_point_reward.json";
            Config::set_config_status_point_rewards(&mut config, file_path).unwrap();
            let file_path = "../config/status_point_raising_cost.json";
            Config::set_config_status_point_raising_cost(&mut config, file_path).unwrap();
            let file_path = "../config/exp.json";
            Config::set_exp_requirements(&mut config, file_path).unwrap();
            config.game.mob_move_frequency_when_no_player_around = 0.95;
            config.game.mob_spawn_refresh_frequency = 0.2;
            config.game.mob_action_refresh_frequency = 0.2;
            CONFIGS = Some(config);
        }
        let skills_config = Config::load_skills_config("..").unwrap();

        let item_models = serde_json::from_str::<ItemModels>(&fs::read_to_string("../config/items.json").unwrap());
        let mut items: Vec<ItemModel> = item_models.unwrap().into();
        ItemService::convert_script_into_bonuses(&mut items, "../native_functions_list.txt");

        let mob_models = serde_json::from_str::<MobModels>(&fs::read_to_string("../config/mobs.json").unwrap());
        let mobs: Vec<MobModel> = mob_models.unwrap().into();

        let job_configs = Config::load_jobs_config("..").unwrap();
        let job_skills_tree = Config::load_jobs_skill_tree("..").unwrap();
        let config = unsafe { CONFIGS.clone().unwrap() };
        crate::GlobalConfigService::init(config,
                                         items,
                                         mobs,
                                         job_configs,
                                         job_skills_tree,
                                         skills_config,
                                         Default::default());
    });
    // unsafe { GlobalConfigService::instance_mut().set_config(CONFIGS.clone().unwrap()); }
}

pub fn mocked_repository() -> Arc<MockedRepository> {
    Arc::new(MockedRepository)
}

pub fn test_script_vm() -> Arc<Vm> {
    Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value()))
}

// pub struct ServerBuilder {
//     pub configuration: &'static Config,
//     pub repository: Arc<MockedRepository>,
//     state: MyUnsafeCell<ServerState>,
//     tasks_queue: Arc<TasksQueue<GameEvent>>,
//     movement_tasks_queue: Arc<TasksQueue<GameEvent>>,
// }
//
// impl ServerBuilder {
//     pub fn new(configuration: &'static Config) -> Self {
//         ServerBuilder {
//             configuration,
//             repository: mocked_repository(),
//             state: (),
//             tasks_queue: Arc::new(Default::default()),
//             movement_tasks_queue: Arc::new(Default::default()),
//         }
//     }
// }

#[macro_export]
macro_rules! status_snapshot {
    ($context:ident, $object:expr) => {
        &$context.status_service.to_snapshot(&$object.status)
    }
}
#[macro_export]
macro_rules! status_snapshot_mob {
    ($context:ident, $object:expr) => {
        &$context.status_service.to_snapshot_mob(&$object.status)
    }
}
