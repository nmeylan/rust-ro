use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
use std::convert::TryInto;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::read::ZlibDecoder;
use std::{fs, thread};
use std::any::Any;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::slice::Iter;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicI8;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::Receiver;
use std::thread::sleep;
use rand::Rng;
use accessor::Setters;
use crate::Script;
use crate::server::core::character::Character;
use crate::server::core::map_event::MapEvent;
use crate::server::core::map_instance::MapInstance;
use crate::server::core::path::{allowed_dirs, DIR_EAST, DIR_NORTH, DIR_SOUTH, DIR_WEST, is_direction};
use crate::server::enums::map_item::MapItemType;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
use crate::util::cell::MyUnsafeCell;
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
    pub x_size: u16,
    pub y_size: u16,
    pub length: i32,
    pub name: String,
    pub warps: Arc<Vec<Arc<Warp>>>,
    pub mob_spawns: Arc<Vec<Arc<MobSpawn>>>,
    pub scripts: Arc<Vec<Script>>,
    pub map_instances: MyUnsafeCell<Vec<Arc<MapInstance>>>,
    pub map_instances_count: AtomicI8,
}

#[derive(Debug, Copy, Clone)]
pub struct MapItem {
    id: u32,
    client_item_class: i16,
    object_type: MapItemType,
}

