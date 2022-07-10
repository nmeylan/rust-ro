use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use futures::StreamExt;
use packets::packets::PacketZcNotifyMove;
use crate::server::core::character::Character;
use crate::server::core::map::{Map, MapItem, WARP_MASK};
use crate::server::core::mob::Mob;
use crate::server::core::path::manhattan_distance;
use crate::server::core::status::Status;
use crate::server::enums::map_item::MapItemType;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::{MOB_FOV, Server};
use crate::util::coordinate;
use crate::util::packet::{chain_packets_raws};
use packets::packets::Packet;
use std::io::Write;
use rathena_script_lang_interpreter::lang::vm::Vm;
use crate::ScriptHandler;

pub struct MapInstance {
    pub name: String,
    pub id: u32,
    pub x_size: u16,
    pub y_size: u16,
    // index in this array will give x and y position of the cell.
    // 2 bytes representing cell type:
    // bit 0 -> walkable
    // bit 1 -> shootable
    // bit 2 -> water
    // bit 3 -> npc
    // bit 4 -> basilica
    // bit 5 -> landprotector
    // bit 6 -> novending
    // bit 7 -> nochat
    // bit 8 -> icewall
    // bit 9 -> noicewall
    // bit 10 -> noskill
    // bit 11 -> warp
    // bit 12 -> mob
    pub cells: Vec<u16>,
    pub warps: Arc<Vec<Arc<Warp>>>,
    pub mob_spawns: Arc<Vec<Arc<MobSpawn>>>,
    pub mob_spawns_tracks: RwLock<Vec<MobSpawnTrack>>,
    pub mobs: RwLock<HashMap<u32, Arc<Mob>>>,
    pub characters: RwLock<HashSet<Arc<dyn MapItem>>>,
    pub map_items: RwLock<HashSet<Arc<dyn MapItem>>>
}

pub struct MobSpawnTrack {
    pub spawn_id: u32,
    pub spawned_amount: i16,
    pub mob_respawn_at: Vec<u128>
}

impl MobSpawnTrack {
    pub fn default(spawn_id: u32) -> MobSpawnTrack {
        MobSpawnTrack {
            spawn_id,
            spawned_amount: 0,
            mob_respawn_at: Default::default()
        }
    }

    pub fn increment_spawn(&mut self) {
        self.spawned_amount += 1;
    }
}

