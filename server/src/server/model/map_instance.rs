#![allow(dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::server::model::map::{Map, MAP_EXT};
use crate::server::model::map_item::{MapItems, ToMapItem};
use std::sync::mpsc::SyncSender;

use rathena_script_lang_interpreter::lang::vm::Vm;


use crate::server::model::events::map_event::MapEvent;
use crate::server::model::events::client_notification::{Notification};

use crate::server::model::tasks_queue::TasksQueue;
use crate::server::map_instance_loop::MAP_LOOP_TICK_RATE;

use crate::server::model::script::Script;
use crate::server::model::warp::Warp;

use crate::server::script::MapScriptHandler;
use crate::server::state::map_instance::{MapInstanceState, MobSpawnTrack};
use crate::util::cell::{MyRef, MyRefMut, MyUnsafeCell};

use crate::util::string::StringUtil;
use crate::util::tick::delayed_tick;


#[derive(Clone, Debug, PartialEq)]
pub struct MapInstanceKey {
    instance_id: u8,
    map_name: [char; 16],
    map_name_string: String,
}

impl MapInstanceKey {
    pub fn map_name(&self) -> &String {
        &self.map_name_string
    }

    pub fn map_name_char(&self) -> [char; 16] {
        self.map_name
    }
    pub fn map_instance(&self) -> u8 {
        self.instance_id
    }

    pub fn map_without_ext(&self) -> String {
        let map_name_without_ext: String = self.map_name_string.clone();
        if map_name_without_ext.ends_with(MAP_EXT) {
            return map_name_without_ext[0..map_name_without_ext.len()-4].to_string();
        }
        map_name_without_ext
    }

    pub fn new(map_name: String, id: u8) -> Self {
        let mut new_current_map: [char; 16] = [0 as char; 16];
        let map_name = if !map_name.ends_with(MAP_EXT) {
            format!("{map_name}{MAP_EXT}")
        } else {
            map_name
        };
        map_name.fill_char_array(new_current_map.as_mut());
        Self {
            map_name: new_current_map,
            map_name_string: map_name,
            instance_id: id
        }
    }
}

pub struct MapInstance {
    key: MapInstanceKey,
    client_notification_channel: SyncSender<Notification>,
    tasks_queue: Arc<TasksQueue<MapEvent>>,
    map: &'static Map,
    scripts: Vec<Arc<Script>>,
    state: MyUnsafeCell<MapInstanceState>,
    shutdown: AtomicBool,
}
unsafe impl Sync for MapInstance {}
unsafe impl Send for MapInstance {}


impl MapInstance {
    pub fn from_map(vm: Arc<Vm>, map: &'static Map, id: u8, cells: Vec<u16>, client_notification_channel: SyncSender<Notification>, mut map_items: MapItems, tasks_queue: Arc<TasksQueue<MapEvent>>) -> MapInstance {
        let mut scripts = vec![];
        map.scripts().iter().for_each(|script| {
            let (_, instance_reference) = Vm::create_instance(vm.clone(), script.class_name.clone(), Box::new(&MapScriptHandler), script.constructor_args.clone()).unwrap();
            let mut script = script.clone();
            script.set_instance_reference(instance_reference);
            let script_arc = Arc::new(script);
            map_items.insert(script_arc.id(), script_arc.to_map_item());
            scripts.push(script_arc);
        });
        let key = MapInstanceKey::new(map.name().to_string(), id);
        MapInstance {
            key: key.clone(),
            client_notification_channel,
            tasks_queue,
            map,
            scripts,
            state: MyUnsafeCell::new(MapInstanceState::new(key, map.x_size(), map.y_size(), cells, map_items,
                                         map.mob_spawns().iter().map(|spawn| (spawn.id, MobSpawnTrack::default(spawn.id))).collect::<HashMap<u32, MobSpawnTrack>>())),
            shutdown: AtomicBool::new(false),
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }

    pub fn is_alive(&self) -> bool {
        !self.shutdown.load(Ordering::Relaxed)
    }

    pub(crate) fn pop_task(&self) -> Option<Vec<MapEvent>> {
        self.tasks_queue.pop()
    }

    pub fn add_to_next_tick(&self, event: MapEvent) {
        self.tasks_queue.add_to_first_index(event)
    }

    pub fn add_to_tick(&self, event: MapEvent, index: usize) {
        self.tasks_queue.add_to_index(event, index)
    }

    pub fn add_to_delayed_tick(&self, event: MapEvent, delay: u128) {
        self.add_to_tick(event, delayed_tick(delay, MAP_LOOP_TICK_RATE));
    }

    pub fn task_queue(&self) -> Arc<TasksQueue<MapEvent>> {
        self.tasks_queue.clone()
    }

    pub fn id(&self) -> u8{
        self.key().instance_id
    }

    pub fn get_warp_at(&self, x: u16, y: u16) -> Option<Warp> {
        for warp in self.map().warps().iter() {
            if x >= warp.x - warp.x_size && x <= warp.x + warp.x_size
                && y >= warp.y - warp.y_size && y <= warp.y + warp.y_size {
                return Some(warp.clone());
            }
        }
        None
    }

    pub fn get_warp(&self, warp_id: u32) -> Option<Warp> {
        for warp in self.map().warps().iter() {
            if warp.id == warp_id {
                return Some(warp.clone())
            }
        }
        None
    }

    pub fn get_script(&self, script_id: u32) -> Option<Arc<Script>> {
        for script in self.scripts.iter() {
            if script.id() == script_id {
                return Some(script.clone())
            }
        }
        None
    }

    #[inline]
    pub fn get_fov_start_x(&self, x: u16, range: u16) -> u16 {
        if range > x {
            return 0
        }
        x - range
    }

    #[inline]
    pub fn get_fov_start_y(&self, y: u16, range: u16) -> u16 {
        if range > y {
            return 0
        }
        y - range
    }

    #[inline]
    pub fn get_item_x_from_fov(&self, x: u16, range: u16, i: u16) -> u16 {
        let x = self.get_fov_start_x(x, range) + i;
        if x >= self.map().x_size() {
            return self.map().x_size() - 1;
        }
        x
    }

    #[inline]
    pub fn get_item_y_from_fov(&self, y: u16, range: u16, j: u16) -> u16 {
        let y = self.get_fov_start_y(y, range) + j;
        if y >= self.map().y_size() {
            return self.map().y_size() - 1;
        }
        y
    }
    pub fn key(&self) -> &MapInstanceKey {
        &self.key
    }
    pub fn client_notification_channel(&self) -> &SyncSender<Notification> {
        &self.client_notification_channel
    }
    pub fn map(&self) -> &Map {
        self.map
    }
    pub fn name(&self) -> &str {
        self.map().name()
    }
    pub fn name_with_ext(&self) -> &str {
        self.map().name_with_ext()
    }
    pub fn x_size(&self) -> u16 {
        self.map().x_size()
    }
    pub fn y_size(&self) -> u16 {
        self.map().y_size()
    }

    pub fn state(&self) -> MyRef<MapInstanceState> {
        self.state.borrow()
    }

    pub fn state_mut(&self) -> MyRefMut<MapInstanceState> {
        self.state.borrow_mut()
    }
}