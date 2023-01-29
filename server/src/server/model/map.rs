use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::convert::TryInto;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use std::{fs};

use std::time::{Instant};
use std::collections::{HashMap};

use crate::server::model::path::{allowed_dirs, DIR_EAST, DIR_NORTH, DIR_SOUTH, DIR_WEST, is_direction};
use crate::server::model::map_item::{MapItem, ToMapItem};
use crate::server::model::mob_spawn::MobSpawn;
use crate::server::model::script::Script;
use crate::server::model::warp::Warp;

use crate::server::Server;


use crate::util::coordinate;


static MAPCACHE_EXT: &str = ".mcache";
static MAP_DIR: &str = "./maps/pre-re";
pub static MAP_EXT: &str = ".gat";
pub const WARP_MASK: u16 = 0b0000_0100_0000_0000;
pub const WALKABLE_MASK: u16 = 0b0000000000000001;
pub const RANDOM_CELL: (u16, u16) = (u16::MAX, u16::MAX);

struct Header {
    #[allow(dead_code)]
    pub version: i16,
    #[allow(dead_code)]
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}

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
    pub fn find_random_walkable_cell(cells: &Vec<u16>, x_size: u16) -> (u16, u16) {
        let rng = fastrand::Rng::new();

        loop {
            let index = rng.usize(0..cells.len());
            if cells.get(index).unwrap() & WALKABLE_MASK == 1 {
                return coordinate::get_pos_of(index as u32, x_size);
            }
        }
    }

    pub fn find_random_walkable_cell_in_max_range(cells: &[u16], x_size: u16, y_size: u16, x: u16, y: u16, max_range: usize) -> Option<(u16, u16)> {
        let rng = fastrand::Rng::new();
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
                    if cells.get(coordinate::get_cell_index_of(dest_x, dest_y, x_size)).unwrap() & WALKABLE_MASK == 1 {
                        return Some((dest_x, dest_y));
                    }
                    i += 1;
                }
            } else {
                directions.swap_remove(random_index);
            }
        }
    }

    pub fn generate_cells(&self, map_items: &mut HashMap<u32, MapItem>) -> Vec<u16> {
        let file_path = Path::join(Path::new(MAP_DIR), format!("{}{}", self.name, MAPCACHE_EXT));
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut map_cache_zip_content_buf = Vec::new();
        let mut map_cache_content_buf = Vec::new();
        reader.read_to_end(&mut map_cache_zip_content_buf).unwrap_or_else(|_| panic!("Fail to read map-cache zip content for map: {}", self.name));
        let mut decoder = ZlibDecoder::new(&map_cache_zip_content_buf[26..]); // skip header
        decoder.read_to_end(&mut map_cache_content_buf).unwrap_or_else(|_| panic!("Fail to read map-cache unzipped content for map: {}", self.name));

        let mut cells: Vec<u16> = Vec::with_capacity(self.length as usize);
        for cell in map_cache_content_buf {
            cells.push(match cell {
                0 | 2 | 4 | 6 => 3, // 3 => bytes 0 and byte 1 are set. walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 7, // 7 => bytes 0, 1 ,2 are set. walkable water
                5 => 2, // 2 => byte 1 is set gap, (shootable)
                _ => 0
            })
        }

        self.set_warp_cells(&mut cells, map_items);
        cells
    }

    fn set_warp_cells(&self, cells: &mut [u16], map_items: &mut HashMap<u32, MapItem>) {
        for warp in self.warps.iter() {
            map_items.insert(warp.id, warp.to_map_item());
            let start_x = warp.x - warp.x_size;
            let to_x = warp.x + warp.x_size;
            let start_y = warp.y - warp.y_size;
            let to_y = warp.y + warp.y_size;
            for x in start_x..to_x {
                for y in start_y..to_y {
                    let index = coordinate::get_cell_index_of(x, y, self.x_size);
                    let cell = cells.get_mut(index).unwrap();
                    *cell |= WARP_MASK;
                }
            }
        }
    }

    fn set_warps(&mut self, warps: &[Warp], map_item_ids: &mut HashMap<u32, MapItem>) {
        let warps = warps.iter().map(|warp| {
            let mut warp = warp.clone();
            warp.set_id(Server::generate_id(map_item_ids));
            warp
        }).collect::<Vec<Warp>>();
        self.warps = warps;
    }

    fn set_mob_spawns(&mut self, mob_spawns: &[MobSpawn]) {
        self.mob_spawns = mob_spawns.to_vec();
    }

    fn set_scripts(&mut self, scripts: &[Script], map_item_ids: &mut HashMap<u32, MapItem>) {
        self.scripts =
            scripts.iter().map(|script| {
                let mut script = script.clone();
                script.set_id(Server::generate_id(map_item_ids));
                script
            }).collect::<Vec<Script>>();
    }

    pub fn load_maps(warps: HashMap<String, Vec<Warp>>, mob_spawns: HashMap<String, Vec<MobSpawn>>, scripts: HashMap<String, Vec<Script>>, map_items: &mut HashMap<u32, MapItem>) -> HashMap<String, Map> {
        let mut maps = HashMap::<String, Map>::new();
        let paths = fs::read_dir(MAP_DIR).unwrap();
        for path in paths {
            let _start = Instant::now();
            let path = path.as_ref().unwrap();
            let map_name = path.file_name().to_str().unwrap().replace(MAPCACHE_EXT, "");
            let file = File::open(path.path()).unwrap_or_else(|_| panic!("Can't open file for map: {}", map_name));
            let mut reader = BufReader::new(file);
            let mut buf = [0_u8; 26];
            reader.read_exact(&mut buf).unwrap_or_else(|_| panic!("Can't read file for map: {}", map_name));
            let header = Header {
                version: Cursor::new(buf[0..2].to_vec()).read_i16::<LittleEndian>().unwrap(),
                checksum: buf[2..18].try_into().unwrap(),
                x_size: Cursor::new(buf[18..20].to_vec()).read_i16::<LittleEndian>().unwrap(),
                y_size: Cursor::new(buf[20..22].to_vec()).read_i16::<LittleEndian>().unwrap(),
                length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap(),
            };
            // TODO validate checksum
            // TODO validate size + length

            let mut map = Map {
                x_size: header.x_size as u16,
                y_size: header.y_size as u16,
                length: header.length,
                name: map_name.to_string(),
                name_with_ext: Self::_name_with_ext(map_name.as_str()),
                warps: Default::default(),
                mob_spawns: Default::default(),
                scripts: Default::default(),
            };
            map.set_warps(warps.get(&map_name).unwrap_or(&vec![]), map_items);
            map.set_mob_spawns(mob_spawns.get(&map_name).unwrap_or(&vec![]));
            map.set_scripts(scripts.get(&map_name).unwrap_or(&vec![]), map_items);
            maps.insert(map.name.clone(), map);
        }
        maps
    }

    pub fn name_without_ext(map_name: &str) -> String {
        map_name.replace(MAP_EXT, "")
    }

    fn _name_with_ext(map_name: &str) -> String {
        if !map_name.ends_with(MAP_EXT) {
            format!("{}{}", map_name, MAP_EXT)
        } else {
            map_name.to_string()
        }
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