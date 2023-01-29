use std::collections::HashMap;
use crate::server::model::map::WARP_MASK;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::MapItem;
use crate::server::state::mob::Mob;


use crate::util::coordinate;

#[derive(SettersAll)]
pub struct MapInstanceState {
    key: MapInstanceKey,
    x_size: u16,
    y_size: u16,
    // index in this array will give x and y position of the cell.
    // 2 bytes representing cell type:
    // bit 0 -> walkable
    // bit 1 -> shootable
    // bit 2 -> water
    // bit 3 -> boot
    // bit 4 -> basilica
    // bit 5 -> landprotector
    // bit 6 -> novending
    // bit 7 -> nochat
    // bit 8 -> icewall
    // bit 9 -> noicewall
    // bit 10 -> noskill
    // bit 11 -> warp
    // bit 12 -> mob
    cells: Vec<u16>,
    mobs: HashMap<u32, Mob>,
    map_items: HashMap<u32, MapItem>,
    mob_spawns_tracks: Vec<MobSpawnTrack>,
}

pub struct MobSpawnTrack {
    pub spawn_id: u32,
    pub spawned_amount: i16,
    pub mob_respawn_at: Vec<u128>,
}

impl MobSpawnTrack {
    pub fn default(spawn_id: u32) -> MobSpawnTrack {
        MobSpawnTrack {
            spawn_id,
            spawned_amount: 0,
            mob_respawn_at: Default::default(),
        }
    }

    pub fn increment_spawn(&mut self) {
        self.spawned_amount += 1;
    }
}

impl MapInstanceState {
    pub fn new(key: MapInstanceKey, x_size: u16, y_size: u16, cells: Vec<u16>,
               map_items: HashMap<u32, MapItem>, mob_spawns_tracks: Vec<MobSpawnTrack>) -> MapInstanceState {
        Self {
            key,
            x_size,
            y_size,
            cells,
            mobs: Default::default(),
            map_items,
            mob_spawns_tracks,
        }
    }

    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        coordinate::get_cell_index_of(x, y, self.x_size())
    }

    pub fn is_warp_cell(&self, x: u16, y: u16) -> bool {
        if self.cells().is_empty() {
            warn!("Cannot call is_warp_cell as cells are not initialized, returning false");
            return false
        }
        let i = self.get_cell_index_of(x, y);
        match self.cells().get(i) {
            Some(value) => (value & WARP_MASK) == WARP_MASK,
            None => false
        }
    }

    pub fn insert_item(&mut self, map_item: MapItem) {
        self.map_items.insert(map_item.id(), map_item);
    }

    pub fn remove_item(&mut self, map_item: MapItem) {
        self.remove_item_with_id(map_item.id());
    }

    pub fn remove_item_with_id(&mut self, id: u32) {
        self.map_items.remove(&id);
    }

    pub fn get_mob(&self, mob_id: u32) -> Option<&Mob> {
        self.mobs().get(&mob_id)
    }

    pub fn mobs(&self) -> &HashMap<u32, Mob> {
        &self.mobs
    }
    pub fn mobs_mut(&mut self) -> &mut HashMap<u32, Mob> {
        &mut self.mobs
    }
    pub fn map_items(&self) -> &HashMap<u32, MapItem> {
        &self.map_items
    }
    pub fn map_items_mut(&mut self) -> &mut HashMap<u32, MapItem> {
        &mut self.map_items
    }
    pub fn mob_spawns_tracks(&self) -> &Vec<MobSpawnTrack> {
        &self.mob_spawns_tracks
    }
    pub fn mob_spawns_tracks_mut(&mut self) -> &mut Vec<MobSpawnTrack> {
        &mut self.mob_spawns_tracks
    }

    pub fn key(&self) -> &MapInstanceKey {
        &self.key
    }

    pub fn cells(&self) -> &Vec<u16> {
        &self.cells
    }
    pub fn cells_mut(&mut self) -> &mut Vec<u16> {
        &mut self.cells
    }

    pub fn x_size(&self) -> u16 {
        self.x_size
    }
    pub fn y_size(&self) -> u16 {
        self.y_size
    }
}