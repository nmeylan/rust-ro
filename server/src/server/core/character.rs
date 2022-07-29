use std::any::Any;
use std::collections::HashSet;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::{Acquire, Relaxed};

use tokio::runtime::Runtime;

use accessor::Setters;
use packets::packets::{PacketZcNotifyStandentry7, PacketZcNotifyVanish};
use packets::packets::Packet;

use crate::Server;
use crate::server::core::character_movement::Position;
use crate::server::core::map::{MAP_EXT, MapItem};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::mob::Mob;
use crate::server::core::path::manhattan_distance;
use crate::server::core::session::Session;
use crate::server::core::status::{LookType, Status};
use crate::server::enums::map_item::MapItemType;
use crate::server::server::PLAYER_FOV;
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
    pub current_map: RwLock<Option<Arc<MapInstance>>>,
    pub current_map_name: RwLock<[char; 16]>,
    pub x: AtomicU16,
    pub y: AtomicU16,
    pub movement_tasks: Mutex<Vec<MovementTask>>,
    pub map_view: RwLock<HashSet<Arc<dyn MapItem>>>,
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
    pub fn set_map_socket(&self, map_socket: Arc<RwLock<TcpStream>>) {
        let mut map_server_socket_guard = write_lock!(self.map_server_socket);
        *map_server_socket_guard = Some(map_socket);
    }

    pub fn set_self_ref(&self, self_ref: Arc<Character>) {
        let mut self_ref_guard = write_lock!(self.self_ref);
        *self_ref_guard = Some(self_ref);
    }

    fn update_x_y(&self, x: u16, y: u16) {
        self.x.store(x, Relaxed);
        self.y.store(y, Relaxed);
    }

    pub fn get_x(&self) -> u16 {
        self.x.load(Relaxed)
    }
    pub fn get_y(&self) -> u16 {
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
            let self_ref_guard = read_lock!(self.self_ref);
            map_instance_ref.remove_item(self_ref_guard.as_ref().unwrap().clone());
        }
    }

    pub fn join_and_set_map(&self, map_instance: Arc<MapInstance>) {
        self.set_current_map(map_instance.clone());
        let self_ref_guard = read_lock!(self.self_ref);
        map_instance.insert_item(self_ref_guard.as_ref().unwrap().clone());
    }

    pub fn update_position(&self, x: u16, y: u16) {
        info!("[{:?}] update_position {},{}", std::thread::current().id(), x, y);
        self.update_x_y(x, y);
    }

    pub fn get_current_map_name(&self) -> String {
        let current_map_name_guard = read_lock!(self.current_map_name);
        current_map_name_guard.iter().filter(|c| **c != '\0').collect()
    }
    pub fn add_movement_task_id(&self, task: MovementTask) {
        if let Ok(mut movement_tasks_guard) = self.movement_tasks.try_lock() {
            movement_tasks_guard.clear();
            movement_tasks_guard.push(task);
        } else {
            info!("Can't add task movement, drop request")
        }
    }
    pub fn remove_movement_task_id(&self, task: MovementTask) {
        let mut movement_tasks_guard = self.movement_tasks.lock().unwrap();
        let maybe_task = movement_tasks_guard.iter().enumerate().find(|(_, movement_task)| **movement_task == task);
        if let Some((index, _)) = maybe_task {
            movement_tasks_guard.remove(index);
        }
    }
    pub fn set_current_map(&self, current_map: Arc<MapInstance>) {
        let mut new_current_map: [char; 16] = [0 as char; 16];
        let map_name = format!("{}{}", current_map.name, MAP_EXT);
        map_name.fill_char_array(new_current_map.as_mut());
        self.set_current_map_name(new_current_map);
        let mut current_map_guard = write_lock!(self.current_map);
        *current_map_guard = Some(current_map);
    }

    pub fn clear_map_view(&self) {
        let mut map_view_guard = write_lock!(self.map_view);
        *map_view_guard = Default::default();
    }

    pub fn load_units_in_fov(&self, session: &Arc<Session>) {
        let mut new_map_view: HashSet<Arc<dyn MapItem>> = HashSet::with_capacity(2048);
        let mut map_view_guard = write_lock!(self.map_view);
        let current_map_guard = read_lock!(self.current_map);
        let map_ref = current_map_guard.as_ref().unwrap().clone();
        let map_items_guard = read_lock!(map_ref.map_items);
        let map_items_clone = map_items_guard.clone();
        drop(current_map_guard);
        drop(map_items_guard);
        for item in map_items_clone {
            if item.id() != self.id() && manhattan_distance(self.x(), self.y(), item.x(), item.y()) <= PLAYER_FOV {
                // info!("seeing {}", item.object_type());
                new_map_view.insert(item.clone());
            }
        }

        for map_item in new_map_view.iter() {
            if map_item.object_type() == MapItemType::Character.value() {
                continue;
            }
            // info!("See map_item {} at {},{}", map_item.object_type(), map_item.x(), map_item.y());
            if !map_view_guard.contains(&*map_item.clone()) {
                let mut name = [0 as char; 24];
                map_item.name().fill_char_array(name.as_mut());
                let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new();
                packet_zc_notify_standentry.set_job(map_item.client_item_class());
                packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(session.packetver()) as i16);
                packet_zc_notify_standentry.set_name(name);
                packet_zc_notify_standentry.set_pos_dir(Position { x: map_item.x(), y: map_item.y(), dir: 3 }.to_pos());
                packet_zc_notify_standentry.set_objecttype(map_item.object_type() as u8);
                packet_zc_notify_standentry.set_aid(map_item.id());
                packet_zc_notify_standentry.set_gid(map_item.id());
                if map_item.object_type() == MapItemType::Mob.value() {
                    let mob = cast!(map_item, Mob);
                    packet_zc_notify_standentry.set_clevel(3);
                    packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                    packet_zc_notify_standentry.set_hp(mob.status.hp);
                    packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                }
                packet_zc_notify_standentry.fill_raw_with_packetver(Some(session.packetver()));
                session.send_to_map_socket(packet_zc_notify_standentry.raw());
            }
        }

        for map_item in map_view_guard.iter() {
            if !new_map_view.contains(&*map_item.clone()) {
                // info!("Vanish map_item {} at {},{}", map_item.object_type(), map_item.x(), map_item.y());
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(map_item.id());
                packet_zc_notify_vanish.fill_raw();
                session.send_to_map_socket(packet_zc_notify_vanish.raw());
            }
        }
        *map_view_guard = new_map_view;
    }

    pub fn get_look(&self, look_type: LookType) -> u32 {
        if self.status.look.is_none() {
            error!("Character has no look");
            return 0;
        }
        let look = self.status.look.as_ref().unwrap();
        match look_type {
            LookType::Hair => look.hair.load(Relaxed) as u32,
            LookType::HairColor => look.hair_color.load(Relaxed),
            LookType::ClothesColor => look.clothes_color.load(Relaxed),
            LookType::Body => look.body.load(Relaxed),
            LookType::Weapon => look.weapon.load(Relaxed),
            LookType::Shield => look.shield.load(Relaxed),
            LookType::HeadBottom => look.head_bottom.load(Relaxed),
            LookType::HeadTop => look.head_top.load(Relaxed),
            LookType::HeadMid => look.head_middle.load(Relaxed),
            LookType::Robe => look.robe.load(Relaxed),
        }
    }

    pub fn change_look(&self, look: LookType, value: u32, runtime: &Runtime, server: Arc<Server>) {
        if self.status.look.is_none() {
            error!("Character has no look");
            return;
        }
        let db_column = match look {
            LookType::Hair => {
                self.status.look.as_ref().unwrap().hair.store(value as u16, Relaxed);
                "hair"
            }
            LookType::HairColor => {
                self.status.look.as_ref().unwrap().hair_color.store(value as u32, Relaxed);
                "hair_color"
            }
            LookType::ClothesColor => {
                self.status.look.as_ref().unwrap().clothes_color.store(value as u32, Relaxed);
                "clothes_color"
            }
            LookType::Body => {
                self.status.look.as_ref().unwrap().body.store(value as u32, Relaxed);
                "body"
            }
            LookType::Weapon => {
                self.status.look.as_ref().unwrap().weapon.store(value as u32, Relaxed);
                "weapon"
            }
            LookType::Shield => {
                self.status.look.as_ref().unwrap().shield.store(value as u32, Relaxed);
                "shield"
            }
            LookType::HeadBottom => {
                self.status.look.as_ref().unwrap().head_bottom.store(value as u32, Relaxed);
                "head_bottom"
            }
            LookType::HeadTop => {
                self.status.look.as_ref().unwrap().head_top.store(value as u32, Relaxed);
                "head_top"
            }
            LookType::HeadMid => {
                self.status.look.as_ref().unwrap().head_middle.store(value as u32, Relaxed);
                "head_mid"
            }
            LookType::Robe => {
                self.status.look.as_ref().unwrap().robe.store(value as u32, Relaxed);
                "robe"
            }
        };
        runtime.block_on(async {
            let sql = format!("UPDATE `char` SET `{}` = ? WHERE `char_id` = ?", db_column);
            sqlx::query(&sql).bind(value).bind(self.char_id).execute(&server.repository.pool).await.unwrap();
        });
    }
}