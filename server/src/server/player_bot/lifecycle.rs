use std::sync::Arc;

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterRemoveFromMap, GameEvent};
use crate::server::model::map::Map;
use crate::server::model::session::Session;
use crate::server::request_handler::char::load_character;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;

pub struct SpawnLocation {
    pub map_name: String,
    pub x: u16,
    pub y: u16,
}

/// Put a saved character in game as a player bot, following the same steps
/// than a client selecting a character then entering the map.
/// Without `location` the bot spawns at its last saved position.
pub fn spawn(server: &Server, char_name: &str, location: Option<SpawnLocation>) -> Result<u32, String> {
    if !server.configuration.player_bot.enabled {
        return Err("Player bots are disabled, set player_bot.enabled in config.json".to_string());
    }

    let char_model = server
        .runtime()
        .block_on(async { server.repository.character_with_name_fetch(char_name).await })
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => format!("Character {char_name} not found"),
            _ => {
                error!("Failed to load character {}: {:?}", char_name, error);
                format!("Failed to load character {char_name}")
            }
        })?;

    let char_id = char_model.char_id as u32;
    let account_id = char_model.account_id as u32;

    if server.state().get_character(char_id).is_some() {
        return Err(format!("Character {char_name} is already in game"));
    }

    // Sessions are indexed by account id, a bot would replace the session of a
    // connected player.
    if read_lock!(server.state().sessions()).contains_key(&account_id) {
        return Err(format!("Account {account_id} of character {char_name} is already connected"));
    }

    let mut character = load_character(server, &char_model, account_id);
    character.is_player_bot = true;
    let location = location.unwrap_or_else(|| SpawnLocation {
        map_name: character.current_map_name().clone(),
        x: character.x(),
        y: character.y(),
    });
    server.state_mut().insert_character(character);
    server.state().add_session(
        account_id,
        Arc::new(Session::create_for_player_bot(
            account_id,
            char_id,
            GlobalConfigService::instance().packetver(),
        )),
    );

    server.add_to_next_tick(GameEvent::CharacterJoinGame(char_id));
    server
        .server_service()
        .schedule_warp_to_walkable_cell(
            server.state_mut().as_mut(),
            &Map::name_without_ext(&location.map_name),
            location.x,
            location.y,
            char_id,
        );
    Ok(char_id)
}

/// Remove a player bot from the game, its state is saved like for a player
/// leaving the game.
pub fn despawn(server: &Server, char_name: &str) -> Result<u32, String> {
    let character = player_bots(server)
        .into_iter()
        .find(|character| character.name == char_name)
        .ok_or_else(|| format!("No player bot named {char_name} in game"))?;

    let char_id = character.char_id;

    server.add_to_tick(
        GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap {
            char_id,
            map_name: character.current_map_name().clone(),
            instance_id: character.current_map_instance(),
        }),
        1,
    );

    server.add_to_tick(GameEvent::CharacterLeaveGame((char_id, 0)), 2);
    server.state().remove_session(character.account_id);
    Ok(char_id)
}

pub fn player_bots(server: &Server) -> Vec<&Character> {
    server
        .state()
        .characters()
        .values()
        .filter(|character| character.is_player_bot)
        .collect()
}
