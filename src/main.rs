// uninitialized variable
// control flow

// possibly uninitialized = maybe doesn't have a value yet

fn main() {
    let my_number;

    {
        my_number = 9;
    }

    println!("{}", my_number);
}
