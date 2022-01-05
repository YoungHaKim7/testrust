fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

// .is_ok()
// .is_err()

// .is_some()
// .is_none()

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    if check_error(5).is_ok() {
        println!("It's okay, guys!")
    } else {
        println!("It's an error, guys!")
    }
}
