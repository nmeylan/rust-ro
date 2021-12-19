use std::any::Any;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU16, AtomicU64};
use std::sync::atomic::Ordering::Relaxed;
use rand::Rng;
use packets::packets::PacketZcNotifyMove;
use packets::packets::Packet;
use crate::server::core::character::Character;
use crate::server::core::character_movement::Position;
use crate::server::server::{MOB_FOV_SLICE_LEN};
use crate::server::core::map::{Map, MapItem};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::status::Status;
use crate::server::enums::map_item::MapItemType;
use crate::server::server::MOB_FOV;
use crate::util::coordinate;
use std::io::Write;
use crate::util::tick::get_tick;

#[derive(Setters)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub mob_id: i16,
    pub spawn_id: u32,
    pub status: Status,
    #[set]
    pub x: AtomicU16,
    #[set]
    pub y: AtomicU16,
    pub current_map: RwLock<Arc<MapInstance>>,
    pub map_view: RwLock<Vec<Option<Arc<dyn MapItem>>>>,
    pub is_view_char: RwLock<bool>,
    pub movement_task_id: AtomicU64,
    pub self_ref: RwLock<Option<Arc<Mob>>>,
}

impl MapItem for Mob {
    fn id(&self) -> u32 {
        self.id
    }
    fn client_item_class(&self) -> i16 {
        self.mob_id
    }

    fn object_type(&self) -> i16 {
        MapItemType::Mob.value()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn x(&self) -> u16 {
        self.x.load(Relaxed)
    }

    fn y(&self) -> u16 {
        self.y.load(Relaxed)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Mob {
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: Arc<MapInstance>, status: Status) -> Mob {
        Mob {
            id,
            x: AtomicU16::new(x),
            y: AtomicU16::new(y),
            mob_id,
            spawn_id,
            status,
            name,
            map_view: RwLock::new(vec![None; MOB_FOV_SLICE_LEN]),
            current_map: RwLock::new(current_map.clone()),
            is_view_char: RwLock::new(false),
            movement_task_id: Default::default(),
            self_ref: Default::default(),
        }
    }

    pub fn set_self_ref(&self, self_ref: Arc<Mob>) {
        let mut self_ref_guard = write_lock!(self.self_ref);
        *self_ref_guard = Some(self_ref);
    }

    pub fn load_units_in_fov(&self, map_ref: &MapInstance) {
        let mut items = Vec::with_capacity(MOB_FOV_SLICE_LEN);
        let mut has_seen_char = false;
        let map_items = map_ref.get_map_items(self.x.load(Relaxed), self.y.load(Relaxed), MOB_FOV);
        for map_item in map_items {
            if map_item.as_any().downcast_ref::<Character>().is_some() {
                // info!("{{{}:{}}},{{{}:{}}} {},{}", self.get_fov_start_x(), self.get_fov_start_y(), self.get_fov_start_x()  + (MOB_FOV * 2), self.get_fov_start_y() + (MOB_FOV * 2), self.x, self.y  );
                // info!("{} {} {},{} - seen char_id {} from map view, at {},{}", self.name, self.id, self.x, self.y, map_item.id(), map_item.x(), map_item.y());
                has_seen_char = true;
                items.push(Some(map_item.clone()));
            }
        }
        let mut is_view_char_guard = write_lock!(self.is_view_char);
        let mut map_view_guard = write_lock!(self.map_view);
        *is_view_char_guard = has_seen_char;
        *map_view_guard = items;
    }

    pub fn action_move(&self) {
        let mut rng = rand::thread_rng();
        let is_view_char = read_lock!(self.is_view_char);
        let rand = rng.gen_range(0..=100);
        let should_move = if *is_view_char {
            rand <= 80
        } else {
            rand <= 10
        };

        if should_move {
            let map_guard = write_lock!(self.current_map);
            let rand_distance = rng.gen_range(2..=8);
            let current_x = self.x.load(Relaxed);
            let current_y = self.y.load(Relaxed);
            let (x, y) = Map::find_random_walkable_cell_in_max_range(&map_guard.cells, map_guard.x_size, map_guard.y_size, current_x, current_y, rand_distance);
            // Todo: implement server side movement, to avoid desync between client and server
            self.x.store(x, Relaxed);
            self.y.store(y, Relaxed);
            map_guard.remove_item_at(coordinate::get_cell_index_of(current_x, current_y, map_guard.x_size));
            let self_ref_guard = read_lock!(self.self_ref);
            map_guard.insert_item_at(coordinate::get_cell_index_of(x, y, map_guard.x_size), self_ref_guard.as_ref().unwrap().clone());
            drop(map_guard);
            if *is_view_char {
                let map_view_guard = read_lock!(self.map_view);
                let from = Position {
                    x: current_x,
                    y: current_y,
                    dir: 0
                };
                let to = Position {
                    x,
                    y,
                    dir: 0
                };
                map_view_guard.iter()
                    .filter(|map_item| map_item.as_ref().unwrap().object_type() == MapItemType::Character.value())
                    .for_each(|map_item| {
                        let character = cast!(map_item.as_ref().unwrap(), Character);
                        let map_socket_guard = write_lock!(character.map_server_socket);
                        let socket_guard = map_socket_guard.as_ref().unwrap();
                        let mut packet_zc_notify_move = PacketZcNotifyMove::default();
                        packet_zc_notify_move.set_gid(self.id);
                        packet_zc_notify_move.move_data = from.to_move_data(&to);
                        let start_time = get_tick();
                        packet_zc_notify_move.set_move_start_time(start_time);
                        packet_zc_notify_move.fill_raw();
                        socket_send!(socket_guard, packet_zc_notify_move.raw())
                    })
            }
        }
    }
}