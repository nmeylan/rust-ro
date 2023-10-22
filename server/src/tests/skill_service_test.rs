#![allow(dead_code)]


use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::skill_service::SkillService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct SkillServiceTestContext {
    test_context: TestContext,
    skill_service: SkillService,
}

fn before_each() -> SkillServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> SkillServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    SkillServiceTestContext {
        test_context:TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        skill_service: SkillService::new(client_notification_sender, persistence_event_sender, GlobalConfigService::instance()),
    }
}