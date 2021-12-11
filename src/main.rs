fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            // 102 / 50  2 remainder 2
            break;
        }
    }
    counter
}

fn main() {
    let my_number;

    {
        // 복잡한 코딩 후
        let x = loop_then_return(43);
        my_number = x
    }

    println!("{}", my_number);
}
