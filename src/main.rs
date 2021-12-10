// attribute
const HIGH_SCORE: i32 = 20; // global scope
static mut LOW_SCORE: i32 = 0; //unsafe
                               // 'static lifetime

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    print_high_score();

    let my_name = "David"; // &'static str
    LOW_SCORE = 1;
}
