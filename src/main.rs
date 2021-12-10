// attribute
const HIGH_SCORE: i32 = 20; // global scope
static mut LOW_SCORE: i32 = 0; //unsafe

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    print_high_score();

    unsafe {
        LOW_SCORE = 1;
    }
}
