use enums::cell::CellType;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::map_instance::MapInstanceState;
use crate::enums::EnumWithMaskValueU16;

pub fn create_empty_map_instance_state() -> MapInstanceState {
    let cells: Vec<u16> = vec![CellType::Walkable.as_flag(); 100 * 100 + 1];
    MapInstanceState::new(MapInstanceKey::new("empty.gat".to_string(), 0), 100, 100, cells, Default::default(), Default::default())
}