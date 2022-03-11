// impl trait
// 
fn returns_a_closure() -> Box<dyn Fn(i32)> {
    Box::new(|x| println!("{x}"))
}

fn main () {}
