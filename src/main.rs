// nightly 1.61.0 rust사용하는 방법 unstable을 미리 써 볼 수 있다.
#![feature(panic_internals)]
use std::panic::{set_hook, take_hook};

fn main() {
    let mut impotant_code = 400;

    set_hook(Box::new(|panic_info| {
        panic_info.set_payload(9);
        println!("Didn't get a 200 code yet");
        println!(
            "Panic info: {:?}",
            panic_info.payload().downcast_ref::<&str>()
        );
    }));

    let my_number = "8876".parse::<i32>().unwrap();
    impotant_code = 200;
    let _ = take_hook();
    let other_number = "nthoent898760".parse::<i32>().unwrap();
}
