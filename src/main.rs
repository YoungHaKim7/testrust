use std::{fs::File, io::Write};

fn main() {
    let mut file = File::create("file.txt").unwrap();
    file.write_all("Hello there\n".as_bytes()).unwrap();
}
