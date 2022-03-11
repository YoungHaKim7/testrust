// impl trait

// 
// fn returns_a_closure() -> Box<dyn Fn(i32)> {
//     Box::new(|x| println!("{x}"))
// }
// same code (About impl Trait)
fn returns_a_closure() -> impl Fn(i32) {
    |x| println!("{x}")
}


fn main () {
    let my_number = 9;
    let my_closure = returns_a_closure();
    my_closure(my_number);
}
