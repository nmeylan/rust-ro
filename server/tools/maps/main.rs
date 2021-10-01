use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor, BufRead};
use std::convert::TryInto;
use byteorder::{ReadBytesExt, LittleEndian};
use flate2::{Decompress, FlushDecompress};
use flate2::read::ZlibDecoder;

#[derive(Debug)]
struct Header {
    pub version: i16,
    pub checksum: [u8; 16],
    pub x_size: i16,
    pub y_size: i16,
    pub length: i32,
}

fn main() {
    // Convert rathena mcache.dat into many .mcache files "à la hercules"
    let rathena_map_index_path = Path::new("D:\\ragnarok\\rathena\\db\\map_index.txt");
    let rathena_map_dat_path = Path::new("D:\\ragnarok\\rathena\\db\\pre-re\\map_cache.dat");
    let file_map_index = File::open(rathena_map_index_path).unwrap();
    let mut reader = BufReader::new(file_map_index);
    let mut map_names = Vec::<String>::new();
    for line in reader.lines() {
        if line.is_err() {
            break;
        }
        let string = line.unwrap();
        if string.starts_with("//") || string.is_empty() {
            continue;
        }
        map_names.push(string);
    }

    println!("{:?}", map_names);
    // let header = Header {
    //     version: Cursor::new(buf[0..2].to_vec()).read_i16::<LittleEndian>().unwrap(),
    //     checksum: buf[2..18].try_into().unwrap(),
    //     x_size: Cursor::new(buf[18..20].to_vec()).read_i16::<LittleEndian>().unwrap(),
    //     y_size: Cursor::new(buf[20..22].to_vec()).read_i16::<LittleEndian>().unwrap(),
    //     length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap()
    // };

}