use std::any::Any;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::mem;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::{Acquire, Relaxed};
use std::sync::mpsc::SyncSender;

use tokio::runtime::Runtime;

use accessor::Setters;
use packets::packets::{PacketZcNotifyStandentry7, PacketZcNotifyVanish};
use packets::packets::Packet;

use crate::Server;
use crate::server::core::movement::Movement;
use crate::server::core::map::MAP_EXT;
use crate::server::core::map_instance::{MapInstance, MapInstanceKey};
use crate::server::state::mob::Mob;
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::core::path::manhattan_distance;
use crate::server::core::position::Position;
use crate::server::core::session::Session;
use crate::server::state::status::{LookType, Status};
use crate::server::core::map_item::{MapItem, MapItemSnapshot, MapItemType};
use crate::server::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::script::{GlobalVariableEntry, ScriptGlobalVariableStore};
use crate::server::server::{PACKETVER, PLAYER_FOV};
use crate::util::coordinate;
use crate::util::string::StringUtil;

pub type MovementTask = u64;

#[derive(Setters)]
pub struct Character {
    #[set]
    pub name: String,
    pub status: Status,
    #[set]
    pub char_id: u32,
    pub account_id: u32,
    pub map_instance_key: MapInstanceKey,
    pub loaded_from_client_side: bool,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    pub movements: Vec<Movement>,
    pub map_view: HashSet<MapItem>,
    pub script_variable_store: Mutex<ScriptGlobalVariableStore>,
}

impl Character {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn dir(&self) -> u16 {
        self.dir
    }

    pub fn is_moving(&self) -> bool {
        self.movements.len() > 0
    }

    pub fn pop_movement(&mut self) -> Option<Movement> {
        self.movements.pop()
    }
    pub fn peek_movement(&self) -> Option<&Movement> {
        self.movements.last()
    }
    pub fn peek_mut_movement(&mut self) -> Option<&mut Movement> {
        self.movements.last_mut()
    }
    pub fn movements(&mut self) -> &Vec<Movement> {
        &self.movements
    }

    pub fn set_movement(&mut self, movements: Vec<Movement>) {
        self.movements = movements;
    }
    pub fn clear_movement(&mut self) {
        self.movements = vec![];
    }

