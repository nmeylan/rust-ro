use std::any::Any;
use std::cmp;
use std::collections::HashMap;
use std::fs::read;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
use packets::packets::{PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use crate::server::core::map::{MapItem};
use crate::server::core::movement::Position;
use crate::server::core::session::Session;
use crate::server::server::PLAYER_FOV;
use packets::packets::Packet;
use crate::util::coordinate;
use crate::util::string::StringUtil;
use std::io::Write;
use accessor::Setters;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::status::Status;

#[derive(Setters)]
pub struct CharacterSession {
    #[set]
    pub name: String,
    pub status: Status,
    #[set]
    pub char_id: u32,
    pub current_map: RwLock<Option<Arc<RwLock<MapInstance>>>>,
    pub current_map_name: RwLock<[char; 16]>,
    pub current_position: RwLock<Position>,
    pub movement_task_id: RwLock<Option<u128>>,
    pub map_view: RwLock<HashMap<usize, Arc<dyn MapItem>>>,
    pub self_ref: RwLock<Option<Arc<CharacterSession>>>,
}

impl MapItem for CharacterSession {
    fn id(&self) -> u32 {
        self.char_id
    }

    fn client_item_class(&self) -> i16 {
        todo!() // TODO return job id
    }

    fn object_type(&self) -> i16 {
        0
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn x(&self) -> u16 {
        let current_position_guard = read_lock!(self.current_position);
        current_position_guard.x
    }

    fn y(&self) -> u16 {
        let current_position_guard = read_lock!(self.current_position);
        current_position_guard.y
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
}

impl CharacterSession {
    pub fn set_self_ref(&self, self_ref: Arc<CharacterSession>) {
        let mut self_ref_guard = write_lock!(self.self_ref);
        *self_ref_guard = Some(self_ref);
    }
    pub fn update_x_y(&self, x: u16, y: u16) {
        let mut current_position_guard= write_lock!(self.current_position);
        current_position_guard.x = x;
        current_position_guard.y = y;
    }
    pub fn set_current_map_name(&self, new_name: [char; 16]) {
        let mut current_map_name_guard= write_lock!(self.current_map_name);
        *current_map_name_guard = new_name;
    }


    pub fn get_pos_index(&self) -> usize {
        let current_map_guard = read_lock!(self.current_map);
        let map_guard = read_lock!(current_map_guard.as_ref().unwrap());
        let current_position_guard = read_lock!(self.current_position);
        coordinate::get_cell_index_of(current_position_guard.y, current_position_guard.y, map_guard.x_size)
    }

    pub fn change_map(&self, map_instance: Arc<RwLock<MapInstance>>) {
        let mut map_view_guard = write_lock!(self.map_view);
        map_view_guard.clear(); // TODO reset map_view of MapItem present in this map view
        self.remove_from_existing_map();
        self.join_and_set_map(map_instance);
    }

    fn remove_from_existing_map(&self) {
        let current_map_guard = read_lock!(self.current_map);
        if current_map_guard.is_some() {
            let map_instance_ref = current_map_guard.as_ref().unwrap();
            let mut map_instance_ref_guard = write_lock!(map_instance_ref);
            let x_size = map_instance_ref_guard.x_size;
            let current_position_guard = read_lock!(self.current_position);
            map_instance_ref_guard.remove_char_id_from_map(coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, x_size));
        }
    }

    fn join_and_set_map(&self, map_instance: Arc<RwLock<MapInstance>>) {
        self.set_current_map(map_instance.clone());
        let pos_index = self.get_pos_index();
        let mut map_instance_guard = write_lock!(map_instance);
        let self_ref_guard = read_lock!(self.self_ref);
        map_instance_guard.add_char_to_map(pos_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn update_position(&self, x:u16, y: u16) {
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        let mut map = write_lock!(map_ref);
        {
            let current_position_guard = read_lock!(self.current_position);
            let old_position_index = coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, map.x_size);
            map.map_items.remove(old_position_index);
        }
        self.update_x_y(x, y);
        let current_position_guard = read_lock!(self.current_position);
        let new_position_index = coordinate::get_cell_index_of(current_position_guard.x, current_position_guard.y, map.x_size);
        let self_ref_guard = read_lock!(self.self_ref);
        map.map_items.insert(new_position_index, self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn get_current_map_name(&self) -> String {
        let current_map_name_guard = read_lock!(self.current_map_name);
        current_map_name_guard.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&self, id: u128) {
        let mut movement_task_id_guard = write_lock!(self.movement_task_id);
        *movement_task_id_guard = Some(id);
    }
    pub fn set_current_map(&self, current_map: Arc<RwLock<MapInstance>>) {
        let mut current_map_guard = write_lock!(self.current_map);
        *current_map_guard = Some(current_map);
    }

    // pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<&Arc<dyn MapItem>> {
    //     let map_view_guard = read_lock!(self.map_view);
    //     let i = coordinate::get_cell_index_of(x, y, PLAYER_FOV);
    //     map_view_guard.get(&i)
    // }

    pub fn set_map_item_at(&self, x: u16, y: u16, item: Arc<dyn MapItem>) {
        let mut map_view_guard = write_lock!(self.map_view);
        let i = coordinate::get_cell_index_of(x, y, PLAYER_FOV);
        map_view_guard.insert(i, item);
    }

    // TODO try to optimize, method below take ~0.5ms to execute (peak at 1.5ms)
    #[elapsed]
    pub fn load_units_in_fov(&self, session_guard: &RwLockReadGuard<Session>) {
        let mut new_map_view = HashMap::<usize, Arc<dyn MapItem>>::new();
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        let map = read_lock!(map_ref);
        let mut start_x;
        let mut end_x;
        let mut start_y;
        let mut end_y;
        {
            let mut current_position_guard= read_lock!(self.current_position);
            start_x = cmp::max(current_position_guard.x - PLAYER_FOV, 0);
            end_x = cmp::min(current_position_guard.x + PLAYER_FOV, map.x_size);
            start_y = cmp::max(current_position_guard.y - PLAYER_FOV, 0);
            end_y = cmp::min(current_position_guard.y + PLAYER_FOV, map.y_size);
        }
        for x in start_x..end_x {
            for y in start_y..end_y {
                if map.is_warp_cell(x, y) {
                    let warp = map.get_warp_at(x, y).unwrap();
                    if coordinate::get_item_at(warp.x, warp.y, PLAYER_FOV, &new_map_view).is_none() {
                        let mut warp_name = [0 as char; 24];
                        warp.name.fill_char_array(warp_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(warp.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(6);
                        packet_zc_notify_standentry.set_aid(warp.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x: warp.x, y: warp.y, dir: 0 }.to_pos());
                        packet_zc_notify_standentry.set_name(warp_name);
                        packet_zc_notify_standentry.fill_raw();

                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                    new_map_view.insert(coordinate::get_cell_index_of(x, y, PLAYER_FOV), warp as Arc<dyn MapItem>);
                }
                let map_item = map.get_map_item_at(x, y);
                if map_item.is_some() {
                    let map_item = map_item.unwrap();
                    if map_item.object_type() != 5 {
                        continue;
                    }
                    if coordinate::get_item_at(map_item.x(), map_item.y(), PLAYER_FOV, &new_map_view).is_none() {
                        info!("Seeing: {} at {},{}",map_item.name(), map_item.x(), map_item.y());
                        let mut mob_name = [0 as char; 24];
                        map_item.name().fill_char_array(mob_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(map_item.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(map_item.object_type() as u8);
                        packet_zc_notify_standentry.set_clevel(3);
                        packet_zc_notify_standentry.set_aid(map_item.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x: map_item.x(), y: map_item.y(), dir: 3 }.to_pos());
                        packet_zc_notify_standentry.set_name(mob_name);
                        packet_zc_notify_standentry.fill_raw();
                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                    new_map_view.insert(coordinate::get_cell_index_of(x, y, PLAYER_FOV), map_item);
                }
            }
        }
        let mut map_view_guard = write_lock!(self.map_view);
        let old_items = map_view_guard.keys().map(|k| *k).collect::<Vec<usize>>();
        *map_view_guard = new_map_view;
        // for item in old_items {
        //     if !self.map_view.contains_key(&item) {
        //         let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
        //         packet_zc_notify_vanish.set_gid(old_map_view.get(&item).unwrap().id());
        //         packet_zc_notify_vanish.fill_raw();
        //         let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
        //         socket_send!(tcp_stream, packet_zc_notify_vanish.raw());
        //     }
        // }
    }
}