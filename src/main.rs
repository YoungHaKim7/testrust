// attribute
const HIGH_SCORE: i32 = 20; // global scope
                            // static

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    print_high_score();
}
