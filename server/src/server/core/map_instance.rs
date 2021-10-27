use std::sync::{Arc, RwLock};
use crate::server::core::map::{Map};
use crate::server::core::mob::Mob;
use crate::server::npc::mob::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::Server;


pub struct MapInstance {
    pub name: String,
    pub id: u32,
    pub x_size: u16,
    pub y_size: u16,
    pub cells: Arc<Vec<u16>>,
    pub warps: Arc<Vec<Arc<Warp>>>,
    pub mob_spawns: Arc<Vec<Arc<MobSpawn>>>,
    pub mob_spawns_tracks: Vec<MobSpawnTrack>,
    pub mobs: Vec<Arc<RwLock<Mob>>>,
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
    pub fn from_map(map: &Map, id: u32) -> MapInstance {
        MapInstance {
            name: map.name.clone(),
            id,
            x_size: map.x_size.clone(),
            y_size: map.y_size.clone(),
            cells: map.cells.clone(),
            warps: map.warps.clone(),
            mob_spawns: map.mob_spawns.clone(),
            mob_spawns_tracks: map.mob_spawns.iter().map(|spawn| MobSpawnTrack::default(spawn.id.clone())).collect::<Vec<MobSpawnTrack>>(),
            mobs: Default::default()
        }
    }

    pub fn spawn_mobs(&mut self, server: Arc<Server>, now: u128) {
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
                let mob = Mob::new(server.generate_map_item_id(), cell.0, cell.1, mob_spawn.mob_id, mob_spawn.id, mob_spawn.name.clone());
                info!("Spawned {} at {},{})", mob_spawn.name, cell.0, cell.1);
                self.mobs.push(Arc::new(RwLock::new(mob)));
                mob_spawn_track.increment_spawn();
            }
            info!("Spawned {} {} (spawn id {})", spawned, mob_spawn.name, mob_spawn.id);
        }
    }

    pub fn get_mob_at(&self, x: u16, y: u16) -> Option<Arc<RwLock<Mob>>> {
        let option = self.mobs.iter().find(|mob| {
            let mob_guard = read_lock!(mob);
            mob_guard.x == x && mob_guard.y == y
        });
        match option {
            Some(e) => Some(e.clone()),
            None => None
        }
    }
}