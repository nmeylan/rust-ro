use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::map_instance::MapInstanceState;

pub fn create_empty_map_instance_state() -> MapInstanceState {
    MapInstanceState::new(MapInstanceKey::new("empty.gat".to_string(), 0), 100, 100, Default::default(), Default::default(), Default::default())
}