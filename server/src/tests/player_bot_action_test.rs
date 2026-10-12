#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::sync::Arc;

    use movement::path::get_cell_index_of;
    use movement::position::Position;

    use crate::server::model::events::game_event::GameEvent;
    use crate::server::model::map_instance::MapInstance;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::player_bot::action;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::server_service_test::{ServerServiceTestContext, before_each};

    const CHAR_ID: u32 = 150000;
    const NOW: u128 = 1_000_000;

    fn before_each_with_map() -> (ServerServiceTestContext, MapInstance) {
        let context = before_each();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        (context, map_instance)
    }

    fn position(x: u16, y: u16) -> Position {
        Position { x, y, dir: 0 }
    }

    #[test]
    fn move_to_should_schedule_character_move_on_movement_queue() {
        // Given
        let (context, map_instance) = before_each_with_map();
        // When
        let result = action::move_to(&context.server, &map_instance, CHAR_ID, position(10, 10), 0, NOW, position(15, 12));
        // Then
        assert_eq!(result, Ok(()));
        let events = context.server.pop_movement_task().unwrap();
        assert_eq!(events.len(), 1);
        let GameEvent::CharacterMove(movement) = &events[0] else {
            panic!("Expected CharacterMove but got {:?}", events[0]);
        };
        assert_eq!(movement.char_id, CHAR_ID);
        assert_eq!(movement.current_position, position(10, 10));
        assert_eq!(movement.destination, position(15, 12));
        assert_eq!(movement.start_at, NOW);
        assert!(movement.cancel_attack);
        // Path is stored from destination to first step.
        let last_step = movement.path.first().unwrap().position();
        assert_eq!((last_step.x, last_step.y), (15, 12));
    }

    #[test]
    fn move_to_should_start_when_character_can_move() {
        // Given
        let (context, map_instance) = before_each_with_map();
        let can_move_at = NOW + 500;
        // When
        action::move_to(&context.server, &map_instance, CHAR_ID, position(10, 10), can_move_at, NOW, position(15, 12)).unwrap();
        // Then
        let events = context.server.pop_movement_task().unwrap();
        let GameEvent::CharacterMove(movement) = &events[0] else {
            panic!("Expected CharacterMove but got {:?}", events[0]);
        };
        assert_eq!(movement.start_at, can_move_at);
    }

    #[test]
    fn move_to_should_fail_when_destination_is_outside_of_map() {
        // Given
        let (context, map_instance) = before_each_with_map();
        // When
        let result = action::move_to(&context.server, &map_instance, CHAR_ID, position(10, 10), 0, NOW, position(100, 12));
        // Then
        assert_eq!(result, Err("100,12 is outside of the map".to_string()));
        assert!(context.server.pop_movement_task().is_none());
    }

    #[test]
    fn move_to_should_fail_when_destination_is_not_walkable() {
        // Given
        let (context, map_instance) = before_each_with_map();
        let x_size = map_instance.x_size();
        map_instance.state_mut().cells_mut()[get_cell_index_of(15, 12, x_size)] = 0;
        // When
        let result = action::move_to(&context.server, &map_instance, CHAR_ID, position(10, 10), 0, NOW, position(15, 12));
        // Then
        assert_eq!(result, Err("No path from 10,10 to 15,12".to_string()));
        assert!(context.server.pop_movement_task().is_none());
    }
}