    pub fn join_and_set_map(&mut self, map_instance: Arc<MapInstance>) {
        self.set_current_map(map_instance.clone());
        map_instance.insert_item(self.to_map_item());
        self.movements = vec![];
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn set_current_map(&mut self, current_map: Arc<MapInstance>) {
        self.map_instance_key = MapInstanceKey::new(current_map.name.clone(), current_map.id);
    }

    pub fn current_map_name(&self) -> &String {
        &self.map_instance_key.map_name()
    }

    pub fn current_map_name_char(&self) -> [char; 16] {
        self.map_instance_key.map_name_char()
    }
    pub fn current_map_instance(&self) -> u8 {
        self.map_instance_key.map_instance()
    }

    pub fn clear_map_view(&mut self) {
        self.map_view = Default::default();
    }

    pub fn load_units_in_fov(&mut self, server: &Server, client_notification_sender_clone: SyncSender<Notification>) {
        let map_instance = server.get_map_instance(self.current_map_name(), self.current_map_instance());
        if map_instance.is_none() {
            return;
        }
        let map_instance = map_instance.unwrap();
        let mut new_map_view: HashSet<MapItem> = HashSet::with_capacity(2048);
        for (_, item) in map_instance.map_items.borrow().iter() {
            if let Some(position) = server.map_item_x_y(&item, &self.current_map_name(), self.current_map_instance()) {
                if item.id() != self.char_id && manhattan_distance(self.x(), self.y(), position.x, position.y) <= PLAYER_FOV {
                    // info!("seeing {}", item.object_type());
                    new_map_view.insert(item.clone());
                }
            }
        }

        for map_item in new_map_view.iter() {
            if !self.map_view.contains(map_item) {
                let default_name = "unknown".to_string();
                let map_item_name = server.map_item_name(map_item, &self.current_map_name(), self.current_map_instance()).unwrap_or(default_name);
                let position = server.map_item_x_y(&map_item, &self.current_map_name(), self.current_map_instance()).unwrap();
                info!("See map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                let mut name = [0 as char; 24];
                map_item_name.fill_char_array(name.as_mut());
                let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new();
                packet_zc_notify_standentry.set_job(map_item.client_item_class());
                packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(server.packetver()) as i16);
                // packet_zc_notify_standentry.set_name(name);
                packet_zc_notify_standentry.set_pos_dir(position.to_pos());
                packet_zc_notify_standentry.set_objecttype(map_item.object_type_value() as u8);
                packet_zc_notify_standentry.set_aid(map_item.id());
                packet_zc_notify_standentry.set_gid(map_item.id());
                match *map_item.object_type() {
                    MapItemType::Character => {}
                    MapItemType::Mob => {
                        if let Some(mob) = map_instance.get_mob(map_item.id()) {
                            packet_zc_notify_standentry.set_clevel(3);
                            packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                            packet_zc_notify_standentry.set_hp(mob.status.hp);
                            packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                        }
                    }
                    MapItemType::Warp => {}
                    MapItemType::Unknown => {}
                    MapItemType::Npc => {}
                }
                packet_zc_notify_standentry.fill_raw_with_packetver(Some(server.packetver()));
                client_notification_sender_clone.send(Notification::Char(CharNotification::new(self.char_id, mem::take(packet_zc_notify_standentry.raw_mut()))));
            }
        }

        for map_item in self.map_view.iter() {
            if !new_map_view.contains(map_item) {
                let position = server.map_item_x_y(&map_item, &self.current_map_name(), self.current_map_instance()).unwrap();
                info!("Vanish map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(map_item.id());
                packet_zc_notify_vanish.fill_raw();
                client_notification_sender_clone.send(Notification::Char(CharNotification::new(self.char_id, mem::take(packet_zc_notify_vanish.raw_mut()))));
            }
        }
        self.map_view = new_map_view;
    }

    pub fn get_look(&self, look_type: LookType) -> u32 {
        if self.status.look.is_none() {
            error!("Character has no look");
            return 0;
        }
        let look = self.status.look.as_ref().unwrap();
        match look_type {
            LookType::Hair => look.hair as u32,
            LookType::HairColor => look.hair_color,
            LookType::ClothesColor => look.clothes_color,
            LookType::Body => look.body,
            LookType::Weapon => look.weapon,
            LookType::Shield => look.shield,
            LookType::HeadBottom => look.head_bottom,
            LookType::HeadTop => look.head_top,
            LookType::HeadMid => look.head_middle,
            LookType::Robe => look.robe,
            _ => look.robe, // TODO
        }
    }

    pub fn change_look(&mut self, look: LookType, value: u32) -> Option<String> {
        if self.status.look.is_none() {
            error!("Character has no look");
            return None;
        }
        let db_column = match look {
            LookType::Hair => {
                self.status.look.as_mut().unwrap().hair = value as u16;
                "hair"
            }
            LookType::HairColor => {
                self.status.look.as_mut().unwrap().hair_color = value as u32;
                "hair_color"
            }
            LookType::ClothesColor => {
                self.status.look.as_mut().unwrap().clothes_color = value as u32;
                "clothes_color"
            }
            LookType::Body => {
                self.status.look.as_mut().unwrap().body = value as u32;
                "body"
            }
            LookType::Weapon => {
                self.status.look.as_mut().unwrap().weapon = value as u32;
                "weapon"
            }
            LookType::Shield => {
                self.status.look.as_mut().unwrap().shield = value as u32;
                "shield"
            }
            LookType::HeadBottom => {
                self.status.look.as_mut().unwrap().head_bottom = value as u32;
                "head_bottom"
            }
            LookType::HeadTop => {
                self.status.look.as_mut().unwrap().head_top = value as u32;
                "head_top"
            }
            LookType::HeadMid => {
                self.status.look.as_mut().unwrap().head_middle = value as u32;
                "head_mid"
            }
            LookType::Robe => {
                self.status.look.as_mut().unwrap().robe = value as u32;
                "robe"
            }
            _ => { "shoes" }
        };
        return Some(db_column.to_string());
    }

    pub fn get_zeny(&self) -> u32 {
        self.status.zeny
    }

    pub fn change_zeny(&mut self, value: u32) {
        self.status.zeny = value;
    }
}

impl ToMapItem for Character {
    fn to_map_item(&self) -> MapItem {
        let client_item_class = 0;  // TODO return job id
        MapItem::new(self.char_id, client_item_class, MapItemType::Character)
    }
}

impl ToMapItemSnapshot for Character {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot {
        let client_item_class = 0;  // TODO return job id
        MapItemSnapshot::new(
            MapItem::new(self.char_id, client_item_class, MapItemType::Character),
            Position { x: self.x, y: self.y, dir: self.dir },
        )
    }
}