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
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    fields_count: u16,
    methods_count: u16,
    methods: Vec<Method>,
    attributes_count: u16,
    attributes: Vec<Attribute>,
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

#[derive(Debug)]
#[allow(dead_code)]
struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}


#[derive(Debug)]
#[allow(dead_code)]
struct Attribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

fn main() {
    let bytes = open_file("./java/Main.class");

    let magic = read_u32(&bytes, 0);
    let minor_version = read_u16(&bytes, 4);
    let major_version = read_u16(&bytes, 6);

    let const_pool_count = read_u16(&bytes, 8);
    let mut pos = 10;
    let mut const_pool = Vec::new();

    for _ in 0..(const_pool_count-1) {
        let (result, new_pos) = read_constant_pool(&bytes, pos);
        const_pool.push(result);
        pos = new_pos;
    }

    let access_flags = read_u16(&bytes, pos);
    let this_class = read_u16(&bytes, pos+2);
    let super_class = read_u16(&bytes, pos+4);
    let interfaces_count = read_u16(&bytes, pos+6);
    assert!(interfaces_count == 0);
    let fields_count = read_u16(&bytes, pos+8);
    assert!(fields_count == 0);

    let methods_count = read_u16(&bytes, pos+10);
    pos = pos + 12;
    let mut methods = Vec::new();

    for _ in 0..methods_count {
        let (result, new_pos) = read_method(&bytes, pos);
        methods.push(result);
        pos = new_pos;
    }
    
    let attributes_count = read_u16(&bytes, pos);
    pos = pos+2;
    let mut attributes = Vec::new();

    for _ in 0..attributes_count {
        let (result, new_pos) = read_attribute(&bytes, pos);
        attributes.push(result);
        pos = new_pos;
    }

    let class = Class {
        magic,
        minor_version,
        major_version,
        const_pool_count,
        const_pool,
        access_flags,
        this_class, 
        super_class, 
        interfaces_count,
        fields_count,
        methods_count,
        methods,
        attributes,
        attributes_count,
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

fn read_method(bytes: &Vec<u8>, pos: usize) -> (Method, usize) {
    let access_flags = read_u16(&bytes, pos);
    let name_index = read_u16(&bytes, pos+2);
    let descriptor_index = read_u16(&bytes, pos+4);
    
    let attributes_count = read_u16(&bytes, pos+6);
    let mut attributes = Vec::new();
    let mut pos = pos+8;
    for _ in 0..attributes_count {
        let (result, new_pos) = read_attribute(&bytes, pos);
        attributes.push(result);
        pos = new_pos;
    }

    return (Method {
        access_flags,
        name_index,
        descriptor_index,
        attributes_count,
        attributes,
    }, pos);
}

fn read_attribute(bytes: &Vec<u8>, pos: usize) -> (Attribute, usize) {
    let attribute_name_index = read_u16(&bytes, pos);
    let attribute_length = read_u32(&bytes, pos+2);
    
    let mut info = Vec::new();
    for i in 0..(attribute_length as usize) {
        let result = read_u8(&bytes, pos+6+i);
        info.push(result);
    }

    return (Attribute {
        attribute_name_index,
        attribute_length,
        info,
    }, pos+6+(attribute_length as usize));
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
