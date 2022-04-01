use std::panic;
use std::panic::set_hook;

fn main() {
    let mut impotant_code = 400;
    set_hook(Box::new(|panic_info| {
        println!("Didn't get a 200 code yet");
        println!(
            "Panic info: {:?}",
            panic_info.payload().downcast_ref::<&str>()
        );
    }));

    panic!("Oh the humanity!");

    let my_number = "thoe8876".parse::<i32>().unwrap();
}
