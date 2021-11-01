use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::server::core::map::{Map, MapItem, WARP_MASK};
use crate::server::core::mob::Mob;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
use crate::server::server::UNKNOWN_MAP_ITEM;
use crate::util::coordinate;


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
    pub mob_spawns_tracks: Vec<MobSpawnTrack>,
    pub mobs: HashMap<u32, Arc<Mob>>,
    pub map_items: Vec<Arc<dyn MapItem>>
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
    pub fn from_map(map: &Map, id: u32, cells: Vec<u16>) -> MapInstance {
        let cells_len = cells.len();
        MapInstance {
            name: map.name.clone(),
            id,
            x_size: map.x_size.clone(),
            y_size: map.y_size.clone(),
            cells,
            warps: map.warps.clone(),
            mob_spawns: map.mob_spawns.clone(),
            mob_spawns_tracks: map.mob_spawns.iter().map(|spawn| MobSpawnTrack::default(spawn.id.clone())).collect::<Vec<MobSpawnTrack>>(),
            mobs: Default::default(),
            map_items: vec![UNKNOWN_MAP_ITEM.clone(); cells_len]
        }
    }

    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        coordinate::get_cell_index_of(x, y, self.x_size)
    }
    #[inline]
    pub fn get_pos_of(&self, index: u32) -> (u16, u16) {
        coordinate::get_pos_of(index, self.x_size)
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

    pub fn spawn_mobs(&mut self, server: Arc<Server>, now: u128, self_ref: Arc<RwLock<MapInstance>>) {
        for mob_spawn in self.mob_spawns.iter() {
            let mob_spawn_track = self.mob_spawns_tracks.iter_mut().find(|spawn_track| spawn_track.spawn_id == mob_spawn.id).unwrap();
            if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                continue;
            }
            if mob_spawn.has_delay() {
                // TODO check when respawn is planned
            }
            let mut cell: (u16, u16);

            let spawned = (mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount);
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {}
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(self.cells.as_ref(), self.x_size);
                } else {
                    cell = Map::find_random_walkable_cell(self.cells.as_ref(), self.x_size);
                }
                let mob_id = server.generate_map_item_id();
                let mob = Mob::new(mob_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone(), self_ref.clone());
                info!("Spawned {} at {},{})", mob_spawn.name, cell.0, cell.1);
                let mob_ref = Arc::new(mob);
                // TODO: On mob dead clean up should be down also for items below
                server.insert_map_item(mob_id, mob_ref.clone());

                self.mobs.insert(mob_id, mob_ref.clone());
                self.map_items.insert(coordinate::get_cell_index_of(cell.0, cell.1, self.x_size), mob_ref);
                // END
                mob_spawn_track.increment_spawn();
            }
            info!("Spawned {} {} (spawn id {})", spawned, mob_spawn.name, mob_spawn.id);
        }
    }

    pub fn update_mob_fov(&mut self) {
        for mob in self.mobs.values() {
            mob.load_units_in_fov(&self);
        }
    }

    pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<Arc<dyn MapItem>> {
        let key = coordinate::get_cell_index_of(x, y, self.x_size);
        let option = self.map_items.get(key);
        match option {
            Some(e) => Some(e.clone()),
            None => None
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

    pub fn add_char_to_map(&mut self, pos_index: usize, char_session: Arc<dyn MapItem>) {
        self.map_items.insert(pos_index, char_session);
        // TODO notify mobs
    }

    pub fn remove_char_id_from_map(&mut self, pos_index: usize) {
        self.map_items.remove(pos_index);
    }
}