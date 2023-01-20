use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::server::events::client_notification::Notification;
use crate::server::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TEST_CONTEXT, TestContext};

struct StatusServiceTestContext {
    test_context: TestContext,
    status_service: StatusService,
}

fn before_each() -> StatusServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    StatusServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        status_service: StatusService::new(client_notification_sender, persistence_event_sender, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use enums::weapon::WeaponType;
    use crate::server::core::map_instance::MapInstanceKey;
    use crate::server::state::character::Character;
    use crate::server::state::status::Status;
    use crate::tests::common::character_helper::{create_character, equip_item};
    use super::*;

    #[test]
    fn test_right_hand_weapon_type_is_returning_the_right_hand_weapon_when_character_has_one() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item(&mut character, "Knife");
        // When
        let weapon_type = context.status_service.right_hand_weapon_type(&character);
        // Then
        assert_eq!(weapon_type, WeaponType::Dagger);
    }
}