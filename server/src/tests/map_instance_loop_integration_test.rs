
#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod tests {
    use tokio::runtime::Runtime;
    use crate::tests::common::integration_test::{before_all, character_join_game};

    #[tokio::test]
    async fn test_server1() {
        // Given
        let server = before_all().await;
        // When
        server.state();
        println!("test1");
        character_join_game().await;
        // Then
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