#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::sync::Arc;

    use movement::position::Position;

    use crate::server::model::events::game_event::{CharacterChangeMap, CharacterRemoveFromMap, GameEvent};
    use crate::server::model::session::Session;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::player_bot::lifecycle;
    use crate::tests::common::character_helper::create_character;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::mocked_repository::{MOCKED_ACCOUNT_ID, MOCKED_CHAR_ID, MOCKED_UNKNOWN_CHAR_NAME};
    use crate::tests::server_service_test::{ServerServiceTestContext, before_each};

    fn before_each_with_empty_map() -> ServerServiceTestContext {
        let context = before_each();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        context
            .server
            .state_mut()
            .map_instances_mut()
            .entry("empty".to_string())
            .or_default()
            .push(Arc::new(map_instance));
        context
    }

    fn drain_events_by_tick(task_queue: &TasksQueue<GameEvent>) -> Vec<Vec<GameEvent>> {
        let mut events_by_tick = vec![];
        while let Some(events) = task_queue.pop() {
            events_by_tick.push(events);
        }
        events_by_tick
    }

    #[test]
    fn spawn_should_insert_character_flagged_as_player_bot() {
        // Given
        let context = before_each_with_empty_map();
        // When
        let result = lifecycle::spawn(&context.server, "bot1");
        // Then
        assert_eq!(result, Ok(MOCKED_CHAR_ID));
        let character = context.server.state().get_character(MOCKED_CHAR_ID).unwrap();
        assert!(character.is_player_bot);
        assert_eq!(character.name, "bot1");
        assert_eq!(character.account_id, MOCKED_ACCOUNT_ID);
    }

    #[test]
    fn spawn_should_fail_when_player_bots_are_disabled() {
        // Given
        let mut context = before_each_with_empty_map();
        let mut config = context.server.configuration.clone();
        config.player_bot.enabled = false;
        context.server.configuration = Box::leak(Box::new(config));
        // When
        let result = lifecycle::spawn(&context.server, "bot1");
        // Then
        assert!(result.is_err());
        assert!(context.server.state().get_character(MOCKED_CHAR_ID).is_none());
        assert!(context.server_task_queue.pop().is_none());
    }

    #[test]
    fn spawn_should_create_session_without_socket() {
        // Given
        let context = before_each_with_empty_map();
        // When
        lifecycle::spawn(&context.server, "bot1").unwrap();
        // Then
        let session = context.server.state().get_session(MOCKED_ACCOUNT_ID);
        assert!(session.is_player_bot);
        assert_eq!(session.char_id, Some(MOCKED_CHAR_ID));
        assert!(session.map_server_socket.is_none());
        assert!(session.char_server_socket.is_none());
    }

    #[test]
    fn spawn_should_schedule_join_game_and_warp_to_last_position() {
        // Given
        let context = before_each_with_empty_map();
        // When
        lifecycle::spawn(&context.server, "bot1").unwrap();
        // Then
        let events: Vec<GameEvent> = drain_events_by_tick(&context.server_task_queue).into_iter().flatten().collect();
        assert!(events.contains(&GameEvent::CharacterJoinGame(MOCKED_CHAR_ID)));
        assert!(events.contains(&GameEvent::CharacterChangeMap(CharacterChangeMap {
            char_id: MOCKED_CHAR_ID,
            new_map_name: "empty".to_string(),
            new_instance_id: 0,
            new_position: Some(Position { x: 10, y: 12, dir: 3 }),
        })));
    }

    #[test]
    fn spawn_should_fail_when_character_does_not_exist() {
        // Given
        let context = before_each_with_empty_map();
        // When
        let result = lifecycle::spawn(&context.server, MOCKED_UNKNOWN_CHAR_NAME);
        // Then
        assert_eq!(result, Err(format!("Character {MOCKED_UNKNOWN_CHAR_NAME} not found")));
    }

    #[test]
    fn spawn_should_fail_when_character_is_already_in_game() {
        // Given
        let context = before_each_with_empty_map();
        lifecycle::spawn(&context.server, "bot1").unwrap();
        // When
        let result = lifecycle::spawn(&context.server, "bot1");
        // Then
        assert_eq!(result, Err("Character bot1 is already in game".to_string()));
    }

    #[test]
    fn spawn_should_fail_when_account_is_used_by_a_connected_player() {
        // Given
        let context = before_each_with_empty_map();
        context
            .server
            .state()
            .add_session(MOCKED_ACCOUNT_ID, Arc::new(Session::create_empty(MOCKED_ACCOUNT_ID, 0, 0, 0)));
        // When
        let result = lifecycle::spawn(&context.server, "bot1");
        // Then
        assert_eq!(
            result,
            Err(format!("Account {MOCKED_ACCOUNT_ID} of character bot1 is already connected"))
        );
        assert!(context.server.state().get_character(MOCKED_CHAR_ID).is_none());
        assert!(!context.server.state().get_session(MOCKED_ACCOUNT_ID).is_player_bot);
    }

    #[test]
    fn despawn_should_remove_from_map_then_leave_game_and_remove_session() {
        // Given
        let context = before_each_with_empty_map();
        lifecycle::spawn(&context.server, "bot1").unwrap();
        drain_events_by_tick(&context.server_task_queue);
        // When
        let result = lifecycle::despawn(&context.server, "bot1");
        // Then
        assert_eq!(result, Ok(MOCKED_CHAR_ID));
        let events_by_tick = drain_events_by_tick(&context.server_task_queue);
        assert_eq!(events_by_tick[1], vec![GameEvent::CharacterRemoveFromMap(
            CharacterRemoveFromMap {
                char_id: MOCKED_CHAR_ID,
                map_name: "empty.gat".to_string(),
                instance_id: 0,
            }
        )]);
        assert_eq!(events_by_tick[2], vec![GameEvent::CharacterLeaveGame((MOCKED_CHAR_ID, 0))]);
        assert!(!read_lock!(context.server.state().sessions()).contains_key(&MOCKED_ACCOUNT_ID));
    }

    #[test]
    fn despawn_should_ignore_character_which_is_not_a_player_bot() {
        // Given
        let context = before_each_with_empty_map();
        let mut character = create_character();
        character.name = "player1".to_string();
        let char_id = character.char_id;
        context.server.state_mut().insert_character(character);
        // When
        let result = lifecycle::despawn(&context.server, "player1");
        // Then
        assert_eq!(result, Err("No player bot named player1 in game".to_string()));
        assert!(context.server.state().get_character(char_id).is_some());
    }

    #[test]
    fn player_bots_should_only_list_player_bots() {
        // Given
        let context = before_each_with_empty_map();
        context.server.state_mut().insert_character(create_character());
        lifecycle::spawn(&context.server, "bot1").unwrap();
        // When
        let player_bots = lifecycle::player_bots(&context.server);
        // Then
        assert_eq!(player_bots.len(), 1);
        assert_eq!(player_bots[0].char_id, MOCKED_CHAR_ID);
    }
}
