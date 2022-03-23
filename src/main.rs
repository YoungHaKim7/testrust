// Test-driven development
// fn some_function() {}
// fn some_other_function() {}
// use std::ops::Add;
fn math() -> i32 {
    9
}
// "1 + 1" -> 2

#[cfg(test)]
mod tests {
    use super::math; // super = the space just above

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!()
    }
}

fn main() {}

