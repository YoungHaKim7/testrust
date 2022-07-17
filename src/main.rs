use std::{fs::File, io::Write};

fn main() {
    // let mut file = File::create("file.txt").unwrap();
    let mut file = File::options()
        .read(true)
        .write(true)
        .append(true)
        .truncate(false)
        .open("file.txt")
        .unwrap();
    file.write_all(b"Hello there\n").unwrap();
}
