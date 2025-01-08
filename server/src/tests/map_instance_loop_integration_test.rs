
#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod tests {
    use crate::server::model::events::game_event::{CharacterChangeMap, GameEvent};
    use crate::server::model::events::map_event::MapEvent;
    use crate::tests::common::integration_test::{before_all, character_join_game};
    use models::position::Position;
    use std::time::Duration;

    #[tokio::test]
    async fn concurrency_testKillMonsterWhileMoving() {
        // Given
        let server = before_all().await;
        // When
        server.state();
        println!("test1");
        let char_id = character_join_game().await;
        let character = server.state().get_character_unsafe(char_id);
        let map_instance = server.state().get_map_instance_from_character(character).clone().unwrap();
        for _ in 0..60 {
            map_instance.add_to_delayed_tick(MapEvent::AdminKillAllMobs(char_id), 26);
            // Then
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        println!("test1 done");
    }

    #[tokio::test]
    async fn test_server2() {
        // Given
        let server = before_all().await;
        // When
        server.state();
        character_join_game().await;
        println!("test2");
        // Then
    }

    #[tokio::test]
    async fn test_changemap() {
        // Given
        let server = before_all().await;
        // When
        let char_id = character_join_game().await;
        let mut server_mut = server.state_mut();
        let mut character = server_mut.characters_mut().get_mut(&char_id).unwrap();
        for _ in 0..60 {
           server.add_to_next_tick(GameEvent::CharacterChangeMap(CharacterChangeMap{
               char_id,
               new_map_name: "prt_fild09".to_string(),
               new_instance_id: 0,
               new_position: Some(Position{x: 100, y: 100, dir: 0 }),
           }));
           tokio::time::sleep(Duration::from_millis(30)).await;
        }
        // Then
    }
}