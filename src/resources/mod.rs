use std::collections::VecDeque;
use std::io::prelude::*;
use std::fs::File;
//use std::fs::remove_file;
mod utils;
mod huffman;
use self::utils::*;
use self::huffman::*;

fn read_file(filename: &String) -> Vec<u8> {
    let f = File::open(filename)
    .expect("file not found");
    let mut string = Vec::new();
    for byte in f.bytes() {
        string.push(byte.unwrap());
    }
    return string;
}

pub fn pack(filename: &String, output_path: &String) {
    let file = read_file(&filename);
    let base = create_base(&file);
    ////////////// BUILD TREE //////////////
    let tree = build(base);
    let schema = schema_v2(&tree, vec![]);

    let mut basename: Vec<&str> = filename.split('/').collect();
    let mut buffer = File::create(format!("{}/{}{}", output_path, basename.pop().unwrap(), ".comp"))
    .expect("file not found");
    let mut track_new = Vec::new();
    for letter in file {
        track_new.extend(&schema[letter as usize]);
    }
    let bytes = bit_to_byte(&mut track_new);
    let serialized = serialize_tree(tree);
    buffer.write(serialized.as_bytes()).unwrap();
    buffer.write(bytes.as_slice()).unwrap();
}

pub fn unpack(filename: &String, output_path: &String) {
    let mut file = VecDeque::from(read_file(filename));
    //remove_file(&filename).unwrap();

    let tree = deserialize_tree(&mut file);
    let mut new_file = byte_to_bit(&mut file);
    let lenght = new_file.len();
    let mut string = Vec::new();
    let mut count = 0;
    let mut node = &tree;
    while count < lenght {
        let marker = new_file.pop_front().unwrap();
        if node.value == None {
            if marker < 1 {
                if let Some(ref x) = node.l {
                    node = &x
                }
            } else {
                if let Some(ref y) = node.r {
                    node = &y
                }
            }
        }
        if let Some(ref z) = node.value {
            string.push(*z as u8);
            node = &tree;
        }
        count += 1;
    }
    let mut basename: Vec<&str> = filename.split('/').collect();
    let mut name: Vec<&str> = basename.pop().unwrap().split('.').collect();
    name.remove(2);
    let mut buffer = File::create(format!("{}/{}", output_path, name.join(".") as String))
    .expect("error writing file");
    buffer.write(string.as_slice()).unwrap();
}
