use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom};
use byteorder::{BigEndian, ReadBytesExt};
use flate2::bufread::ZlibDecoder;

pub fn read_mca<F: Read + Seek>(file: &mut F) -> Vec<NbtTag> {
    let mut locations: Vec<Location> = Vec::new();
    for _ in 0..1024 {
        let mut location_offset_raw: Vec<u8> = read_data(file, 3);
        // Add extra byte to start to pad the value to 4 bytes
        location_offset_raw.insert(0, 0);
        let mut location_offset_cursor = Cursor::new(location_offset_raw);
        let location_offset = location_offset_cursor.read_i32::<BigEndian>().unwrap();
        let location_sector_count = file.read_i8().unwrap();
        locations.push(Location {offset: location_offset, sector_count: location_sector_count});
    }

    let mut out: Vec<NbtTag> = Vec::new();
    for location in locations.iter() {
        if location.offset == 0 && location.sector_count == 0 {
            continue;
        }
        file.seek(SeekFrom::Start((location.offset * 4096) as u64)).unwrap();
        let length = file.read_i32::<BigEndian>().unwrap();
        let compression_type = file.read_i8().unwrap();
        if compression_type != 2 {
            panic!("Unsupported compression type found");
        }
        let chunk_raw = read_data_cursor(file, (length - 1) as usize);
        let mut decompressor = ZlibDecoder::new(chunk_raw);
        let nbt = read_nbt(&mut decompressor);
        out.push(nbt);
        //break
    }
    return out;
}

fn read_nbt<F: Read>(file: &mut F) -> NbtTag {
    return decode_tags(file, true, -1);
}

fn decode_tags<F: Read>(file: &mut F, has_name: bool, _tag_type: i8) -> NbtTag {
    // Read and set tag type if it is not defined
    let mut tag_type: i8;
    if _tag_type == -1 {
        tag_type = file.read_i8().unwrap();
    } else {
        tag_type = _tag_type;
    }

    // End tag has no name and no data.
    if tag_type == 0 {
        return NbtTag {name: None, data: None}
    }

    let mut name: Option<String> = None;
    if has_name {
        let name_length = file.read_u16::<BigEndian>().unwrap();
        let name_raw = read_data(file, name_length as usize);
        let name_value = String::from_utf8(name_raw).unwrap();
        name = Some(name_value);
    }

    let tag_out: NbtTypes = match tag_type {
        //1 => NbtTag {name: Some(NbtTypes::Byte(file.read_i8())), data}
        1 => NbtTypes::Byte(file.read_i8().unwrap()),
        2 => NbtTypes::Short(file.read_i16::<BigEndian>().unwrap()),
        3 => NbtTypes::Int(file.read_i32::<BigEndian>().unwrap()),
        4 => NbtTypes::Long(file.read_i64::<BigEndian>().unwrap()),
        5 => NbtTypes::Float(file.read_f32::<BigEndian>().unwrap()),
        6 => NbtTypes::Double(file.read_f64::<BigEndian>().unwrap()),
        7 => {
            let size = file.read_i32::<BigEndian>().unwrap();
            let data = read_data(file, size as usize);
            let out = vec_u8_into_i8(data);
            NbtTypes::ByteArray(out)
        },
        8 => {
            let length = file.read_u16::<BigEndian>().unwrap();
            let data = read_data(file, length as usize);
            let out = String::from_utf8(data).unwrap();
            NbtTypes::String(out)
        },
        9 => {
            let tag_id = file.read_i8().unwrap();
            let size = file.read_i32::<BigEndian>().unwrap();
            let mut out: Vec<NbtTypes> = Vec::new();
            for _ in 0..size {
                let tag = decode_tags(file, false, tag_id);
                let data = tag.data.unwrap();
                out.push(data);
            }
            NbtTypes::List(out)
        },
        10 => {
            let mut out: HashMap<String, NbtTypes> = HashMap::new();
            loop {
                let tag = decode_tags(file, true, -1);
                if tag.data.is_none() && tag.name.is_none() {
                    break;
                }
                out.insert(tag.name.unwrap(), tag.data.unwrap());
            }
            NbtTypes::Compound(out)
        },
        11 => {
            let size = file.read_i32::<BigEndian>().unwrap();
            let mut out: Vec<i32> = Vec::new();
            for _ in 0..size {
                let data = file.read_i32::<BigEndian>().unwrap();
                out.push(data);
            }
            NbtTypes::IntArray(out)
        },
        12 => {
            let size = file.read_i32::<BigEndian>().unwrap();
            let mut out: Vec<i64> = Vec::new();
            for _ in 0..size {
                let data = file.read_i64::<BigEndian>().unwrap();
                out.push(data);
            }
            NbtTypes::LongArray(out)
        },
        _ => panic!("Invalid tag type found. Fed data was not of the correct format."),
    };
    return NbtTag {name, data: Some(tag_out)};
}

fn read_data(file: &mut impl Read, len: usize) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![0; len];
    file.read_exact(&mut buffer).unwrap();
    return buffer;
}

fn read_data_cursor(file: &mut impl Read, len: usize) -> Cursor<Vec<u8>> {
    let data = read_data(file, len);
    let cursor = Cursor::new(data);
    return cursor;
}

// Transforms vec<u8> to vec<i8> in place
fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    // Inhibits the compiler from automatically calling v's destructor, ensuring the memory storing v's value is not removed.
    let mut v = std::mem::ManuallyDrop::new(v);
    // Gets a mutable pointer to v's buffer
    let p = v.as_mut_ptr();
    // Gets the length and capacity of v
    let len = v.len();
    let cap = v.capacity();
    // Rebuilds buffer into Vec<i8> value
    return unsafe { Vec::from_raw_parts(p as *mut i8, len, cap)};
}

struct MCA {
    chunks: Vec<Chunk>,
}

struct Location {
    offset: i32,
    sector_count: i8,
}

struct Chunk {
    nbt_data: HashMap<String, NbtTypes>,
}

#[derive(Debug)]
enum NbtTypes {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<NbtTypes>),
    Compound(HashMap<String, NbtTypes>),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

#[derive(Debug)]
pub struct NbtTag {
    name: Option<String>,
    data: Option<NbtTypes>,
}
