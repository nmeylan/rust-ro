use crate::server::model::map_item::MapItems;
use crate::server::state::server::ServerState;

pub fn create_empty_server_state() -> ServerState {
    ServerState::new(MapItems::new(0))
}
