use std::cmp;
use std::collections::HashMap;
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
    #[set]
    pub current_map: Option<Arc<RwLock<MapInstance>>>,
    #[set]
    pub current_map_name: [char; 16],
    #[set]
    pub current_position: Position,
    pub movement_task_id: Option<u128>,
    pub map_view: HashMap<usize, Arc<dyn MapItem>>
}

impl MapItem for Mutex<CharacterSession> {
    fn id(&self) -> u32 {
        let self_guard = mutex_lock!(self);
        self_guard.char_id
    }

    fn client_item_class(&self) -> i16 {
        todo!() // TODO return job id
    }

    fn object_type(&self) -> i16 {
        0
    }

    fn name(&self) -> String {
        let self_guard = mutex_lock!(self);
        self_guard.name.clone()
    }
}

impl CharacterSession {
    pub fn set_current_x(&mut self, current_x: u16) {
        self.current_position.x = current_x;
    }
    pub fn set_current_y(&mut self, current_y: u16) {
        self.current_position.y = current_y;
    }

    pub fn get_pos_index(&self) -> usize {
        let map = read_lock!(self.current_map.as_ref().unwrap());
        coordinate::get_cell_index_of(self.current_position.y, self.current_position.y, map.x_size)
    }

    pub fn change_map(&mut self, map_instance: Arc<RwLock<MapInstance>>) {
        self.map_view.clear(); // TODO reset map_view of MapItem present in this map view
        self.remove_from_existing_map();
        self.join_and_set_map(map_instance);
    }

    fn remove_from_existing_map(&mut self) {
        if self.current_map.is_some() {
            let map_instance_ref = self.current_map.as_ref().unwrap();
            let mut map_instance_ref_guard = write_lock!(map_instance_ref);
            map_instance_ref_guard.remove_char_id_from_map(self.char_id);
        }
    }

    fn join_and_set_map(&mut self, map_instance: Arc<RwLock<MapInstance>>) {
        self.set_current_map(Some(map_instance.clone()));
        let pos_index = self.get_pos_index();
        let mut map_instance_guard = write_lock!(map_instance);
        map_instance_guard.add_char_id_to_map(pos_index, self.char_id);
    }

    pub fn update_position(&mut self, x:u16, y: u16) {
        let map_ref = self.current_map.as_ref().unwrap().clone();
        let mut map = write_lock!(map_ref);
        let old_position_index = coordinate::get_cell_index_of(self.current_position.x, self.current_position.y, map.x_size);
        map.characters_ids_location.remove(&old_position_index);
        self.set_current_x(x);
        self.set_current_y(y);
        let new_position_index = coordinate::get_cell_index_of(self.current_position.x, self.current_position.y, map.x_size);
        map.characters_ids_location.insert(new_position_index, self.char_id);
    }

    pub fn get_current_map_name(&self) -> String {
        self.current_map_name.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&mut self, id: u128) {
        self.movement_task_id = Some(id);
    }

    pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<&Arc<dyn MapItem>> {
        coordinate::get_item_at(x, y, PLAYER_FOV, &self.map_view)
    }
    pub fn set_map_item_at(&mut self, x: u16, y: u16, item: Arc<dyn MapItem>) {
        let i = coordinate::get_cell_index_of(x, y, PLAYER_FOV);
        self.map_view.insert(i, item);
    }

    // TODO try to optimize, method below take ~0.5ms to execute (peak at 1.5ms)

    #[elapsed]
    pub fn load_units_in_fov(&mut self, session_guard: &RwLockReadGuard<Session>) {
        let old_map_view = self.map_view.clone();
        self.map_view.clear();
        let map_ref = self.current_map.as_ref().unwrap().clone();
        let map = read_lock!(map_ref);
        let start_x = cmp::max(self.current_position.x - PLAYER_FOV, 0);
        let end_x = cmp::min(self.current_position.x + PLAYER_FOV, map.x_size);
        let start_y = cmp::max(self.current_position.y - PLAYER_FOV, 0);
        let end_y = cmp::min(self.current_position.y + PLAYER_FOV, map.y_size);
        for x in start_x..end_x {
            for y in start_y..end_y {
                if map.is_warp_cell(x, y) {
                    let warp = map.get_warp_at(x, y).unwrap();
                    if coordinate::get_item_at(warp.x, warp.y, PLAYER_FOV, &old_map_view).is_none() {
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
                    self.set_map_item_at(warp.x, warp.y, warp as Arc<dyn MapItem>);
                }
                let mob = map.get_mob_at(x, y);
                if mob.is_some() {
                    let mob = mob.unwrap();
                    let mob_clone = mob.clone();
                    let mob_guard = read_lock!(mob_clone);
                    if coordinate::get_item_at(mob_guard.x, mob_guard.y, PLAYER_FOV, &old_map_view).is_none() {
                        info!("Seeing: {} at {},{}",mob_guard.name, mob_guard.x, mob_guard.y);
                        let mut mob_name = [0 as char; 24];
                        mob_guard.name.fill_char_array(mob_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(mob_guard.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(5);
                        packet_zc_notify_standentry.set_clevel(3);
                        packet_zc_notify_standentry.set_aid(mob_guard.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x: mob_guard.x, y: mob_guard.y, dir: 3 }.to_pos());
                        packet_zc_notify_standentry.set_name(mob_name);
                        packet_zc_notify_standentry.fill_raw();
                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                    self.set_map_item_at(mob_guard.x, mob_guard.y, mob as Arc<dyn MapItem>);
                }
            }
        }
        let vanish_items = old_map_view.keys().map(|k| *k).collect::<Vec<usize>>();
        for item in vanish_items {
            if !self.map_view.contains_key(&item) {
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(old_map_view.get(&item).unwrap().id());
                packet_zc_notify_vanish.fill_raw();
                let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                socket_send!(tcp_stream, packet_zc_notify_vanish.raw());
            }
        }
    }
}