impl MapItem {
    pub const fn unknown() -> Self {
        Self {
            id: 0,
            client_item_class: 0,
            object_type: MapItemType::Unknown,
        }
    }
    pub fn new(id: u32, client_item_class: i16, object_type: MapItemType) -> Self {
        Self {
            id,
            client_item_class,
            object_type,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn client_item_class(&self) -> i16 {
        self.client_item_class
    }
    pub fn object_type(&self) -> &MapItemType {
        &self.object_type
    }
    pub fn object_type_value(&self) -> i16 {
        self.object_type.value()
    }
}

pub trait ToMapItem {
    fn to_map_item(&self) -> MapItem;
}

impl Hash for MapItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq<Self> for MapItem {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for MapItem{}

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
        (self.is_party as u32)
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
    // Char interact with instance instead of map directly.
    // Instances will make map lifecycle easier to maintain
    // Only 1 instance will be needed for most use case, but it make possible to wipe map instance after a while when no player are on it. to free memory
    pub fn player_join_map(&self, server: &Server) -> Arc<MapInstance> {
        let map_instance_id = 0_u8;
        let instance_exists;
        {
            let map_instances_guard = self.map_instances.borrow();
            instance_exists = map_instances_guard.get(map_instance_id as usize).is_some();
        }
        if !instance_exists {
            self.create_map_instance(server, map_instance_id);
        }
        let map_instances_guard =self.map_instances.borrow();
        let map_instance = map_instances_guard.get(map_instance_id as usize).unwrap();
        map_instance.clone()
    }

    pub fn get_instance(&self, id: u8, server: &Server) -> Option<Arc<MapInstance>>{
        for map_instance in  self.map_instances.borrow().iter() {
            if map_instance.id == id {
                return Some(map_instance.clone())
            }
        }
        Some(self.create_map_instance(server, id))
    }
    pub fn instances(&self) -> Vec<Arc<MapInstance>> {
        self.map_instances.borrow().iter().map(|instance| instance.clone()).collect()
    }

    pub fn find_random_walkable_cell(cells: &Vec<u16>, x_size: u16) -> (u16, u16) {
        let mut rng = rand::thread_rng();

        loop {
            let index = rng.gen_range(0..cells.len());
            if cells.get(index).unwrap() & WALKABLE_MASK == 1 {
                return coordinate::get_pos_of(index as u32, x_size)
            }
        }
    }

    pub fn find_random_walkable_cell_in_max_range(cells: &[u16], x_size: u16, y_size: u16, x: u16, y: u16, max_range: usize) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        let allowed_dirs = allowed_dirs(x_size, y_size, x, y);
        let mut directions = vec![DIR_NORTH, DIR_SOUTH, DIR_EAST, DIR_WEST, DIR_SOUTH | DIR_EAST, DIR_SOUTH | DIR_WEST, DIR_NORTH | DIR_EAST, DIR_NORTH | DIR_WEST];
        let mut dest_x = x;
        let mut dest_y = y;
        // TODO maybe control infinite loop, max 5 iter
        loop {
            let random_index = rng.gen_range(0..directions.len());
            let direction = *directions.get(random_index).unwrap();
            if is_direction(allowed_dirs, direction) {
                loop {
                    let random_x = rng.gen_range(0..=max_range) as u16;
                    let random_y = rng.gen_range(0..=max_range) as u16;
                    if direction == DIR_NORTH {
                        dest_y = y + random_y;
                    } else if direction == DIR_SOUTH {
                        dest_y = y - random_y;
                    } else if direction == DIR_EAST {
                        dest_x = x + random_x;
                    }  else if direction == DIR_WEST {
                        dest_x = x - random_x;
                    } else if direction == DIR_SOUTH | DIR_EAST{
                        dest_y = y - random_y;
                        dest_x = x + random_x;
                    } else if direction == DIR_SOUTH | DIR_WEST {
                        dest_y = y - random_y;
                        dest_x = x - random_x;
                    } else if direction == DIR_NORTH | DIR_EAST {
                        dest_y = y + random_y;
                        dest_x = x + random_x;
                    } else if direction == DIR_NORTH | DIR_WEST  {
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
                        return (dest_x, dest_y)
                    }
                }
            } else {
                directions.swap_remove(random_index);
            }
        }
    }

    fn create_map_instance(&self, server: &Server, instance_id: u8) -> Arc<MapInstance> {
        info!("create map instance: {} x_size: {}, y_size {}, length: {}", self.name, self.x_size, self.y_size, self.length);
        let mut map_items: HashMap<u32, MapItem> = HashMap::with_capacity(2048);
        let cells = self.generate_cells(server.clone(), &mut map_items);

        let (map_event_notification_sender, single_map_event_notification_receiver) = std::sync::mpsc::sync_channel::<MapEvent>(0);
        let map_instance = MapInstance::from_map(self, server, instance_id, cells, map_items, map_event_notification_sender, server.client_notification_sender());
        self.map_instances_count.fetch_add(1, Relaxed);
        let map_instance_ref = Arc::new(map_instance);
        {
            let mut map_instance_guard = self.map_instances.borrow_mut();
            map_instance_guard.push(map_instance_ref.clone());
        }
        Map::start_thread(map_instance_ref.clone(), server, single_map_event_notification_receiver);
        map_instance_ref
    }

    pub fn generate_cells(&self, server: &Server, map_items: &mut HashMap<u32, MapItem>) -> Vec<u16> {
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

    fn set_warps(&mut self, warps: &[Warp], map_item_ids: MyUnsafeCell<HashMap<u32, MapItem>>) {
        let warps = warps.iter().map(|warp| {
            let mut warp = warp.clone();
            warp.set_id(Server::generate_id(&mut map_item_ids.borrow_mut()));
            Arc::new(warp)
        }).collect::<Vec<Arc<Warp>>>();
        self.warps = Arc::new(warps);
    }

    fn set_mob_spawns(&mut self, mob_spawns: &[MobSpawn]) {
        self.mob_spawns = Arc::new(
            mob_spawns.iter().map(|mob_spawn| Arc::new(mob_spawn.clone())).collect::<Vec<Arc<MobSpawn>>>()
        );
    }

    fn set_scripts(&mut self, scripts: &[Script], map_item_ids: MyUnsafeCell<HashMap<u32, MapItem>>) {
        self.scripts = Arc::new(
            scripts.iter().map(|script| {
                let mut script = script.clone();
                script.set_id(Server::generate_id(&mut map_item_ids.borrow_mut()));
                script
            }).collect::<Vec<Script>>()
        );
    }

    fn start_thread(map_instance: Arc<MapInstance>, server: &Server, single_map_event_notification_receiver: Receiver<MapEvent>) {
        let map_instance_clone = map_instance.clone();
        let map_instance_clone_for_thread = map_instance.clone();
        info!("Start thread for {}", map_instance_clone.name);
        thread::Builder::new().name(format!("map_instance_{}_thread", map_instance_clone.name))
            .spawn(move || {
                loop {
                    for event in single_map_event_notification_receiver.iter() {
                        match event {
                            MapEvent::SpawnMob => {
                                map_instance_clone_for_thread.spawn_mobs();
                            }
                            MapEvent::UpdateMobFov => {}
                        }
                    }
                }
                // let mut now = Instant::now();
                // let mut cleanup_notified_at: Option<Instant> = None;
                // let mut last_mobs_action = now;
                // while cleanup_notified_at.is_none() || now.duration_since(cleanup_notified_at.unwrap()).as_secs() < 5 {
                //     now = Instant::now();
                //     if rx.try_recv().is_ok() {
                //         info!("received clean up sig");
                //         cleanup_notified_at = Some(now);
                //     }
                //     {
                //         // map_instance_clone_for_thread.spawn_mobs(server.clone(), now.elapsed().as_millis(), map_instance.clone());
                //         // map_instance_clone_for_thread.update_mobs_fov();
                //         if last_mobs_action.elapsed().as_secs() > 2 {
                //             // map_instance_clone_for_thread.mobs_action();
                //             last_mobs_action = now;
                //         }
                //     }
                //     sleep(Duration::from_millis(50));
                // }
                info!("Clean up {} map", map_instance_clone.name);
            }).unwrap();
    }

    pub fn load_maps(warps: HashMap<String, Vec<Warp>>, mob_spawns: HashMap<String, Vec<MobSpawn>>, scripts: HashMap<String, Vec<Script>>, map_items: MyUnsafeCell<HashMap<u32, MapItem>>) -> HashMap<String, Map> {
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
                length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap()
            };
            // TODO validate checksum
            // TODO validate size + length

            let mut map = Map {
                x_size: header.x_size as u16,
                y_size: header.y_size as u16,
                length: header.length,
                name: map_name.to_string(),
                warps: Default::default(),
                mob_spawns: Default::default(),
                scripts: Default::default(),
                map_instances: Default::default(),
                map_instances_count: Default::default()
            };
            map.set_warps(warps.get(&map_name).unwrap_or(&vec![]), map_items.clone());
            map.set_mob_spawns(mob_spawns.get(&map_name).unwrap_or(&vec![]));
            map.set_scripts(scripts.get(&map_name).unwrap_or(&vec![]), map_items.clone());
            maps.insert(map.name.clone(), map);
        }
        maps
    }

    pub fn name_without_ext(map_name: &String) -> String {
        map_name.replace(MAP_EXT, "")
    }
}