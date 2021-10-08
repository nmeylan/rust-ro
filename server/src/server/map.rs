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
    // byte 11 -> warp
    pub cells: Option<Vec<u32>>,
    pub is_initialized: bool, // maps initialization is lazy, this bool indicate if maps has been initialized or not
}

pub struct MapPropertyFlags {
    // PARTY - Show attack cursor on non-party members (PvP)
    pub is_party: bool,
    // GUILD - Show attack cursor on non-guild members (GvG)
    pub is_guild: bool,
    // SIEGE - Show emblem over characters heads when in GvG (WoE castle)
    pub is_siege: bool,
    // USE_SIMPLE_EFFECT - Automatically enable /mineffect
    pub use_simple_effect: bool,
    // DISABLE_LOCKON - Only allow attacks on other players with shift key or /ns active
    pub is_no_lockon: bool,
    // COUNT_PK - Show the PvP counter
    pub count_pk: bool,
    // NO_PARTY_FORMATION - Prevents party creation/modification (Might be used for instance dungeons)
    pub party_lock: bool,
    // BATTLEFIELD - Unknown (Does something for battlegrounds areas)
    pub is_battleground: bool,
    // DISABLE_COSTUMEITEM - Disable costume sprites
    pub is_no_costum: bool,
    // USECART - Allow opening cart inventory (Well force it to always allow it)
    pub is_use_cart: bool,
    // SUNMOONSTAR_MIRACLE - Blocks Star Gladiator's Miracle from activating
    pub is_summonstar_miracle: bool,
    // Unused bits. 1 - 10 is 0x1 length and 11 is 0x15 length. May be used for future settings.
    pub unused: bool,
}

impl MapPropertyFlags {
    pub fn new() -> MapPropertyFlags {
        MapPropertyFlags {
            is_party: false,
            is_guild: false,
            is_siege: false,
            use_simple_effect: false,
            is_no_lockon: false,
            count_pk: false,
            party_lock: false,
            is_battleground: false,
            is_no_costum: false,
            is_use_cart: false,
            is_summonstar_miracle: false,
            unused: false
        }
    }

    pub fn raw(&self) -> u32 {
        (((self.is_party as u32) << 0)
            | ((self.is_guild as u32) << 1)
            | ((self.is_siege as u32) << 2)
            | ((self.use_simple_effect as u32) << 3)
            | ((self.is_no_lockon as u32) << 4)
            | ((self.count_pk as u32) << 5)
            | ((self.party_lock as u32) << 6)
            | ((self.is_battleground as u32) << 7)
            | ((self.is_no_costum as u32) << 8)
            | ((self.is_use_cart as u32) << 9)
            | ((self.is_summonstar_miracle as u32) << 10)
            | ((self.unused as u32) << 11))
    }

    pub fn set_use_cart(&mut self, use_cart: bool) {
        self.is_use_cart = use_cart
    }
}

impl Map {
    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        (x as u32 + y as u32 * self.x_size as u32) as usize
    }
    #[inline]
    pub fn get_pos_of(&self, index: u32) -> (u16, u16) {
        let y: u16 = (index / self.x_size as u32) as u16;
        let x: u16 = (index - (y as u32 * self.x_size as u32) as u32) as u16;
        (x, y)
    }

    pub fn is_cell_walkable(&self, x: u16, y: u16) -> bool {
        if self.cells.is_none() {
            warn!("Cannot call is_cell_walkable as cells are not initialized, returning false");
            return false
        }
        (self.cells.as_ref().unwrap().get(self.get_cell_index_of(x, y)).unwrap() & 0b0000_0000_0000_0001) == 0b0000_0000_0000_0001
    }

    pub fn player_join_map(&mut self) {
        if !self.is_initialized {
            self.initialize();
        }
        // TODO maintain a list of player in the map
    }

    fn initialize(&mut self) {
        self.set_cells();
        self.is_initialized = true;
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

        let mut cells: Vec<u32> = Vec::new();
        for cell in map_cache_content_buf {
            cells.push(match cell {
                0 | 2 | 4 | 6 => 3, // 3 => bytes 0 and byte 1 are set. walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 7, // 7 => bytes 0, 1 ,2 are set. walkable water
                5 => 2, // 2 => byte 1 is set gap, (shootable)
                _ => 0
            })
        }
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
                cells: None,
                is_initialized: false
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