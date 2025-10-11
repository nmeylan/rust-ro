use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map::Map;
use crate::server::model::map_instance::{MapInstance, MapInstanceKey};
use crate::server::model::map_item::MapItems;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::state::map_instance::MapInstanceState;

static mut EMPTY_MAP: Option<Map> = None;
static INIT_EMPTY_MAP: Once = Once::new();
pub fn create_empty_map_instance_state() -> MapInstanceState {
    let cells: Vec<u16> = vec![CellType::Walkable.as_flag(); 100 * 100 + 1];
    MapInstanceState::new(
        MapInstanceKey::new("empty.gat".to_string(), 0),
        100,
        100,
        cells,
        MapItems::new(0),
        Default::default(),
    )
}

pub fn create_empty_map() -> &'static Map {
    INIT_EMPTY_MAP.call_once(|| unsafe {
        EMPTY_MAP = Some(Map::new(
            100,
            100,
            100 * 100,
            "empty".to_string(),
            "empty.gat".to_string(),
            Default::default(),
            Default::default(),
            Default::default(),
        ));
    });
    unsafe { EMPTY_MAP.as_ref().unwrap() }
}
pub fn create_empty_map_instance(
    client_notification_channel: SyncSender<Notification>,
    task_queue: Arc<TasksQueue<MapEvent>>,
) -> MapInstance {
    let cells: Vec<u16> = vec![CellType::Walkable.as_flag(); 100 * 100 + 1];
    MapInstance::from_map(
        Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value())),
        create_empty_map(),
        0,
        cells,
        client_notification_channel,
        MapItems::new(0),
        task_queue,
    )
}
