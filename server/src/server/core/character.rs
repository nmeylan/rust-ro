use std::cmp;
use std::collections::HashMap;
use std::sync::{Arc, RwLockReadGuard};
use crate::packets::packets::{PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use crate::server::core::map::{Map, MapItem};
use crate::server::core::movement::Position;
use crate::server::core::session::Session;
use crate::server::server::PLAYER_FOV;
use crate::packets::packets::Packet;
use crate::util::coordinate;
use crate::util::string::StringUtil;
use std::io::Write;
use accessor::Setters;

#[derive(Setters, Debug)]
pub struct CharacterSession {
    #[set]
    pub name: [char; 24],
    #[set]
    pub speed: u16,
    #[set]
    pub char_id: u32,
    #[set]
    pub current_map: [char; 16],
    #[set]
    pub current_position: Position,
    pub movement_task_id: Option<u128>,
    pub map_view: HashMap<usize, Arc<dyn MapItem>>
}

impl CharacterSession {
    pub fn set_current_x(&mut self, current_x: u16) {
        self.current_position.x = current_x;
    }
    pub fn set_current_y(&mut self, current_y: u16) {
        self.current_position.y = current_y;
    }
    pub fn get_current_map_name(&self) -> String {
        self.current_map.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&mut self, id: u128) {
        self.movement_task_id = Some(id);
    }

    pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<&Arc<dyn MapItem>> {
        coordinate::get_item_at(x, y, PLAYER_FOV , &self.map_view)
    }
    pub fn set_map_item_at(&mut self, x: u16, y: u16, item: Arc<dyn MapItem>) {
        let i = coordinate::get_cell_index_of(x, y, PLAYER_FOV);
        self.map_view.insert(i, item);
    }

    pub fn load_units_in_fov(&mut self, map: &Map, session_guard: &RwLockReadGuard<Session>) {
        let old_map_view = self.map_view.clone();
        self.map_view.clear();
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
            }
        }
        let items = old_map_view.keys().map(|k| *k).collect::<Vec<usize>>();
        for item in items {
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