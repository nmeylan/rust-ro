
use models::enums::cell::CellType;
use models::enums::EnumWithMaskValueU16;
use crate::server::model::path::{allowed_dirs, DIR_EAST, DIR_NORTH, DIR_SOUTH, DIR_WEST, is_direction};
use crate::server::model::map_item::{MapItems, ToMapItem};
use crate::server::model::mob_spawn::MobSpawn;
use crate::server::model::script::Script;
use crate::server::model::warp::Warp;




use crate::util::coordinate;



pub static MAP_EXT: &str = ".gat";
pub const RANDOM_CELL: (u16, u16) = (u16::MAX, u16::MAX);

pub struct Map {
    x_size: u16,
    y_size: u16,
    length: i32,
    name: String,
    name_with_ext: String,
    warps: Vec<Warp>,
    mob_spawns: Vec<MobSpawn>,
    scripts: Vec<Script>,
}

impl Map {
    pub fn new(x_size: u16, y_size: u16, length: i32, name: String, name_with_ext: String, warps: Vec<Warp>, mob_spawns: Vec<MobSpawn>, scripts: Vec<Script>) -> Self {
        Self { x_size, y_size, length, name, name_with_ext, warps, mob_spawns, scripts }
    }
    pub fn find_random_walkable_cell(cells: &Vec<u16>, x_size: u16) -> (u16, u16) {
        let mut rng = fastrand::Rng::new();

        loop {
            let index = rng.usize(0..cells.len());
            if cells.get(index).unwrap() & CellType::Walkable.as_flag() == 1 {
                return coordinate::get_pos_of(index as u32, x_size);
            }
        }
    }
    pub fn find_random_free_cell_around(cells: &Vec<u16>, x_size: u16, x: u16, y :u16) -> (u16, u16) {
        let mut rng = fastrand::Rng::new();

        loop {
            let random_x = rng.u16((x.max(1) - 1)..(x.max(1) + 1));
            let random_y = rng.u16((y.max(1) - 1)..(y.max(1) + 1));
            let index = coordinate::get_cell_index_of(random_x, random_y, x_size);
            if cells.get(index).unwrap_or_else(||panic!("Expected cell at index {index} to exist")) & CellType::Walkable.as_flag() == 1 {
                return (random_x, random_y);
            }
        }
    }

    pub fn find_random_walkable_cell_in_max_range(cells: &[u16], x_size: u16, y_size: u16, x: u16, y: u16, max_range: usize) -> Option<(u16, u16)> {
        let mut rng = fastrand::Rng::new();
        let max_range = max_range as u16;
        let allowed_dirs = allowed_dirs(x_size, y_size, x, y);
        let mut directions = vec![DIR_NORTH, DIR_SOUTH, DIR_EAST, DIR_WEST, DIR_SOUTH | DIR_EAST, DIR_SOUTH | DIR_WEST, DIR_NORTH | DIR_EAST, DIR_NORTH | DIR_WEST];
        let mut dest_x = x;
        let mut dest_y = y;
        let max_iter = 3;
        let mut i = 0;
        loop {
            let random_index = rng.usize(0..directions.len());
            let direction = *directions.get(random_index).unwrap();
            if is_direction(allowed_dirs, direction) {
                loop {
                    if i > max_iter {
                        return None;
                    }
                    let mut random_x = rng.u16(0..=max_range);
                    if x < random_x {
                        random_x = rng.u16(0..=x);
                    }
                    let mut random_y = rng.u16(0..=max_range);
                    if y < random_y {
                        random_y = rng.u16(0..=y);
                    }
                    if direction == DIR_NORTH {
                        dest_y = y + random_y;
                    } else if direction == DIR_SOUTH {
                        dest_y = y - random_y;
                    } else if direction == DIR_EAST {
                        dest_x = x + random_x;
                    } else if direction == DIR_WEST {
                        dest_x = x - random_x;
                    } else if direction == DIR_SOUTH | DIR_EAST {
                        dest_y = y - random_y;
                        dest_x = x + random_x;
                    } else if direction == DIR_SOUTH | DIR_WEST {
                        dest_y = y - random_y;
                        dest_x = x - random_x;
                    } else if direction == DIR_NORTH | DIR_EAST {
                        dest_y = y + random_y;
                        dest_x = x + random_x;
                    } else if direction == DIR_NORTH | DIR_WEST {
                        dest_y = y + random_y;
                        dest_x = x - random_x;
                    }
                    if dest_x >= x_size {
                        dest_x = x_size - 1;
                    }
                    if dest_y >= y_size {
                        dest_y = y_size - 1;
                    }
                    if cells.get(coordinate::get_cell_index_of(dest_x, dest_y, x_size)).unwrap() & CellType::Walkable.as_flag() == 1 {
                        return Some((dest_x, dest_y));
                    }
                    i += 1;
                }
            } else {
                directions.swap_remove(random_index);
            }
        }
    }

    pub fn set_warp_cells(&self, cells: &mut [u16], map_items: &mut MapItems) {
        for warp in self.warps.iter() {
            map_items.insert_deprecated(warp.id, warp.to_map_item());
            let start_x = warp.x - warp.x_size;
            let to_x = warp.x + warp.x_size;
            let start_y = warp.y - warp.y_size;
            let to_y = warp.y + warp.y_size;
            for x in start_x..to_x {
                for y in start_y..to_y {
                    let index = coordinate::get_cell_index_of(x, y, self.x_size);
                    let cell = cells.get_mut(index).unwrap();
                    *cell |= CellType::Warp.as_flag();
                }
            }
        }
    }



    pub fn set_warps(&mut self, warps: &[Warp], map_item_ids: &mut MapItems) {
        let warps = warps.iter().map(|warp| {
            let mut warp = warp.clone();
            warp.set_id(map_item_ids.generate_internal_id_deprecated());
            warp
        }).collect::<Vec<Warp>>();
        self.warps = warps;
    }

    pub fn set_mob_spawns(&mut self, mob_spawns: &[MobSpawn]) {
        self.mob_spawns = mob_spawns.to_vec();
    }

    pub fn set_scripts(&mut self, scripts: &[Script], map_item_ids: &mut MapItems) {
        self.scripts =
            scripts.iter().map(|script| {
                let mut script = script.clone();
                script.set_id(map_item_ids.generate_internal_id_deprecated());
                script
            }).collect::<Vec<Script>>();
    }

    pub fn name_without_ext(map_name: &str) -> String {
        map_name.replace(MAP_EXT, "")
    }


    pub fn x_size(&self) -> u16 {
        self.x_size
    }
    pub fn y_size(&self) -> u16 {
        self.y_size
    }
    pub fn length(&self) -> i32 {
        self.length
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn name_with_ext(&self) -> &str {
        &self.name_with_ext
    }
    pub fn warps(&self) -> &Vec<Warp> {
        &self.warps
    }
    pub fn mob_spawns(&self) -> &Vec<MobSpawn> {
        &self.mob_spawns
    }
    pub fn scripts(&self) -> &Vec<Script> {
        &self.scripts
    }
}