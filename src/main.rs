fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

// .is_ok()
// .is_err()
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    match check_error(5) {
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error"),
    }
}
