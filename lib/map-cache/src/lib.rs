//! Map cell loading for both container layouts that carry the same GAT-derived
//! payload: per-map `.mcache` files (this server) and rAthena's concatenated
//! `map_cache.dat`. Loads one map per call and holds no cache; caching is the
//! consumer's policy.

use flate2::read::ZlibDecoder;
use models::enums::cell::CellType;
use models::enums::EnumWithMaskValueU16;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::ops::Range;
use std::path::{Path, PathBuf};

const MCACHE_EXT: &str = ".mcache";
const MCACHE_HEADER_LEN: usize = 26;
/// u32 file size + u16 map count, padded to 8 on disk by struct alignment.
const RATHENA_HEADER_LEN: usize = 8;
/// 12-byte NUL-padded name + i16 xs + i16 ys + i32 compressed length.
const RATHENA_RECORD_LEN: usize = 20;

pub struct MapCells {
    pub name: String,
    pub xs: u16,
    pub ys: u16,
    /// `CellType` flags, index = x + y * xs.
    pub cells: Vec<u16>,
}

impl MapCells {
    pub fn cell(&self, x: u16, y: u16) -> u16 {
        if x >= self.xs || y >= self.ys {
            return 0;
        }
        self.cells[x as usize + y as usize * self.xs as usize]
    }

    pub fn is_walkable(&self, x: u16, y: u16) -> bool {
        self.cell(x, y) & CellType::Walkable.as_flag() != 0
    }
}

#[derive(Debug)]
pub enum MapCacheError {
    Io(PathBuf, std::io::Error),
    NotFound(String),
    Corrupt(PathBuf, String),
    ChecksumMismatch(String),
    SizeMismatch {
        map: String,
        expected: usize,
        actual: usize,
    },
}

impl Display for MapCacheError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapCacheError::Io(path, e) => write!(f, "cannot read {}: {e}", path.display()),
            MapCacheError::NotFound(map) => write!(f, "map {map} is not in the cache"),
            MapCacheError::Corrupt(path, reason) => {
                write!(f, "{} is corrupt: {reason}", path.display())
            }
            MapCacheError::ChecksumMismatch(map) => {
                write!(f, "checksum mismatch for map {map}")
            }
            MapCacheError::SizeMismatch {
                map,
                expected,
                actual,
            } => write!(
                f,
                "map {map} decoded to {actual} cells where its header promises {expected}"
            ),
        }
    }
}

impl std::error::Error for MapCacheError {}

/// One byte per cell in, `CellType` flags out. Walkable ground values 2, 4 and 6
/// should not appear in a cache file, but Hercules writes them as ground.
pub fn decode_cells(raw: &[u8]) -> Vec<u16> {
    raw.iter()
        .map(|cell| match cell {
            0 | 2 | 4 | 6 => CellType::Walkable.as_flag() | CellType::Shootable.as_flag(),
            3 => {
                CellType::Walkable.as_flag()
                    | CellType::Shootable.as_flag()
                    | CellType::Water.as_flag()
            }
            5 => CellType::Shootable.as_flag(),
            _ => 0,
        })
        .collect()
}

/// Container A: one `{map_name}.mcache` per map, md5-verified.
pub fn read_mcache(dir: &Path, map_name: &str) -> Result<MapCells, MapCacheError> {
    let path = dir.join(format!("{map_name}{MCACHE_EXT}"));
    let raw = std::fs::read(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            MapCacheError::NotFound(map_name.to_string())
        } else {
            MapCacheError::Io(path.clone(), e)
        }
    })?;
    if raw.len() < MCACHE_HEADER_LEN {
        return Err(MapCacheError::Corrupt(
            path,
            "shorter than its header".to_string(),
        ));
    }
    let checksum = &raw[2..18];
    let xs = i16::from_le_bytes([raw[18], raw[19]]);
    let ys = i16::from_le_bytes([raw[20], raw[21]]);
    let len = i32::from_le_bytes([raw[22], raw[23], raw[24], raw[25]]);
    if xs <= 0 || ys <= 0 || len <= 0 {
        return Err(MapCacheError::Corrupt(
            path,
            format!("implausible dimensions {xs}x{ys} length {len}"),
        ));
    }
    let compressed = &raw[MCACHE_HEADER_LEN..];
    if md5::compute(compressed).0 != checksum {
        return Err(MapCacheError::ChecksumMismatch(map_name.to_string()));
    }
    let cells = inflate_cells(map_name, &path, compressed, xs as usize * ys as usize)?;
    Ok(MapCells {
        name: map_name.to_string(),
        xs: xs as u16,
        ys: ys as u16,
        cells,
    })
}

