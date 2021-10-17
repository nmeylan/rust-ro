use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, Read, Cursor, Write};

use byteorder::{ReadBytesExt, LittleEndian, WriteBytesExt};
use flate2::{Compression};
use std::fs;
use log::{error, info, warn};
use flexi_logger::Logger;
use std::str;
use std::process::exit;
use flate2::write::ZlibEncoder;
use md5;
use std::num::ParseIntError;

use futures::future::join_all;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use std::time::Instant;

/*
TODO make a cli from this.

Usage: this tool need .gat and .rsw files to generated mapcache.
Using GRF editor, extract .gat and .rsw into a folder.

set GRF_DATA_PATH to this folder.
 */
static PARALLEL_EXECUTIONS: usize = 100;
static NO_WATER: f32 = 1000000.0;
static GRF_DATA_PATH: &str = "D:\\ragnarok\\kRO_client\\data"; // source .gat/.rsw
static MAP_CACHE_PATH: &str = "maps/pre-re"; // destination

struct Counter {
    pub value: u32
}

impl Counter {
    pub fn increment_and_get(&mut self) -> u32 {
        self.value += 1;
        self.value
    }
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    Logger::try_with_str("info").unwrap().start().unwrap();
    let grf_data_path = Path::new(GRF_DATA_PATH);
    let paths = fs::read_dir(grf_data_path).unwrap();
    let mut file_paths = Vec::<String>::new();
    let mut map_names = Vec::<String>::new();
    for dir_entry in paths {
        let path = dir_entry.unwrap().path();
        let file_path_without_ext = path.as_path().to_str().unwrap().to_string();
        if file_path_without_ext.ends_with(".gat") {
            map_names.push(path.file_name().unwrap().to_str().unwrap().to_string().replace(".gat", ""));
            file_paths.push(file_path_without_ext.replace(".gat", ""));
        }
    }
    let semaphore = Semaphore::new(PARALLEL_EXECUTIONS);

    let len = file_paths.len();
    let counter: Arc<Mutex<Counter>> = Arc::new(Mutex::new(Counter{value: 0}));
    let mut futures = Vec::new();
    let mut map_iter = map_names.into_iter();
    for (_i, file_name) in file_paths.into_iter().enumerate() {
        semaphore.acquire().await.unwrap();
        let map_name = map_iter.next().unwrap();
        let counter_clone = counter.clone();
        futures.push(tokio::task::spawn_blocking(move || {
            let start = Instant::now();
            let gat_name = format!("{}.gat", file_name);
            let rsw_name = format!("{}.rsw", file_name);

            let gat_path = Path::new(&gat_name[..]);
            let rsw_path = Path::new(&rsw_name[..]);
            let mut water_height: f32 = NO_WATER as f32;

            if !gat_path.exists() {
                error!("{}: .gat file with path {} does not exists", map_name, gat_path.to_str().unwrap());
                return;
            }
            if rsw_path.exists() {
                let rsw_file = File::open(rsw_path).unwrap();
                let mut rsw_content_buf = Vec::new();
                let mut reader = BufReader::new(rsw_file);
                reader.read_to_end(&mut rsw_content_buf);

                if str::from_utf8(&rsw_content_buf[0..4]).unwrap() != "GRSW" {
                    error!("{}: is not rsw file", rsw_path.to_str().unwrap());
                    exit(-1)
                }
                let major_version = rsw_content_buf[4];
                let minor_version = rsw_content_buf[5];
                if (major_version > 2 || (major_version == 2 && minor_version > 5)) || (major_version < 1 || (major_version == 1 && minor_version <= 4)) {
                    error!("{}: unsupported version {}.{}", rsw_path.to_str().unwrap(), major_version, minor_version);
                    exit(-1)
                }
                let mut offset = 166;
                if major_version == 2 && minor_version >= 5 {
                    offset += 4;
                }
                if major_version == 2 && minor_version >= 2 {
                    offset += 1;
                }
                water_height = Cursor::new(rsw_content_buf[offset..(offset + 4)].to_vec()).read_f32::<LittleEndian>().unwrap();
            } else {
                warn!("{}: .rsw file with path {} does not exists", map_name, rsw_path.to_str().unwrap());
            }

            let gat_file = File::open(gat_path).unwrap();
            let mut gat_content_buf = Vec::new();
            let mut reader = BufReader::new(gat_file);
            reader.read_to_end(&mut gat_content_buf);

            let x_size = Cursor::new(gat_content_buf[6..8].to_vec()).read_i16::<LittleEndian>().unwrap() as u32;
            let y_size = Cursor::new(gat_content_buf[10..12].to_vec()).read_i16::<LittleEndian>().unwrap() as u32;
            let map_size: u32 = x_size * y_size;
            let tiles_beginning_offset = 14;
            let tiles_height_offset = 30;
            let mut tile_cursor = 0;
            let mut cells = Vec::<u8>::new();
            for _i in 0..map_size {
                let mut pos = tile_cursor + tiles_beginning_offset;
                let cell_height = Cursor::new(gat_content_buf[pos..(pos + 4)].to_vec()).read_f32::<LittleEndian>().unwrap();
                pos = tile_cursor + tiles_height_offset;
                let mut cell_type = Cursor::new(gat_content_buf[pos..(pos + 4)].to_vec()).read_u32::<LittleEndian>().unwrap();
                if cell_type == 0 && water_height != NO_WATER && cell_height > water_height {
                    cell_type = 3;
                }
                tile_cursor += 20;
                cells.push(cell_type as u8);
            }
            let map_cache_path = Path::join(Path::new(MAP_CACHE_PATH), format!("{}.mcache", map_name));
            let mut map_cache_file = File::create(map_cache_path).unwrap();

            let mut zip_encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            zip_encoder.write_all(&cells[..]).unwrap();
            let compressed_cells = zip_encoder.finish().unwrap();
            let digest = md5::compute(compressed_cells.clone());
            let mut wtr = vec![];
            wtr.write_i16::<LittleEndian>(0x1);
            let mut checksum_buf = [0 as u8; 16];
            let checksum = decode_hex(format!("{:x}", digest).as_str()).unwrap();
            checksum.iter().enumerate().for_each(|(i, value)| checksum_buf[i] = *value);
            wtr.write_all(&checksum_buf);
            wtr.write_i16::<LittleEndian>(x_size as i16);
            wtr.write_i16::<LittleEndian>(y_size as i16);
            wtr.write_i32::<LittleEndian>(compressed_cells.len() as i32);
            wtr.write_all(&compressed_cells[..]);
            map_cache_file.write_all(&wtr[..]);

            {
                let mut counter = counter_clone.lock().unwrap();
                info!("{}: map cache ({}/{}), {}ms", map_name, counter.increment_and_get(), len, start.elapsed().as_millis());
            }
        }));
    }
    join_all(futures).await;
    println!();
    info!("Map cache generation took {}s", start.elapsed().as_millis() as f32 / 1000.0);
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}