// CONTROL FLOW

fn main() {
    let my_number: u8 = 5;
    let something = match my_number {
        8 => 10,
        10 => 200,
        _ => 55,
    };

    println!("{}", something);
}
