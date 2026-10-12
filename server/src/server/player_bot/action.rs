use movement::position::Position;

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterMovement, GameEvent};
use crate::server::model::map_instance::MapInstance;
use crate::server::model::movement::Movement;
use crate::server::model::path::path_search_client_side_algorithm;

/// Inputs come from the caller and not from `ServerState`, so the bot thread
/// can call it with its perception.
pub fn move_to(
    server: &Server,
    map_instance: &MapInstance,
    char_id: u32,
    current_position: Position,
    can_move_at: u128,
    now: u128,
    destination: Position,
) -> Result<(), String> {
    if destination.x >= map_instance.x_size() || destination.y >= map_instance.y_size() {
        return Err(format!("{},{} is outside of the map", destination.x, destination.y));
    }
    let path = path_search_client_side_algorithm(
        map_instance.x_size(),
        map_instance.y_size(),
        map_instance.state().cells().as_ref(),
        current_position.x,
        current_position.y,
        destination.x,
        destination.y,
    );
    // Same search than the client, it gives up when destination is too far or unreachable.
    if path.is_empty() {
        return Err(format!(
            "No path from {},{} to {},{}",
            current_position.x, current_position.y, destination.x, destination.y
        ));
    }
    let start_at = now.max(can_move_at);
    server.add_to_next_movement_tick(GameEvent::CharacterMove(CharacterMovement {
        char_id,
        destination,
        path: Movement::from_path(path, start_at),
        start_at,
        current_position,
        cancel_attack: true,
    }));
    Ok(())
}
