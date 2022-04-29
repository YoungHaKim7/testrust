use std::time::Duration;

const fn get_my_duration(seconds: u64, nanos: u32) -> Duration {
    Duration::new(seconds, nanos)
}
const fn check() {
    let my_array = [8, 9, 10];
}

const MY_DURATION: Duration = get_my_duration(100, 10);

fn main() {
    println!("{MY_DURATION:?}");
    let other_duration = get_my_duration(50, 100);
}