pub struct MapCacheEntry {
    pub name: String,
    pub xs: u16,
    pub ys: u16,
}

/// Container B: rAthena's concatenated cache. No checksum in this layout.
pub fn read_rathena_cache(path: &Path, map_name: &str) -> Result<MapCells, MapCacheError> {
    let raw = read_rathena_file(path)?;
    for (name, xs, ys, data) in rathena_records(&raw, path)? {
        if name == map_name {
            let cells = inflate_cells(map_name, path, &raw[data], xs as usize * ys as usize)?;
            return Ok(MapCells {
                name,
                xs,
                ys,
                cells,
            });
        }
    }
    Err(MapCacheError::NotFound(map_name.to_string()))
}

/// Names and dimensions only, nothing decoded.
pub fn list_rathena_cache(path: &Path) -> Result<Vec<MapCacheEntry>, MapCacheError> {
    let raw = read_rathena_file(path)?;
    Ok(rathena_records(&raw, path)?
        .into_iter()
        .map(|(name, xs, ys, _)| MapCacheEntry { name, xs, ys })
        .collect())
}

/// A cell source the consumer can point at either server's data.
pub enum MapCacheSource {
    PerMap(PathBuf),
    Rathena(PathBuf),
}

impl MapCacheSource {
    pub fn load(&self, map_name: &str) -> Result<MapCells, MapCacheError> {
        match self {
            MapCacheSource::PerMap(dir) => read_mcache(dir, map_name),
            MapCacheSource::Rathena(path) => read_rathena_cache(path, map_name),
        }
    }
}

fn read_rathena_file(path: &Path) -> Result<Vec<u8>, MapCacheError> {
    let raw = std::fs::read(path).map_err(|e| MapCacheError::Io(path.to_path_buf(), e))?;
    if raw.len() < RATHENA_HEADER_LEN {
        return Err(MapCacheError::Corrupt(
            path.to_path_buf(),
            "shorter than its header".to_string(),
        ));
    }
    let file_size = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as usize;
    if file_size != raw.len() {
        return Err(MapCacheError::Corrupt(
            path.to_path_buf(),
            format!("header says {file_size} bytes, file has {}", raw.len()),
        ));
    }
    Ok(raw)
}

fn rathena_records(
    raw: &[u8],
    path: &Path,
) -> Result<Vec<(String, u16, u16, Range<usize>)>, MapCacheError> {
    let count = u16::from_le_bytes([raw[4], raw[5]]) as usize;
    let mut records = Vec::with_capacity(count);
    let mut offset = RATHENA_HEADER_LEN;
    for index in 0..count {
        let end = offset + RATHENA_RECORD_LEN;
        if end > raw.len() {
            return Err(MapCacheError::Corrupt(
                path.to_path_buf(),
                format!("record {index} of {count} runs past the end of the file"),
            ));
        }
        let name = raw[offset..offset + 12]
            .split(|b| *b == 0)
            .next()
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .map(str::to_string)
            .ok_or_else(|| {
                MapCacheError::Corrupt(
                    path.to_path_buf(),
                    format!("record {index} has a non-utf8 map name"),
                )
            })?;
        let xs = i16::from_le_bytes([raw[offset + 12], raw[offset + 13]]);
        let ys = i16::from_le_bytes([raw[offset + 14], raw[offset + 15]]);
        let len = i32::from_le_bytes([
            raw[offset + 16],
            raw[offset + 17],
            raw[offset + 18],
            raw[offset + 19],
        ]);
        if xs <= 0 || ys <= 0 || len < 0 || end + len as usize > raw.len() {
            return Err(MapCacheError::Corrupt(
                path.to_path_buf(),
                format!("record {index} ({name}) has implausible dimensions {xs}x{ys} length {len}"),
            ));
        }
        records.push((name, xs as u16, ys as u16, end..end + len as usize));
        offset = end + len as usize;
    }
    Ok(records)
}

