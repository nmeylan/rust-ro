use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::convert::TryInto;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::read::ZlibDecoder;
use std::{fs, thread};
use std::any::Any;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc};
use std::thread::sleep;
use parking_lot::RwLock;
use rand::Rng;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use accessor::Setters;
use crate::server::core::character::CharacterSession;
use crate::server::core::map_instance::MapInstance;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
use crate::util::coordinate;

static MAPCACHE_EXT: &str = ".mcache";
static MAP_DIR: &str = "./maps/pre-re";
pub static MAP_EXT: &str = ".gat";
pub const WARP_MASK: u16 = 0b0000_0100_0000_0000;

struct Header {
    pub version: i16,
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}

pub struct Map {
    pub x_size: u16,
    pub y_size: u16,
    pub length: i32,
    pub name: String,
    pub warps: Arc<Vec<Arc<Warp>>>,
    pub mob_spawns: Arc<Vec<Arc<MobSpawn>>>,
    pub map_thread_channel_sender: Sender<String>,
    pub map_instances: RwLock<Vec<Arc<MapInstance>>>
}

pub trait MapItem: Send + Sync {
    fn id(&self) -> u32;
    fn client_item_class(&self) -> i16;
    fn object_type(&self) -> i16;
    fn name(&self) -> String;
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Setters)]
pub struct MapPropertyFlags {
    // PARTY - Show attack cursor on non-party members (PvP)
    #[set]
    pub is_party: bool,
    // GUILD - Show attack cursor on non-guild members (GvG)
    #[set]
    pub is_guild: bool,
    // SIEGE - Show emblem over characters heads when in GvG (WoE castle)
    #[set]
    pub is_siege: bool,
    // USE_SIMPLE_EFFECT - Automatically enable /mineffect
    #[set]
    pub use_simple_effect: bool,
    // DISABLE_LOCKON - Only allow attacks on other players with shift key or /ns active
    #[set]
    pub is_no_lockon: bool,
    // COUNT_PK - Show the PvP counter
    #[set]
    pub count_pk: bool,
    // NO_PARTY_FORMATION - Prevents party creation/modification (Might be used for instance dungeons)
    #[set]
    pub party_lock: bool,
    // BATTLEFIELD - Unknown (Does something for battlegrounds areas)
    #[set]
    pub is_battleground: bool,
    // DISABLE_COSTUMEITEM - Disable costume sprites
    #[set]
    pub is_no_costum: bool,
    // USECART - Allow opening cart inventory (Well force it to always allow it)
    #[set]
    pub is_use_cart: bool,
    // SUNMOONSTAR_MIRACLE - Blocks Star Gladiator's Miracle from activating
    #[set]
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
        ((self.is_party as u32) << 0)
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
            | ((self.unused as u32) << 11)
    }
}

impl Map {
    // TODO: implement map instance. This method should return map instance id (or ref) for char session.
    // Char interact with instance instead of map directly.
    // Instances will make map lifecycle easier to maintain
    // Only 1 instance will be needed for most use case, but it make possible to wipe map instance after a while when no player are on it. to free memory
    pub fn player_join_map(&self, char_session: &CharacterSession, server: Arc<Server>) -> Arc<MapInstance> {

        let map_instance_id = 0_u32;
        let mut instance_exists = false; {
            let map_instances_guard = read_lock!(self.map_instances);
            instance_exists = map_instances_guard.get(map_instance_id as usize).is_some();
        }
        if !instance_exists {
            self.create_map_instance(server, map_instance_id);
        }
        let map_instances_guard = read_lock!(self.map_instances);
        let map_instance = map_instances_guard.get(map_instance_id as usize).unwrap();
        map_instance.clone()
    }

    pub fn find_random_walkable_cell(cells: &Vec<u16>, x_size: u16) -> (u16, u16) {
        let mut rng = rand::thread_rng();

        loop {
            let index = rng.gen_range(0..cells.len());
            if cells.get(index).unwrap() & 0000000000000001 == 1 {
                return coordinate::get_pos_of(index as u32, x_size)
            }
        }
    }

    fn create_map_instance(&self, server: Arc<Server>, instance_id: u32) -> Arc<MapInstance> {
        let cells = self.generate_cells(server.clone());
        let map_instance = MapInstance::from_map(&self, instance_id, cells);
        let map_instance_ref = Arc::new(map_instance);
        {
            let mut map_instance_guard = write_lock!(self.map_instances);
            map_instance_guard.push(map_instance_ref.clone());
        }
        Map::start_thread(map_instance_ref.clone(), self.map_thread_channel_sender.subscribe(), server);
        map_instance_ref.clone()
    }

