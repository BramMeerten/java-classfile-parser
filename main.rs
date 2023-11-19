// use std::fs;
// use std::io;
use std::{fs::File, io::{Read, Seek, SeekFrom}};


fn main() {
    let mut input = File::open("./java/Main.class")
        .expect("Failed to open Main.class");
    input.seek(SeekFrom::Start(0))
        .expect("Could not seek");
    
    let mut x = input.take(8);
    let mut magic = [0;4];
    x.read_exact(&mut magic)
        .expect("SHIT");
    println!("Magic number: {:x?}", magic);

    let mut minor = [0,2];
    x.read_exact(&mut minor)
        .expect("SHIT");
    println!("Minor version: {:x?}", minor);

    let mut major = [0,2];
    x.read_exact(&mut major)
        .expect("SHIT");
    println!("Major version: {:x?}", major);
}
