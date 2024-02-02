
#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod tests {
    
    use std::time::Duration;
    use crate::server::model::events::map_event::MapEvent;
    use crate::tests::common::integration_test::{before_all, character_join_game};

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
}