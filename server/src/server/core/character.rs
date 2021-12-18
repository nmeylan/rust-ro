use std::any::Any;
use std::net::TcpStream;

use std::sync::{Arc, RwLock};
use packets::packets::{PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use crate::server::core::map::{MAP_EXT, MapItem};
use crate::server::core::character_movement::Position;
use crate::server::core::session::Session;
use crate::server::server::{PLAYER_FOV_SLICE_LEN, PLAYER_FOV};
use packets::packets::Packet;
use crate::util::coordinate;
use crate::util::string::StringUtil;
use std::sync::atomic::{AtomicU16, AtomicU64};
use std::sync::atomic::Ordering::{Acquire, Relaxed};
use accessor::Setters;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::mob::Mob;
use crate::server::core::status::Status;
use crate::server::enums::map_item::MapItemType;

#[derive(Setters)]
pub struct Character {
    #[set]
    pub name: String,
    pub status: Status,
    #[set]
    pub char_id: u32,
    pub current_map: RwLock<Option<Arc<MapInstance>>>,
    pub current_map_name: RwLock<[char; 16]>,
    pub x: AtomicU16,
    pub y: AtomicU16,
    pub movement_task_id: AtomicU64,
    pub map_view: RwLock<Vec<Option<Arc<dyn MapItem>>>>,
    pub self_ref: RwLock<Option<Arc<Character>>>,
    #[set]
    pub map_server_socket: RwLock<Option<Arc<RwLock<TcpStream>>>>,
}

impl MapItem for Character {
    fn id(&self) -> u32 {
        self.char_id
    }

    fn client_item_class(&self) -> i16 {
        todo!() // TODO return job id
    }

    fn object_type(&self) -> i16 {
        MapItemType::Character.value()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn x(&self) -> u16 {
        self.x.load(Acquire)
    }

    fn y(&self) -> u16 {
        self.y.load(Acquire)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Character {
    pub fn set_map_socket(&self, map_socket:  Arc<RwLock<TcpStream>>) {
        let mut map_server_socket_guard = write_lock!(self.map_server_socket);
        *map_server_socket_guard = Some(map_socket);
    }

    pub fn set_self_ref(&self, self_ref: Arc<Character>) {
        let mut self_ref_guard = write_lock!(self.self_ref);
        *self_ref_guard = Some(self_ref);
    }

    pub fn update_x_y(&self, x: u16, y: u16) {
        self.x.store(x, Relaxed);
        self.y.store(y, Relaxed);
    }

    pub fn get_x(&self)  -> u16 {
        self.x.load(Relaxed)
    }
    pub fn get_y(&self)  -> u16 {
        self.y.load(Relaxed)
    }

    pub fn set_current_map_name(&self, new_name: [char; 16]) {
        let mut current_map_name_guard = write_lock!(self.current_map_name);
        *current_map_name_guard = new_name;
    }

    pub fn get_pos_index(&self) -> usize {
        let current_map_guard = read_lock!(self.current_map);
        coordinate::get_cell_index_of(self.x(), self.y(), current_map_guard.as_ref().unwrap().x_size)
    }

    pub fn remove_from_existing_map(&self) {
        let current_map_guard = read_lock!(self.current_map);
        if current_map_guard.is_some() {
            let map_instance_ref = current_map_guard.as_ref().unwrap();
            let x_size = map_instance_ref.x_size;
            map_instance_ref.remove_item_at(coordinate::get_cell_index_of(self.x(), self.y(), x_size));
        }
    }

    pub fn join_and_set_map(&self, map_instance: Arc<MapInstance>) {
        self.set_current_map(map_instance.clone());
        let pos_index = self.get_pos_index();
        let self_ref_guard = read_lock!(self.self_ref);
        map_instance.insert_item_at(pos_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn update_position(&self, x: u16, y: u16) {
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        {
            let old_position_index = coordinate::get_cell_index_of(self.x(), self.y(), map_ref.x_size);
            map_ref.remove_item_at(old_position_index);
        }
        self.update_x_y(x, y);
        let new_position_index = coordinate::get_cell_index_of(self.x(), self.y(), map_ref.x_size);
        let self_ref_guard = read_lock!(self.self_ref);
        map_ref.insert_item_at(new_position_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn get_current_map_name(&self) -> String {
        let current_map_name_guard = read_lock!(self.current_map_name);
        current_map_name_guard.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&self, id: u64) {
        self.movement_task_id.store(id, Relaxed);
    }
    pub fn set_current_map(&self, current_map: Arc<MapInstance>) {
        let mut new_current_map: [char; 16] = [0 as char; 16];
        let map_name = format!("{}{}", current_map.name, MAP_EXT);
        map_name.fill_char_array(new_current_map.as_mut());
        self.set_current_map_name(new_current_map);
        let mut current_map_guard = write_lock!(self.current_map);
        *current_map_guard = Some(current_map);
    }

    pub fn load_units_in_fov(&self, session: &Arc<Session>) {
        let mut new_map_view = vec![None; PLAYER_FOV_SLICE_LEN]; // TODO not necesserary to init such a big vec
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        let mut map_view_guard = write_lock!(self.map_view);
        let mut previous_item_ids: Vec<u32> = vec![];
        let mut seen_items_ids: Vec<u32> = vec![];

        for item in map_view_guard.iter() {
            if item.is_some() {
                let item = item.as_ref().unwrap();
                previous_item_ids.push(item.id());
            }
        }

        let map_items = map_ref.get_map_items(self.x(), self.y(), PLAYER_FOV);
        for map_item in map_items {
            if map_item.object_type() == MapItemType::Character.value() {
                continue;
            }
            // info!("{{{}:{}}},{{{}:{}}} {},{}", self.get_fov_start_x(), self.get_fov_start_y(), self.get_fov_start_x()  + (PLAYER_FOV * 2), self.get_fov_start_y() + (PLAYER_FOV * 2), self.x(), self.y()  );
            // info!("See map_item at {},{} index {}, id {} (inner {},{} - index {})", map_item.x(), map_item.y(),  coordinate::get_cell_index_of(map_item.x(), map_item.y(), 400), map_item.id(),  x, y, coordinate::get_cell_index_of(x, y, 400));
            new_map_view.push(Some(map_item.clone()));
            seen_items_ids.push(map_item.id());
            if !previous_item_ids.contains(&map_item.id()) {
                let mut name = [0 as char; 24];
                map_item.name().fill_char_array(name.as_mut());
                let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                packet_zc_notify_standentry.set_job(map_item.client_item_class());
                packet_zc_notify_standentry.set_packet_length(108);
                packet_zc_notify_standentry.set_name(name);
                packet_zc_notify_standentry.set_pos_dir(Position { x: map_item.x(), y: map_item.y(), dir: 3 }.to_pos());
                packet_zc_notify_standentry.set_objecttype(map_item.object_type() as u8);
                packet_zc_notify_standentry.set_aid(map_item.id());
                if map_item.object_type() == MapItemType::Mob.value() {
                    let mob = cast!(map_item, Mob);
                    packet_zc_notify_standentry.set_clevel(3);
                    packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                    packet_zc_notify_standentry.set_hp(mob.status.hp);
                    packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                }
                packet_zc_notify_standentry.fill_raw();
                session.send_to_map_socket(packet_zc_notify_standentry.raw());
            }
        }

        *map_view_guard = new_map_view;
        for id in previous_item_ids {
            if !seen_items_ids.contains(&id) {
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(id);
                packet_zc_notify_vanish.fill_raw();
                session.send_to_map_socket(packet_zc_notify_vanish.raw());
            }
        }
    }
}