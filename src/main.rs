use std::panic;
use std::panic::set_hook;

fn main() {
    let mut impotant_code = 400;
    set_hook(Box::new(|panic_info| {
        println!("Didn't get a 200 code yet");
    }));
    panic!("Oh the humanity!");
}
