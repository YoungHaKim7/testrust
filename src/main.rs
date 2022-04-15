// overflowing

fn add(one: u8, two: u8) -> u8 {
    one + two
}

fn main() {
    println!("200 + 200 = {}", add(200, 200));
}


