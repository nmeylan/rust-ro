use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;

static mut SERVICE_INSTANCE: Option<SkillService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct SkillService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    configuration_service: &'static GlobalConfigService,
}

impl SkillService {
    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) -> SkillService {
        SkillService { client_notification_sender, persistence_event_sender, configuration_service }
    }
    pub fn instance() -> &'static SkillService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(SkillService::new(client_notification_sender, persistence_event_sender, configuration_service));
        });
    }

    pub fn hit_count(&self, skid: u32, level: u32) -> u8 {
        let config = self.configuration_service.get_skill_config(skid);
        if let Some(hit_per_level) = config.hit_count_per_level() {
            return hit_per_level[level as usize] as u8;
        } else if let Some(hit) = config.hit_count() {
            return *hit as u8;
        }
        1
    }
}
