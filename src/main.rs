use std::ops::Deref;

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main () {
let x = DerefExample { value: 'a' };
assert_eq!('a', *x);

}

// Deref trait
// struct HoldsANumber(u8);
//

// 
// fn main () {
//     let my_number = HoldsANumber(20);
//     println!("{}", *my_number + 20);
// }