impl MapInstance {
    pub fn from_map(map: &Map, server: Arc<Server>, id: u32, cells: Vec<u16>, mut map_items: HashSet<Arc<dyn MapItem>>) -> MapInstance {
        let _cells_len = cells.len();
        map.scripts.iter().for_each(|script| {
            let (class_reference, instance_reference) = Vm::create_instance(server.vm.clone(), script.class_name.clone(), Box::new(&ScriptHandler)).unwrap();
            let mut script = script.clone();
            script.set_instance_reference(instance_reference);
            let script_arc = Arc::new(script);
            server.insert_map_item(script_arc.id(), script_arc.clone());
            map_items.insert(script_arc.clone());
        });
        MapInstance {
            name: map.name.clone(),
            id,
            x_size: map.x_size.clone(),
            y_size: map.y_size.clone(),
            cells,
            warps: map.warps.clone(),
            mob_spawns: map.mob_spawns.clone(),
            mob_spawns_tracks: RwLock::new(map.mob_spawns.iter().map(|spawn| MobSpawnTrack::default(spawn.id.clone())).collect::<Vec<MobSpawnTrack>>()),
            mobs: Default::default(),
            characters: RwLock::new(HashSet::with_capacity(50)),
            map_items: RwLock::new(map_items)
        }
    }

    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        coordinate::get_cell_index_of(x, y, self.x_size)
    }

    pub fn is_cell_walkable(&self, x: u16, y: u16) -> bool {
        if self.cells.is_empty() {
            warn!("Cannot call is_cell_walkable as cells are not initialized, returning false");
            return false
        }
        (self.cells.get(self.get_cell_index_of(x, y)).unwrap() & 0b0000_0000_0000_0001) == 0b0000_0000_0000_0001
    }

    pub fn is_warp_cell(&self, x: u16, y: u16) -> bool {
        if self.cells.is_empty() {
            warn!("Cannot call is_warp_cell as cells are not initialized, returning false");
            return false
        }
        let i = self.get_cell_index_of(x, y);
        match self.cells.get(i) {
            Some(value) => (value & WARP_MASK) == WARP_MASK,
            None => false
        }
    }

    pub fn spawn_mobs(&self, server: Arc<Server>, _now: u128, self_ref: Arc<MapInstance>) {
        for mob_spawn in self.mob_spawns.iter() {
            let mut mob_spawns_tracks_guard = write_lock!(self.mob_spawns_tracks);
            let mob_spawn_track = mob_spawns_tracks_guard.iter_mut().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap();
            if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                continue;
            }
            if mob_spawn.has_delay() {
                // TODO check when respawn is planned
            }
            let mut cell: (u16, u16);
            let spawned = mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount;
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {
                    // if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(self.cells.as_ref(), self.x_size);
                }
                let mob_id = server.generate_map_item_id();
                let mob = Mob::new(mob_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone(), self_ref.clone(), Status::from_mob_model(&mob_spawn.info));
                let mob_ref = Arc::new(mob);
                mob_ref.set_self_ref(mob_ref.clone());

                let mut mobs_guard = write_lock!(self.mobs);
                let mut map_items_guard = write_lock!(self.map_items);
                // TODO: On mob dead clean up should be down also for items below
                server.insert_map_item(mob_id, mob_ref.clone());
                mobs_guard.insert(mob_id, mob_ref.clone());
                map_items_guard.insert(mob_ref);
                // END
                mob_spawn_track.increment_spawn();
            }
        }
    }

    pub fn update_mobs_fov(&self) {
        let map_items_guard = read_lock!(self.map_items);
        let characters_guard = read_lock!(self.characters);
        let map_items_clone = map_items_guard.clone();
        let characters_clone = characters_guard.clone();
        drop(map_items_guard);
        drop(characters_guard);
        for item in map_items_clone {
            let mut viewed_chars: Vec<Arc<dyn MapItem>> = Vec::with_capacity(characters_clone.len());
            if item.object_type() == MapItemType::Mob.value() {
                for character in characters_clone.iter() {
                    if manhattan_distance(character.x(), character.y(), item.x(), item.y()) <= MOB_FOV {
                        viewed_chars.push(character.clone());
                    }
                }
                let mob = cast!(item, Mob);
                mob.update_map_view(viewed_chars);
            }
        }
    }

    pub fn mobs_action(&self) {
        let mobs_guard = read_lock!(self.mobs);
        let mut character_packets_map: HashMap<Arc<dyn MapItem>, Vec<PacketZcNotifyMove>> = HashMap::new();
        for mob in mobs_guard.values() {
            let character_packets = mob.action_move();
            character_packets.iter().for_each(|(character, packet)| {
                if !character_packets_map.contains_key(&character.clone()) {
                    character_packets_map.insert(character.clone(), Vec::with_capacity(500));
                }
                character_packets_map.get_mut(character).unwrap().push(packet.clone());
            });
        }
        for (character, mut packets) in character_packets_map.iter() {
            let character = cast!(character, Character);
            let packets = chain_packets_raws(packets.into_iter().map(|packet| packet.raw()).collect::<Vec<&Vec<u8>>>());
            let map_socket_guard = write_lock!(character.map_server_socket);
            let character_socket = map_socket_guard.as_ref().unwrap();
            socket_send!(character_socket, &packets);
        }
    }

    pub fn get_warp_at(&self, x: u16, y: u16) -> Option<Arc<Warp>> {
        for warp in self.warps.iter() {
            if x >= warp.x - warp.x_size && x <= warp.x + warp.x_size
                && y >= warp.y - warp.y_size && y <= warp.y + warp.y_size {
                return Some(warp.clone());
            }
        }
        None
    }

    pub fn insert_item(&self, map_item: Arc<dyn MapItem>) {
        let mut map_item_guard = write_lock!(self.map_items);
        map_item_guard.insert(map_item.clone());
        if map_item.object_type() == MapItemType::Character.value() {
            self.insert_character(map_item);
        }
        // TODO notify mobs
    }

    pub fn insert_character(&self, character: Arc<dyn MapItem>) {
        let mut characters_guard = write_lock!(self.characters);
        characters_guard.insert(character);
    }

    pub fn remove_item(&self, map_item: Arc<dyn MapItem>) {
        let mut map_item_guard = write_lock!(self.map_items);
        map_item_guard.remove(&*map_item.clone());
        if map_item.object_type() == MapItemType::Character.value() {
            self.remove_character(map_item);
        }
    }

    pub fn remove_character(&self, character: Arc<dyn MapItem>) {
        let mut characters_guard = write_lock!(self.characters);
        characters_guard.remove(&*character.clone());
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
        if x >= self.x_size {
            return self.x_size - 1;
        }
        x
    }

    #[inline]
    pub fn get_item_y_from_fov(&self, y: u16, range: u16, j: u16) -> u16 {
        let y = self.get_fov_start_y(y, range) + j;
        if y >= self.y_size {
            return self.y_size - 1;
        }
        y
    }
}