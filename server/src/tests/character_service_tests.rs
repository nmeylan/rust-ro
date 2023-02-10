use std::sync::Arc;
use crate::repository::CharacterRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};

struct CharacterServiceTestContext {
    test_context: TestContext,
    character_service: CharacterService,
}

fn before_each(character_repository: Arc<dyn CharacterRepository>) -> CharacterServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    CharacterServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        character_service: CharacterService::new(client_notification_sender, persistence_event_sender, character_repository, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::character_service_tests::before_each;
    use crate::tests::common::mocked_repository;

    #[test]
    fn test_max_weight() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_check_weight() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_map_should_clear_movement() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_map_should_update_position() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_map_should_defer_position_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_map_should_update_map() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_look_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_look_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_sprite_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_zeny_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_zeny_should_fetch_zeny_when_zeny_amount_is_not_specified() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_zeny_should_update_zeny_in_memory() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_notify_weight_should_notify_character() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_base_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_base_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_base_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_job_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_job_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_update_job_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_job_should_defer_db_update() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_change_job_should_notify_sprite_change() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_load_units_in_fov_should_add_new_item_in_character_map_view() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
    
    #[test]
    fn test_load_units_in_fov_should_remove_out_of_fov_item_from_character_map_view() {
        // Given
        let context = before_each(mocked_repository());
        
        // When
        
        // Then
        
    }
}