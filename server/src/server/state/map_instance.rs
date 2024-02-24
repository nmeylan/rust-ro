use std::collections::HashMap;
use models::enums::cell::CellType;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::{MapItem, MapItems, ToMapItem};
use crate::server::state::mob::Mob;
use models::enums::EnumWithMaskValueU16;
use models::item::DroppedItem;


use crate::util::coordinate;
use crate::util::hasher::NoopHasherU32;
use crate::util::vec::VecWithRecycledIndex;

#[derive(SettersAll)]
pub struct MapInstanceState {
    key: MapInstanceKey,
    x_size: u16,
    y_size: u16,
    // index in this array will give x and y position of the cell.
    cells: Vec<u16>,
    map_items: MapItems,
    mobs: VecWithRecycledIndex<Mob>,
    dropped_items: hashbrown::HashMap<u32, DroppedItem, NoopHasherU32>,
    mob_spawns_tracks: HashMap<u32, MobSpawnTrack>,

    mob_movement_paused: bool,
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
    pub fn decrement_spawn(&mut self) {
        self.spawned_amount -= 1;
    }
}

impl MapInstanceState {
    pub fn new(key: MapInstanceKey, x_size: u16, y_size: u16, cells: Vec<u16>,
               map_items: MapItems, mob_spawns_tracks: HashMap<u32, MobSpawnTrack>) -> MapInstanceState {
        Self {
            key,
            x_size,
            y_size,
            cells,
            mobs: VecWithRecycledIndex::new(),
            map_items,
            dropped_items: Default::default(),
            mob_spawns_tracks,
            mob_movement_paused: false
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
            Some(value) => (value & CellType::Warp.as_flag()) == CellType::Warp.as_flag(),
            None => false
        }
    }

    pub fn insert_item(&mut self, map_item: MapItem) {
        self.map_items.insert(map_item);
    }

    pub fn remove_item(&mut self, map_item: MapItem) {
        self.map_items.remove(map_item.generic_internal_id());
    }

    pub fn remove_item_with_id(&mut self, internal_id: u32) {
        self.map_items.remove_with_id(internal_id);
    }

    pub fn get_mob(&self, map_item: &MapItem) -> Option<&Mob> {
        self.mobs().get(map_item.specific_internal_id() as usize)
    }

    pub fn get_mob_with_client_id(&self, client_id: u32) -> Option<&Mob> {
        self.mobs().iter().find(|mob| mob.client_id == client_id)
    }
    pub fn get_mob_mut_with_client_id(&mut self, client_id: u32) -> Option<&mut Mob> {
        self.mobs.get_elements_mut().iter_mut().find(|mob| mob.client_id == client_id)
    }

    pub fn mobs(&self) -> &Vec<Mob> {
        &self.mobs.get_elements()
    }
    pub fn mobs_mut(&mut self) -> &mut Vec<Mob> {
        self.mobs.get_elements_mut()
    }

    pub fn insert_mob(&mut self, mut mob: Mob) {
        self.insert_item(mob.to_map_item());
        let index = self.mobs.get_free_index();
        mob.set_map_instance_mobs_index(index as u32);
        self.mobs.insert(index, mob);
    }

    pub fn remove_mob(&mut self, index: u32) -> Option<Mob>{
        let mob = self.mobs.remove(index as usize);
        self.remove_item(mob.to_map_item());
        Some(mob)
    }
    pub fn get_dropped_item(&self, dropped_item_id: u32) -> Option<&DroppedItem> {
        self.dropped_items().get(&dropped_item_id)
    }
    fn dropped_items(&self) -> &hashbrown::HashMap<u32, DroppedItem, NoopHasherU32> {
        &self.dropped_items
    }
    fn dropped_items_mut(&mut self) -> &mut hashbrown::HashMap<u32, DroppedItem, NoopHasherU32> {
        &mut self.dropped_items
    }
    pub fn insert_dropped_item(&mut self, dropped_item: DroppedItem) {
        self.insert_item(dropped_item.to_map_item());
        self.dropped_items_mut().insert_unique_unchecked(dropped_item.map_item_id, dropped_item);
    }

    pub fn remove_dropped_item(&mut self, id: u32) -> Option<DroppedItem>{
        if let Some(dropped_item) = self.dropped_items_mut().remove(&id) {
            self.remove_item(dropped_item.to_map_item());
            Some(dropped_item)
        } else {
            None
        }
    }

    pub fn map_items(&self) -> &Vec<MapItem> {
        self.map_items.get()
    }

    pub fn map_items_deprecated(&self) -> &hashbrown::HashMap<u32, MapItem, NoopHasherU32> {
        self.map_items.get_deprecated()
    }

    pub fn map_items_mut(&mut self) -> &mut MapItems {
        &mut self.map_items
    }
    pub fn get_map_item_deprecated(&self, item_id: u32) -> Option<&MapItem> {
        self.map_items_deprecated().get(&item_id)
    }
    pub fn mob_spawns_tracks(&self) -> &HashMap<u32, MobSpawnTrack> {
        &self.mob_spawns_tracks
    }
    pub fn mob_spawns_tracks_mut(&mut self) -> &mut HashMap<u32, MobSpawnTrack> {
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

    pub fn mob_movement_paused(&self) -> bool {
        self.mob_movement_paused
    }
}