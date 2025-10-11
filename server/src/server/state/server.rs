use std::collections::{HashMap, HashSet};
use std::net::TcpStream;
use std::sync::atomic::AtomicI8;
use std::sync::{Arc, RwLock};

use models::position::Position;
use models::status::StatusSnapshot;

use crate::server::model::map::MAP_EXT;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType, MapItems, ToMapItem, ToMapItemSnapshot};
use crate::server::model::request::Request;
use crate::server::model::script::Script;
use crate::server::model::session::Session;
use crate::server::state::character::Character;
use crate::util::hasher::NoopHasherU32;

pub struct ServerState {
    map_items: MapItems,
    map_instances: HashMap<String, Vec<Arc<MapInstance>>>,
    map_instances_count: AtomicI8,
    sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    characters: HashMap<u32, Character, NoopHasherU32>,
    locked_map_item: HashSet<u32, NoopHasherU32>, /* map item that should be removed from map instance, in next tick, avoid to use them
                                                   * meanwhile. */
}

unsafe impl Sync for ServerState {}
unsafe impl Send for ServerState {}

impl ServerState {
    pub fn new(map_items: MapItems) -> Self {
        Self {
            map_items,
            map_instances: Default::default(),
            map_instances_count: Default::default(),
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            characters: Default::default(),
            locked_map_item: Default::default(),
        }
    }

    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }

    pub fn add_session(&self, session_id: u32, session: Arc<Session>) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id, session);
    }

    pub fn get_session(&self, session_id: u32) -> Arc<Session> {
        let sessions = self.sessions.read().unwrap();
        let session_ref = sessions.get(&session_id).unwrap();
        session_ref.clone()
    }

    pub fn get_character_unsafe(&self, char_id: u32) -> &Character {
        self.characters.get(&char_id).unwrap()
    }

    pub fn get_character(&self, char_id: u32) -> Option<&Character> {
        self.characters.get(&char_id)
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
        if let Some(character) = self.get_character(char_id) {
            let account_id = character.account_id;
            let sessions = self.sessions.read().unwrap();
            let maybe_session = sessions.get(&account_id);
            if let Some(session) = maybe_session {
                return session.map_server_socket.clone();
            }
        }
        None
    }

    pub fn insert_map_item(&mut self, id: u32, map_item: MapItem) {
        self.map_items.insert(id, map_item);
    }

    pub fn sessions(&self) -> &Arc<RwLock<HashMap<u32, Arc<Session>>>> {
        &self.sessions
    }

    pub fn characters(&self) -> &HashMap<u32, Character, NoopHasherU32> {
        &self.characters
    }

    pub fn characters_mut(&mut self) -> &mut HashMap<u32, Character, NoopHasherU32> {
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

    pub fn insert_locked_map_item(&mut self, id: u32) {
        self.locked_map_item.insert(id);
    }

    pub fn contains_locked_map_item(&self, id: u32) -> bool {
        self.locked_map_item.contains(&id)
    }

    pub fn remove_locked_map_item(&mut self, id: u32) {
        self.locked_map_item.remove(&id);
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
                    return Some(Position {
                        x: character.x(),
                        y: character.y(),
                        dir: 3,
                    }); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(Position {
                            x: mob.x(),
                            y: mob.y(),
                            dir: 3,
                        }); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(Position {
                            x: warp.x(),
                            y: warp.y(),
                            dir: 0,
                        });
                    }
                }
                None
            }
            MapItemType::Unknown => None,
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(Position {
                            x: script.x(),
                            y: script.y(),
                            dir: script.dir(),
                        });
                    }
                }
                None
            }
            MapItemType::DroppedItem => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item.id()) {
                        return Some(Position {
                            x: dropped_item.x(),
                            y: dropped_item.y(),
                            dir: 0,
                        });
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
                        return Some(mob.name_english.clone()); // TODO add dir to character
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
            MapItemType::Unknown => None,
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.name().clone());
                    }
                }
                None
            }
            MapItemType::DroppedItem => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item.id()) {
                        return Some(dropped_item.item_id().to_string());
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
            _ => None,
        }
    }

    pub fn map_item_character(&self, map_item: &MapItem) -> Option<&Character> {
        match map_item.object_type() {
            MapItemType::Character => return Some(self.get_character_unsafe(map_item.id())),
            _ => None,
        }
    }

    pub fn map_item_mob_status(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<StatusSnapshot> {
        match map_item.object_type() {
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(mob.status.clone());
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn map_item_snapshot(&self, map_item_id: u32, map_name: &String, map_instance_id: u8) -> Option<MapItemSnapshot> {
        let characters = self.characters();
        if let Some(character) = characters.get(&map_item_id) {
            return Some(character.to_map_item_snapshot());
        }
        if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
            if let Some(mob) = map_instance.state().get_mob(map_item_id) {
                return Some(mob.to_map_item_snapshot());
            }
            if let Some(_warp) = map_instance.get_warp(map_item_id) {
                return None;
            }
            if let Some(_script) = map_instance.get_script(map_item_id) {
                return None;
            }
        }
        None
    }
}
