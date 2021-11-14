use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::server::core::map::{Map, MapItem, WALKABLE_MASK, WARP_MASK};
use crate::server::core::mob::Mob;
use crate::server::enums::map_item::MapItemType;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
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
    pub mob_spawns_tracks: RwLock<Vec<MobSpawnTrack>>,
    pub mobs: RwLock<HashMap<u32, Arc<Mob>>>,
    pub map_items: RwLock<Vec<Option<Arc<dyn MapItem>>>>
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
    pub fn from_map(map: &Map, id: u32, cells: Vec<u16>, map_items: Vec<Option<Arc<dyn MapItem>>>) -> MapInstance {
        let _cells_len = cells.len();
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
        let mut spawned_something = false;
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
            spawned_something = true;
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
                let mob = Mob::new(mob_id, cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone(), self_ref.clone());
                info!("Spawned {} at {},{} (index {})", mob_spawn.name, cell.0, cell.1, coordinate::get_cell_index_of(cell.0, cell.1, self.x_size));
                let mob_ref = Arc::new(mob);

                let mut mobs_guard = write_lock!(self.mobs);
                let mut map_items_guard = write_lock!(self.map_items);
                // TODO: On mob dead clean up should be down also for items below
                server.insert_map_item(mob_id, mob_ref.clone());
                mobs_guard.insert(mob_id, mob_ref.clone());
                map_items_guard[coordinate::get_cell_index_of(cell.0, cell.1, self.x_size)] = Some(mob_ref);
                // END
                mob_spawn_track.increment_spawn();
            }
            info!("Spawned {} {} (spawn id {})", spawned, mob_spawn.name, mob_spawn.id);
        }
        if spawned_something {
            // self.print_map_cells();
        }
    }

    pub fn update_mob_fov(&self) {
        let mobs_guard = read_lock!(self.mobs);
        for mob in mobs_guard.values() {
            mob.load_units_in_fov(&self);
        }
    }

    #[allow(dead_code)]
    pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<Arc<dyn MapItem>> {
        let map_items_guard = read_lock!(self.map_items);
        let key = coordinate::get_cell_index_of(x, y, self.x_size);
        let map_item_option = unsafe { map_items_guard.get_unchecked(key) };
        return map_item_option.clone();
    }

    pub fn get_map_items(&self, x: u16, y: u16, range: u16) -> Vec<Arc<dyn MapItem>> {
        let map_items_guard = read_lock!(self.map_items);
        let row_size = range * 2;
        let mut items = Vec::with_capacity((row_size * row_size) as usize);
        for j in 0..row_size {
            for i in 0..row_size {
                let x = self.get_item_x_from_fov(x, range, i);
                let y = self.get_item_y_from_fov(y, range, j);
                if x > self.x_size || y > self.y_size || coordinate::get_cell_index_of(x, y, self.x_size) >= map_items_guard.len() {
                    info!("{},{}", x ,y);
                }
                let item_option = map_items_guard[
                    coordinate::get_cell_index_of(x, y, self.x_size)
                    ].as_ref();
                if item_option.is_some() {
                    items.push(item_option.unwrap().clone());
                }
            }
        }
        items
    }

    #[allow(dead_code)]
    pub fn print_map_cells(&self) {
        for i in (0..self.y_size).rev() {
            for j in 0..self.x_size {
                let index = coordinate::get_cell_index_of(j, i, self.x_size);
                let cell = self.cells.get(index).unwrap();
                let mut c = "0";
                if cell & WARP_MASK == WARP_MASK {
                    c = "w";
                } else if cell & WALKABLE_MASK == WALKABLE_MASK {
                    c = "1"
                }
                let option = self.get_map_item_at(j, i);
                if option.is_some() {
                    let item = option.unwrap();
                    if item.object_type() == MapItemType::Mob.value() {
                        c = "M";
                    } else if item.object_type() == MapItemType::Character.value() {
                        c = "P";
                    } else if item.object_type() == MapItemType::Warp.value() {
                        c = "W";
                    } else {
                        c = "u"
                    }
                }
                print!("{}", c);
            }
            println!();
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

    pub fn insert_item_at(&self, pos_index: usize, char_session: Arc<dyn MapItem>) {
        let mut map_item_guard = write_lock!(self.map_items);
        map_item_guard.insert(pos_index, Some(char_session));
        // TODO notify mobs
    }

    pub fn remove_item_at(&self, pos_index: usize) {
        let mut map_item_guard = write_lock!(self.map_items);
        map_item_guard.remove(pos_index);
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