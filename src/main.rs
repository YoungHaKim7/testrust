use std::fs::read_to_string;
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    // let mut file = File::create("file.txt").unwrap();
    // let mut file = File::options()
    //     .read(true)
    //     .write(true)
    //     .append(true)
    //     .truncate(false)
    //     .open("file.txt")
    //     .unwrap();
    // file.write_all(b"Hello there\n").unwrap();

    // let mut file_string = String::new();
    // file.read_to_string(&mut file_string).unwrap();
    // println!("{file_string}");
    println!("{}", read_to_string("file.txt").unwrap());
    // let my_str = include_str!("file.txt");
}
