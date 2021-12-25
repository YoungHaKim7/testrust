// loops

fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("The counter is: {}", counter);
        if counter == 5 {
            break;
        }
    }
}
