use crate::server::state::server::ServerState;

pub fn create_empty_server_state() -> ServerState {
    ServerState::new(Default::default())
}