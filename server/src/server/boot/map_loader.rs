use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::time::Instant;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;
use crate::server::model::map::{Map, MAP_EXT, WARP_MASK};
use crate::server::model::map_item::MapItem;
use crate::server::model::mob_spawn::MobSpawn;
use crate::server::model::script::Script;
use crate::server::model::warp::Warp;
use crate::util::coordinate;

pub struct MapLoader;

static MAPCACHE_EXT: &str = ".mcache";
static MAP_DIR: &str = "./config/maps/pre-re";
struct Header {
    #[allow(dead_code)]
    pub version: i16,
    #[allow(dead_code)]
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}


impl MapLoader {

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

            let mut map = Map::new(
                header.x_size as u16,
                header.y_size as u16,
                header.length,
                map_name.to_string(),
                Self::name_with_ext(map_name.as_str()),
                Default::default(),
                Default::default(),
                Default::default(),
            );
            map.set_warps(warps.get(&map_name).unwrap_or(&vec![]), map_items);
            map.set_mob_spawns(mob_spawns.get(&map_name).unwrap_or(&vec![]));
            map.set_scripts(scripts.get(&map_name).unwrap_or(&vec![]), map_items);
            maps.insert(map.name().to_string(), map);
        }
        maps
    }

    // This method is called each time a new instance is created. If we load this during boot it comes with 2 drawback:
    // - It slow done startup (yet it can be improved)
    // - We may store in memory cells for map that are not visited by player
    pub fn generate_cells(name: &str, length: usize) -> Vec<u16> {
        let file_path = Path::join(Path::new(MAP_DIR), format!("{}{}", name, MAPCACHE_EXT));
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut map_cache_zip_content_buf = Vec::new();
        let mut map_cache_content_buf = Vec::new();
        reader.read_to_end(&mut map_cache_zip_content_buf).unwrap_or_else(|_| panic!("Fail to read map-cache zip content for map: {}", name));
        let mut decoder = ZlibDecoder::new(&map_cache_zip_content_buf[26..]); // skip header
        decoder.read_to_end(&mut map_cache_content_buf).unwrap_or_else(|_| panic!("Fail to read map-cache unzipped content for map: {}", name));

        let mut cells: Vec<u16> = Vec::with_capacity(length as usize);
        for cell in map_cache_content_buf {
            cells.push(match cell {
                0 | 2 | 4 | 6 => 3, // 3 => bytes 0 and byte 1 are set. walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 7, // 7 => bytes 0, 1 ,2 are set. walkable water
                5 => 2, // 2 => byte 1 is set gap, (shootable)
                _ => 0
            })
        }

        cells
    }




    fn name_with_ext(map_name: &str) -> String {
        if !map_name.ends_with(MAP_EXT) {
            format!("{}{}", map_name, MAP_EXT)
        } else {
            map_name.to_string()
        }
    }
}