use std::fs::File;
use std::io::Read;
use std::str;
use std::usize;

#[derive(Debug)]
#[allow(dead_code)]
struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    const_pool_count: u16,
    const_pool: Vec<ConstantPoolEntry>,
}

#[derive(Debug)]
enum ConstantPoolEntry {
    Class(u8, u16),
    FieldRef(u8, u16, u16),
    Methodref(u8, u16, u16),
    InterfaceMethodref(u8, u16, u16),
    String(u8, u16),
    Integer(u8, u32),
    Float(u8, u32),
    Long(u8, u32, u32),
    Double(u8, u32, u32),
    NameAndType(u8, u16, u16),
    Utf8(u8, u16, String),
    MethodHandle(u8, u8, u16),
    MethodType(u8, u16),
    InvokeDynamic(u8, u16, u16),
}

fn main() {
    let bytes = open_file("./java/Main.class");

    let mut const_pool = Vec::new();
    let magic = read_u32(&bytes, 0);
    let minor_version = read_u16(&bytes, 4);
    let major_version = read_u16(&bytes, 6);
    let const_pool_count = read_u16(&bytes, 8);

    let mut pos = 10;
    for _ in 0..(const_pool_count-1) {
        let (result, new_pos) = read_constant_pool(&bytes, pos);
        const_pool.push(result);
        pos = new_pos;
    }

    let class = Class {
        magic,
        minor_version,
        major_version,
        const_pool_count,
        const_pool,
    };
    println!("{:#?}", class);
}

fn read_constant_pool(bytes: &Vec<u8>, pos: usize) -> (ConstantPoolEntry, usize) {
    let tag = read_u8(&bytes, pos);
    return match tag {
        1 => {
            let length = read_u16(&bytes, pos+1) as usize;
            let bytes = read_bytes(&bytes, pos+3, length);
            let text = str::from_utf8(&bytes).unwrap();

            (ConstantPoolEntry::Utf8(1, length as u16, text.to_string()), pos+3+length)
        }
        3 => {
            let integer_bytes = read_u32(&bytes, pos+1);

            (ConstantPoolEntry::Integer(3, integer_bytes), pos+5)
        }
        4 => {
            let float_bytes = read_u32(&bytes, pos+1);

            (ConstantPoolEntry::Float(4, float_bytes), pos+5)
        }
        5 => {
            let high_bytes = read_u32(&bytes, pos+1);
            let low_bytes = read_u32(&bytes, pos+5);

            (ConstantPoolEntry::Double(5, high_bytes, low_bytes), pos+9)
        }
        6 => {
            let high_bytes = read_u32(&bytes, pos+1);
            let low_bytes = read_u32(&bytes, pos+5);

            (ConstantPoolEntry::Long(6, high_bytes, low_bytes), pos+9)
        }
        7 => {
            let name_index = read_u16(&bytes, pos+1);

            (ConstantPoolEntry::Class(7, name_index), pos+3)
        }
        9 => {
            let class_index = read_u16(&bytes, pos+1);
            let name_and_type_index = read_u16(&bytes, pos+3);

            (ConstantPoolEntry::FieldRef(9, class_index, name_and_type_index), pos+5)
        }
        8 => {
            let string_index = read_u16(&bytes, pos+1);

            (ConstantPoolEntry::String(8, string_index), pos+3)
        }
        10 => {
            let class_index = read_u16(&bytes, pos+1);
            let name_and_type_index = read_u16(&bytes, pos+3);

            (ConstantPoolEntry::Methodref(10, class_index, name_and_type_index), pos+5)
        }
        11 => {
            let class_index = read_u16(&bytes, pos+1);
            let name_and_type_index = read_u16(&bytes, pos+3);

            (ConstantPoolEntry::InterfaceMethodref(11, class_index, name_and_type_index), pos+5)
        }
        12 => {
            let name_index = read_u16(&bytes, pos+1);
            let descriptor_index = read_u16(&bytes, pos+3);

            (ConstantPoolEntry::NameAndType(12, name_index, descriptor_index), pos+5)
        }
        15 => {
            let reference_kind = read_u8(&bytes, pos+1);
            let reference_index = read_u16(&bytes, pos+2);

            (ConstantPoolEntry::MethodHandle(15, reference_kind, reference_index), pos+4)
        }
        16 => {
            let descriptor_index = read_u16(&bytes, pos+1);

            (ConstantPoolEntry::MethodType(16, descriptor_index), pos+3)
        }
        18 => {
            let bootstrap_method_attr_index = read_u16(&bytes, pos+1);
            let name_and_type_index = read_u16(&bytes, pos+3);

            (ConstantPoolEntry::InvokeDynamic(18, bootstrap_method_attr_index, name_and_type_index), pos+5)
        }
        unknown_tag => {
            panic!("NOT SUPPORTED: {unknown_tag}");
        }
    };
}

fn open_file(file: &str) -> Vec<u8> {
    let mut input = File::open(file).unwrap();
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();

    buffer
}

fn read_u32(bytes: &Vec<u8>, pos: usize) -> u32 {
    return ((bytes[pos] as u32) << 24) |
        ((bytes[pos + 1] as u32) << 16) |
        ((bytes[pos + 2] as u32) << 8) |
        (bytes[pos + 3] as u32);
}

fn read_u16(bytes: &Vec<u8>, pos: usize) -> u16 {
    return ((bytes[pos] as u16) << 8) |
        (bytes[pos + 1] as u16);
}

fn read_u8(bytes: &Vec<u8>, pos: usize) -> u8 {
    return bytes[pos];
}

fn read_bytes(bytes: &Vec<u8>, pos: usize, length: usize) -> &[u8] {
    return &bytes[pos..(pos+length)];
}
