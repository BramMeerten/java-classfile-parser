use std::fs::File;
use std::io::Read;
use std::usize;

#[derive(Debug)]
struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16
}

fn main() {
    let bytes = open_file("./java/Main.class");
    let magic = read_u32(&bytes, 0);
    let minor = read_u16(&bytes, 4);
    let major = read_u16(&bytes, 6);

    let class = Class {
        magic,
        minor_version: minor,
        major_version: major
    };

    println!("{:#02x?}", class);
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
