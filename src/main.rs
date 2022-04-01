use std::panic;
use std::panic::set_hook;

fn main() {
    set_hook(Box::new(|panic_info| {
        let x = 9;
        println!("don't forget about x: {x}");
    }));
    panic!("Oh the humanity!");
}
