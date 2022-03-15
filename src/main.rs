use std::ops::Deref;

struct HoldsANumber(u8);

// struct HoldsANumber {
//     number1: u8,
//     number2: u16
// }

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main () {
    let my_number = HoldsANumber(20);
    println!("{}", *my_number + 20);
}