fn inflate_cells(
    map_name: &str,
    path: &Path,
    compressed: &[u8],
    expected: usize,
) -> Result<Vec<u16>, MapCacheError> {
    let mut decoded = Vec::with_capacity(expected);
    ZlibDecoder::new(compressed)
        .read_to_end(&mut decoded)
        .map_err(|e| MapCacheError::Corrupt(path.to_path_buf(), format!("zlib: {e}")))?;
    if decoded.len() != expected {
        return Err(MapCacheError::SizeMismatch {
            map: map_name.to_string(),
            expected,
            actual: decoded.len(),
        });
    }
    Ok(decode_cells(&decoded))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;

    /// 4x2 map: ground, gap, water, wall on the first row.
    const RAW_CELLS: [u8; 8] = [0, 5, 3, 1, 0, 0, 0, 0];

    fn compress(cells: &[u8]) -> Vec<u8> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(cells).unwrap();
        encoder.finish().unwrap()
    }

    fn mcache_bytes(cells: &[u8], xs: i16, ys: i16) -> Vec<u8> {
        let compressed = compress(cells);
        let mut out = Vec::new();
        out.extend_from_slice(&1i16.to_le_bytes());
        out.extend_from_slice(&md5::compute(&compressed).0);
        out.extend_from_slice(&xs.to_le_bytes());
        out.extend_from_slice(&ys.to_le_bytes());
        out.extend_from_slice(&(cells.len() as i32).to_le_bytes());
        out.extend_from_slice(&compressed);
        out
    }

    fn rathena_bytes(maps: &[(&str, i16, i16, &[u8])]) -> Vec<u8> {
        let mut body = Vec::new();
        for (name, xs, ys, cells) in maps {
            let compressed = compress(cells);
            let mut record = [0u8; 12];
            record[..name.len()].copy_from_slice(name.as_bytes());
            body.extend_from_slice(&record);
            body.extend_from_slice(&xs.to_le_bytes());
            body.extend_from_slice(&ys.to_le_bytes());
            body.extend_from_slice(&(compressed.len() as i32).to_le_bytes());
            body.extend_from_slice(&compressed);
        }
        let mut out = Vec::new();
        out.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
        out.extend_from_slice(&(maps.len() as u16).to_le_bytes());
        out.extend_from_slice(&[0, 0]);
        out.extend_from_slice(&body);
        out
    }

    fn assert_payload(map: &MapCells) {
        assert_eq!((map.xs, map.ys), (4, 2));
        assert!(map.is_walkable(0, 0));
        assert!(!map.is_walkable(1, 0));
        assert_ne!(map.cell(1, 0) & CellType::Shootable.as_flag(), 0);
        assert!(map.is_walkable(2, 0));
        assert_ne!(map.cell(2, 0) & CellType::Water.as_flag(), 0);
        assert!(!map.is_walkable(3, 0));
        assert_eq!(map.cell(3, 0), 0);
        assert!(!map.is_walkable(4, 0));
    }

    #[test]
    fn reads_both_containers_and_rejects_tampering() {
        let dir = std::env::temp_dir().join("map-cache-test");
        std::fs::create_dir_all(&dir).unwrap();

        let mut bytes = mcache_bytes(&RAW_CELLS, 4, 2);
        std::fs::write(dir.join("tiny.mcache"), &bytes).unwrap();
        assert_payload(&read_mcache(&dir, "tiny").unwrap());
        assert!(matches!(
            read_mcache(&dir, "absent"),
            Err(MapCacheError::NotFound(_))
        ));

        bytes[2] ^= 0xFF;
        std::fs::write(dir.join("tampered.mcache"), &bytes).unwrap();
        assert!(matches!(
            read_mcache(&dir, "tampered"),
            Err(MapCacheError::ChecksumMismatch(_))
        ));

        let concatenated = rathena_bytes(&[
            ("filler", 1, 1, &[0]),
            ("tiny", 4, 2, &RAW_CELLS),
        ]);
        let cache_path = dir.join("map_cache.dat");
        std::fs::write(&cache_path, &concatenated).unwrap();
        assert_payload(&read_rathena_cache(&cache_path, "tiny").unwrap());
        assert!(matches!(
            read_rathena_cache(&cache_path, "absent"),
            Err(MapCacheError::NotFound(_))
        ));
        let listing = list_rathena_cache(&cache_path).unwrap();
        assert_eq!(listing.len(), 2);
        assert_eq!(listing[1].name, "tiny");

        std::fs::write(&cache_path, &concatenated[..40]).unwrap();
        assert!(matches!(
            read_rathena_cache(&cache_path, "tiny"),
            Err(MapCacheError::Corrupt(..))
        ));

        assert_payload(
            &MapCacheSource::PerMap(dir.clone())
                .load("tiny")
                .unwrap(),
        );

        std::fs::remove_dir_all(&dir).unwrap();
    }

    /// The payloads come from different GAT revisions, so a small drift per map
    /// is data, not a decoder bug; a decoder bug shows up as massive disagreement.
    #[test]
    fn cross_validates_real_containers_when_paths_are_set() {
        let (Ok(mcache_dir), Ok(rathena_path)) = (
            std::env::var("RUST_RO_MCACHE_DIR"),
            std::env::var("RATHENA_MAP_CACHE"),
        ) else {
            return;
        };
        let dir = Path::new(&mcache_dir);
        let mut shared = 0u32;
        let mut identical = 0u32;
        let mut dim_mismatches = Vec::new();
        let mut worst: (String, f64) = (String::new(), 0.0);
        for entry in list_rathena_cache(Path::new(&rathena_path)).unwrap() {
            let theirs = match read_rathena_cache(Path::new(&rathena_path), &entry.name) {
                Ok(map) => map,
                Err(e) => panic!("{e}"),
            };
            let ours = match read_mcache(dir, &entry.name) {
                Ok(map) => map,
                Err(MapCacheError::NotFound(_)) => continue,
                Err(e) => panic!("{e}"),
            };
            shared += 1;
            if (ours.xs, ours.ys) != (theirs.xs, theirs.ys) {
                dim_mismatches.push(entry.name.clone());
                continue;
            }
            if ours.cells == theirs.cells {
                identical += 1;
                continue;
            }
            let diff = ours
                .cells
                .iter()
                .zip(&theirs.cells)
                .filter(|(a, b)| a != b)
                .count() as f64
                / ours.cells.len() as f64;
            if diff > worst.1 {
                worst = (entry.name.clone(), diff);
            }
        }
        println!(
            "cross-validated {shared} shared maps: {identical} identical, \
             {} with dimension mismatches {dim_mismatches:?}, worst drift {:.3}% on {}",
            dim_mismatches.len(),
            worst.1 * 100.0,
            worst.0
        );
        assert!(shared > 500, "only {shared} shared maps; wrong paths?");
        assert!(
            f64::from(identical) / f64::from(shared) > 0.9,
            "decoders disagree on too many maps"
        );
        assert!(worst.1 < 0.02, "drift on {} is {:.3}%", worst.0, worst.1 * 100.0);
    }
}
