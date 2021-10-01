use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::convert::TryInto;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::{Decompress, FlushDecompress};
use flate2::read::ZlibDecoder;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use log::warn;

static MAP_EXT: &str = ".mcache";
static MAP_DIR: &str = "./maps/pre-re";

struct Header {
    pub version: i16,
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}

#[derive(Debug)]
pub struct Map {
    pub x_size: u16,
    pub y_size: u16,
    pub length: i32,
    pub name: String,
    // index in this array will give x and y position of the cell.
    // 2 bytes representing cell type:
    // byte 0 -> walkable
    // byte 1 -> shootable
    // byte 2 -> water
    // byte 3 -> npc
    // byte 4 -> basilica
    // byte 5 -> landprotector
    // byte 6 -> novending
    // byte 7 -> nochat
    // byte 8 -> icewall
    // byte 9 -> noicewall
    // byte 10 -> noskill
    pub cells: Option<Vec<u16>>
}

impl Map {
    pub fn get_cell_index_of(&self, x: i16, y: i16) -> usize {
        (x as u32 + y as u32 * self.x_size as u32) as usize
    }
    pub fn get_pos_of(&self, index: u32) -> (i16, i16) {
        let y: i16 = (index / self.x_size as u32) as i16;
        let x: i16 = (index - (y as u32 * self.x_size as u32) as u32) as i16;
        (x, y)
    }

    pub fn is_cell_walkable(&self, x: i16, y: i16) -> bool {
        if self.cells.is_none() {
            warn!("Cannot call is_cell_walkable as cells are not initialized, returning false");
            return false
        }
        (self.cells.as_ref().unwrap().get(self.get_cell_index_of(x, y)).unwrap() & 0b0000_0000_0000_0001) == 0b0000_0000_0000_0001
    }

    pub fn player_join_map(&mut self) {
        if self.cells.is_none() {
            self.set_cells();
        }
        // TODO maintain a list of player in the map
    }

    pub fn set_cells(&mut self) {
        let file_path = Path::join(Path::new(MAP_DIR), format!("{}{}", self.name, MAP_EXT));
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut map_cache_zip_content_buf = Vec::new();
        let mut map_cache_content_buf = Vec::new();
        reader.read_to_end(&mut map_cache_zip_content_buf);
        let mut decoder = ZlibDecoder::new(&map_cache_zip_content_buf[26..]); // skip header
        decoder.read_to_end(&mut map_cache_content_buf).unwrap();

        let mut cells: Vec<u16> = Vec::new();
        for cell in map_cache_content_buf {
            cells.push(match cell {
                0 | 2 | 4 | 6 => 0b0000_0000_0000_0011, // walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 0b0000_0000_0000_0111, // walkable water
                5 => 0b0000_0000_0000_0010, // gap, (shootable)
                _ => 0
            })
        }
        println!("{:?}", cells);
        self.cells = Some(cells);
    }

    pub fn load_maps() -> HashMap<String, Map> {
        let mut maps = HashMap::<String, Map>::new();
        let paths = fs::read_dir(MAP_DIR).unwrap();
        for path in paths {
            let start = Instant::now();
            let path = path.as_ref().unwrap();
            let map_name = path.file_name().to_str().unwrap().replace(MAP_EXT, "");
            let file = File::open(path.path()).unwrap();
            let mut reader = BufReader::new(file);
            let mut buf = [0 as u8; 26];
            reader.read(&mut buf);
            let header = Header {
                version: Cursor::new(buf[0..2].to_vec()).read_i16::<LittleEndian>().unwrap(),
                checksum: buf[2..18].try_into().unwrap(),
                x_size: Cursor::new(buf[18..20].to_vec()).read_i16::<LittleEndian>().unwrap(),
                y_size: Cursor::new(buf[20..22].to_vec()).read_i16::<LittleEndian>().unwrap(),
                length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap()
            };
            // TODO validate checksum
            // TODO validate size + length


            let map = Map {
                x_size: header.x_size as u16,
                y_size: header.y_size as u16,
                length: header.length,
                name: map_name.to_string(),
                cells: None
            };
            println!("Map {} loaded in {} secs", map.name, start.elapsed().as_millis() as f32 / 1000.0);
            maps.insert(map.name.clone(), map);
        }
        maps
    }

    pub fn name_without_ext(map_name: String) -> String {
        map_name.replace(".gat", "")
    }
}