use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read, Cursor};
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

#[derive(Debug)]
struct Map {
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
    pub cells: Vec<u16>
}

impl Map {
    pub fn get_cell_index_of(&self, x: i16, y: i16) -> u32 {
        let i = (x as u32 + y as u32 * self.x_size as u32) as usize;
        i as u32
    }
    pub fn get_pos_of(&self, index: u32) -> (i16, i16) {
        let y : i16 = (index / self.x_size as u32) as i16;
        let x : i16 = (index - (y as u32 * self.x_size as u32) as u32) as i16;
        (x, y)
    }
}

fn main() {
    let izlude_map_path = Path::new("maps/pre-re/izlude.mcache");
    let file = File::open(izlude_map_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = [0 as u8; 26];
    reader.read(&mut buf);
    println!("{:?}", buf);
    let header = Header {
        version: Cursor::new(buf[0..2].to_vec()).read_i16::<LittleEndian>().unwrap(),
        checksum: buf[2..18].try_into().unwrap(),
        x_size: Cursor::new(buf[18..20].to_vec()).read_i16::<LittleEndian>().unwrap(),
        y_size: Cursor::new(buf[20..22].to_vec()).read_i16::<LittleEndian>().unwrap(),
        length: Cursor::new(buf[22..26].to_vec()).read_i32::<LittleEndian>().unwrap()
    };
    // TODO validate checksum
    // TODO validate size + length

    let mut map_cache_zip_content_buf = Vec::new();
    let mut map_cache_content_buf = Vec::new();
    reader.read_to_end(&mut map_cache_zip_content_buf);
    let mut decoder = ZlibDecoder::new(&map_cache_zip_content_buf[..]);
    decoder.read_to_end(&mut map_cache_content_buf).unwrap();

    let mut cells: Vec<u16> = Vec::new();
    for (i, cell) in map_cache_content_buf.iter().enumerate() {
        cells.push(match cell {
            (0 | 2 | 4 | 6) => 0b0000_0000_0000_0011, // walkable ground values 2,4,6 are unknown, should not be present in mapcache file. but hercules set them to this value.
            1 => 0, // no walkable ground
            3 => 0b0000_0000_0000_0111, // walkable water
            5 => 0b0000_0000_0000_0010, // gap, (shootable)
            _ => 0
        })
    }
    let map = Map {
        x_size: header.x_size as u16,
        y_size: header.y_size as u16,
        length: header.length,
        name: "izlude".to_string(),
        cells
    };
    let a = map.get_cell_index_of(267, 299);
    println!("{:?}", a);
    println!("{:?}", map.get_pos_of(a));
}