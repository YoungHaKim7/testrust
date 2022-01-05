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

None.unwrap -> panic
Err.unwrap -> panic

fn main() {
    check_error(5).unwrap()
}