    pub fn generate_cells(&self, server: Arc<Server>) -> Vec<u16> {
        let file_path = Path::join(Path::new(MAP_DIR), format!("{}{}", self.name, MAPCACHE_EXT));
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
                0 | 2 | 4 | 6 => 3, // 3 => bytes 0 and byte 1 are set. walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
                1 => 0, // no walkable ground
                3 => 7, // 7 => bytes 0, 1 ,2 are set. walkable water
                5 => 2, // 2 => byte 1 is set gap, (shootable)
                _ => 0
            })
        }

        self.set_warp_cells(&mut cells, server);
        cells
    }

    fn set_warp_cells(&self, cells: &mut Vec<u16>, server: Arc<Server>) {
        for warp in self.warps.iter() {
            server.insert_map_item(warp.id, warp.clone());
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

    fn set_warps(&mut self, warps: &Vec<Warp>, map_item_ids: &RwLock<HashMap<u32, Arc<dyn MapItem>>>) {
        let mut ids_write_guard = write_lock!(map_item_ids);
        let warps = warps.iter().map(|warp| {
            let mut warp = warp.clone();
            warp.set_id(Server::generate_id(&mut ids_write_guard));
            let warp_ref = Arc::new(warp);
            warp_ref
        }).collect::<Vec<Arc<Warp>>>();
        self.warps = Arc::new(warps);
    }

    fn set_mob_spawns(&mut self, mob_spawns: &Vec<MobSpawn>) {
        self.mob_spawns = Arc::new(
            mob_spawns.iter().map(|mob_spawn| Arc::new(mob_spawn.clone())).collect::<Vec<Arc<MobSpawn>>>()
        );
    }

    fn start_thread(map_instance: Arc<MapInstance>, mut rx: Receiver<String>, server: Arc<Server>) {
        let map_instance_clone = map_instance.clone();
        let map_instance_clone_for_thread = map_instance.clone();
        info!("Start thread for {}", map_instance_clone.name);
        thread::Builder::new().name(format!("{}-thread", map_instance_clone.name))
            .spawn(move || {
                let now = Instant::now();
                let mut cleanup_notified_at: Option<Instant> = None;
                while cleanup_notified_at.is_none() || now.clone().duration_since(cleanup_notified_at.unwrap()).as_secs() < 5 {
                    if rx.try_recv().is_ok() {
                        info!("received clean up sig");
                        cleanup_notified_at = Some(now.clone());
                    }
                    {
                        map_instance_clone_for_thread.spawn_mobs(server.clone(), now.clone().elapsed().as_millis(), map_instance.clone());
                        map_instance_clone_for_thread.update_mob_fov();
                    }
                    sleep(Duration::from_millis(200));
                }
                info!("Clean up {} map", map_instance_clone.name);
            });
    }

    pub fn load_maps(warps: HashMap<String, Vec<Warp>>, mob_spawns: HashMap<String, Vec<MobSpawn>>, map_items: &RwLock<HashMap<u32, Arc<dyn MapItem>>>) -> HashMap<String, Map> {
        let mut maps = HashMap::<String, Map>::new();
        let paths = fs::read_dir(MAP_DIR).unwrap();
        for path in paths {
            let _start = Instant::now();
            let path = path.as_ref().unwrap();
            let map_name = path.file_name().to_str().unwrap().replace(MAPCACHE_EXT, "");
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

            let (tx, _) = broadcast::channel::<String>(32);
            let mut map = Map {
                x_size: header.x_size as u16,
                y_size: header.y_size as u16,
                length: header.length,
                name: map_name.to_string(),
                warps: Default::default(),
                mob_spawns: Default::default(),
                map_thread_channel_sender: tx,
                map_instances: Default::default(),
            };
            map.set_warps(warps.get(&map_name).unwrap_or(&vec![]), map_items);
            map.set_mob_spawns(mob_spawns.get(&map_name).unwrap_or(&vec![]));
            maps.insert(map.name.clone(), map);
        }
        maps
    }

    pub fn name_without_ext(map_name: String) -> String {
        map_name.replace(MAP_EXT, "")
    }
}