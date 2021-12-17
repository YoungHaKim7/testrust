// Control Flow and match

fn main() {
    let my_number = 5;
    let my_second_number = 10;
    if my_number == 5 && my_second_number == 10 {
        println!("They both match.");
    } else if my_number == 6 {
        // && and  /  || or
        println!("It's six");
    } else {
        println!("It's a different number");
    }
}
