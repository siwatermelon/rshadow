use std::env;
// use rexiv2::Metadata;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use base64::encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: rshadow <PE_PATH> <PIC_PATH>");
        exit(0);
    }
    let pe_path = &args[1];
    let pic_path = &args[2];
    // let new_pic_path = &args[3];

    let mut file = File::open(pe_path).expect("Failed to open file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");
    let encoded = encode(buffer.as_slice());
    write_meta(&encoded,pic_path);
    read_meta(&pic_path);
}

fn write_meta(buffer :&str,pic_path: &str) {
    // write    
    let file = pic_path;
    let meta = rexiv2::Metadata::new_from_path(&file).expect("error");
    meta.clear();
    meta.set_tag_string("Iptc.Application2.Keywords",buffer).expect("set tag fail");
    meta.save_to_file(file).expect("save fail");
}

fn read_meta(pic_path:&str) {
    // read
    let file = pic_path;
    let meta = rexiv2::Metadata::new_from_path(&file).expect("error");
    let encode_dll = meta.get_tag_string("Iptc.Application2.Keywords").expect("read fail due to the file size is too much.....");
    // println!("{:?}", encode_dll);
    let dll = base64::decode(encode_dll).expect("decode fail");
    let mut file = File::create("buffer.dll").expect("Failed to create file");
    let mut offset = 0;
    while offset < dll.len() {
    let bytes_written = file.write(&dll[offset..]).expect("Failed to write to file");
    offset += bytes_written;
    }
}