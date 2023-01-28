use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicI8;

use crate::server::core::map::MAP_EXT;

use crate::server::core::map_instance::MapInstance;
use crate::server::core::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::core::position::Position;
use crate::server::core::request::Request;
use crate::server::core::session::Session;
use crate::server::npc::script::Script;

use crate::server::state::character::Character;



pub struct ServerState {
    map_items: HashMap<u32, MapItem>,
    map_instances: HashMap<String, Vec<Arc<MapInstance>>>,
    map_instances_count: AtomicI8,
    sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    characters: HashMap<u32, Character>,
}

unsafe impl Sync for ServerState {}
unsafe impl Send for ServerState {}

impl ServerState {
    pub fn new(map_items: HashMap<u32, MapItem>) -> Self {
        Self {
            map_items,
            map_instances: Default::default(),
            map_instances_count: Default::default(),
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            characters: Default::default()
        }
    }

    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }
    pub fn get_session(&self, session_id: u32) -> Arc<Session> {
        let sessions = self.sessions.read().unwrap();
        let session_ref = sessions.get(&session_id).unwrap();
        session_ref.clone()
    }

    pub fn get_character_unsafe(&self, char_id: u32) -> &Character {
        self.characters.get(&char_id).unwrap()
    }

    pub fn get_character_from_context_unsafe(&self, context: &Request) -> &Character {
        let char_id = context.session().char_id.unwrap();
        self.characters.get(&char_id).unwrap()
    }

    pub fn insert_character(&mut self, character: Character) {
        self.map_items.insert(character.char_id, character.to_map_item());
        self.characters.insert(character.char_id, character);
    }

    pub fn get_map_socket_for_char_id(&self, char_id: u32) -> Option<Arc<RwLock<TcpStream>>> {
        let account_id = self.get_character_unsafe(char_id).account_id;
        let sessions = self.sessions.read().unwrap();
        let maybe_session = sessions.get(&account_id);
        if let Some(session) = maybe_session {
            session.map_server_socket.clone()
        } else {
            None
        }
    }
    pub fn insert_map_item(&mut self, id: u32, map_item: MapItem) {
        self.map_items.insert(id, map_item);
    }
    pub fn map_items(&self) -> &HashMap<u32, MapItem> {
        &self.map_items
    }

    pub fn sessions(&self) -> &Arc<RwLock<HashMap<u32, Arc<Session>>>> {
        &self.sessions
    }
    pub fn characters(&self) -> &HashMap<u32, Character> {
        &self.characters
    }
    pub fn characters_mut(&mut self) -> &mut HashMap<u32, Character> {
        &mut self.characters
    }
    pub fn map_instances(&self) -> &HashMap<String, Vec<Arc<MapInstance>>> {
        &self.map_instances
    }
    pub fn map_instances_mut(&mut self) -> &mut HashMap<String, Vec<Arc<MapInstance>>> {
        &mut self.map_instances
    }
    pub fn map_instances_count(&self) -> &AtomicI8 {
        &self.map_instances_count
    }

    #[inline]
    pub fn get_map_instance_from_character(&self, character: &Character) -> Option<Arc<MapInstance>> {
        self.get_map_instance(character.current_map_name(), character.current_map_instance())
    }
    #[inline]
    pub fn get_map_instance(&self, map_name: &String, map_instance_id: u8) -> Option<Arc<MapInstance>> {
        let map_name = if map_name.ends_with(MAP_EXT) {
            &map_name[..(map_name.len() - 4)]
        } else {
            map_name.as_str()
        };

        if let Some(instances) = self.map_instances().get(map_name) {
            for (id, map_instance) in instances.iter().enumerate() {
                if map_instance_id == id as u8 {
                    return Some(map_instance.clone());
                }
            }
        }
        None
    }


    #[inline]
    pub fn map_item_x_y(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Position> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(Position { x: character.x(), y: character.y(), dir: 3 }); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(Position { x: mob.x(), y: mob.y(), dir: 3 }); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(Position { x: warp.x(), y: warp.y(), dir: 0 });
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(Position { x: script.x(), y: script.y(), dir: script.dir() });
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item_name(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<String> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(character.name.clone()); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(mob.name.clone()); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(warp.name);
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.name().clone());
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item(&self, map_item_id: u32, map_name: &String, map_instance_id: u8) -> Option<MapItem> {
        let characters = self.characters();
        if let Some(character) = characters.get(&map_item_id) {
            return Some(character.to_map_item());
        }
        if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
            if let Some(mob) = map_instance.state().get_mob(map_item_id) {
                return Some(mob.to_map_item());
            }
            if let Some(warp) = map_instance.get_warp(map_item_id) {
                return Some(warp.to_map_item());
            }
            if let Some(script) = map_instance.get_script(map_item_id) {
                return Some(script.to_map_item());
            }
        }
        None
    }

    pub fn map_item_script(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Arc<Script>> {
        match map_item.object_type() {
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script);
                    }
                }
                None
            }
            _ => None
        }
    }

    pub fn map_item_character(&self, map_item: &MapItem) -> Option<&Character> {
        match map_item.object_type() {
            MapItemType::Character => {
                return Some(self.get_character_unsafe(map_item.id()))
            }
            _ => None
        }
    }

}