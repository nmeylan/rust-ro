use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use std::time::Instant;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::server::model::map::{MAP_EXT, Map};
use crate::server::model::map_item::MapItems;
use crate::server::model::mob_spawn::MobSpawn;
use crate::server::model::script::Script;
use crate::server::model::warp::Warp;

pub struct MapLoader;

static MAPCACHE_EXT: &str = ".mcache";
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
    pub fn load_maps(
        warps: HashMap<String, Vec<Warp>>,
        mob_spawns: HashMap<String, Vec<MobSpawn>>,
        scripts: HashMap<String, Vec<Script>>,
        map_items: &mut MapItems,
        map_dir: &'static str,
    ) -> HashMap<String, Map> {
        let mut maps = HashMap::<String, Map>::new();
        let paths = fs::read_dir(map_dir).unwrap();
        for path in paths {
            let _start = Instant::now();
            let path = path.as_ref().unwrap();
            let map_name = path.file_name().to_str().unwrap().replace(MAPCACHE_EXT, "");
            let file = File::open(path.path()).unwrap_or_else(|_| panic!("Can't open file for map: {map_name}"));
            let mut reader = BufReader::new(file);
            let mut buf = [0_u8; 26];
            reader
                .read_exact(&mut buf)
                .unwrap_or_else(|_| panic!("Can't read file for map: {map_name}"));
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

    // This method is called each time a new instance is created. If we load this
    // during boot it comes with 2 drawback:
    // - It slow done startup (yet it can be improved)
    // - We may store in memory cells for map that are not visited by player
    pub fn generate_cells(name: &str, _length: usize, map_dir: &'static str) -> Vec<u16> {
        map_cache::read_mcache(Path::new(map_dir), name)
            .unwrap_or_else(|e| panic!("{e}"))
            .cells
    }

    fn name_with_ext(map_name: &str) -> String {
        if !map_name.ends_with(MAP_EXT) {
            format!("{map_name}{MAP_EXT}")
        } else {
            map_name.to_string()
        }
    }
}
