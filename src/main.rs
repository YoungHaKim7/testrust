// CONTROL FLOW

fn main() {
    let my_number = 5;
    if my_number % 2 == 1 && my_number > 0 {
        // % 2 means the number that ramains after diving by two
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }
}
