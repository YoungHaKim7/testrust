// uninitialized variable
// control flow

// possibly uninitialized = maybe doesn't have a value yet

fn main() {
    let my_number = {
        // 복잡한 코딩 후
        let x = 9;
        x + 9
    };

    println!("{}", my_number);
}
