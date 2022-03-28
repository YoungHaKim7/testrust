#[derive(Clone)]
struct Calculator {
    results: Vec<String>,
    current_input: String,
    total: i32,
    adds: bool,
}

impl Calculator {
    fn new() -> Self {
        Self {
            results: vec![],
            current_input: String::new(),
            total: 0,
            adds: true,
        }
    }
}

// Let's see what breaks

fn main() {
    let mut calculator = Calculator::new();
    math("1 + 1");
}

const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if !input
        .chars()
        .all(|character| OKAY_CHARACTERS.contains(character))
    {
        panic!("Please only input numbers, +-, or spaces");
    }

    let input = input
        .trim_end_matches(|x| "+- ".contains(x))
        .chars()
        .filter(|character| *character != ' ')
        .collect::<String>();
    let mut calculator = Calculator::new();

    for character in input.chars() {
        match character {
            '+' => {
                if !calculator.current_input.is_empty() {
                    // ""
                    calculator.results.push(calculator.current_input.clone());
                    calculator.current_input.clear();
                }
            }
            '-' => {
                if calculator.current_input.contains('-') || calculator.current_input.is_empty() {
                    calculator.current_input.push(character);
                } else {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.current_input.clear();
                    calculator.current_input.push(character);
                }
            }
            number => {
                if calculator.current_input.contains('-') {
                    calculator.results.push(calculator.current_input.clone());
                    calculator.current_input.clear();
                    calculator.current_input.push(number);
                } else {
                    calculator.current_input.push(number);
                }
            }
        }
    }
    calculator.results.push(calculator.current_input);
    // vec!["1".to_string(), "-", "20"];
    let mut total = 0; // Now it's time to do math. Start with a total
    let mut adds = true; // true = add, false = subtract
    let math_iter = calculator.results.into_iter();
    for entry in math_iter {
        // Iter through the item
        if entry.contains('-') {
            // If it has a - character, check if it's even or odd
            // --
            if entry.chars().count() % 2 == 1 {
                adds = match adds {
                    true => false,
                    false => true,
                };
                continue; // Go to the next item
            } else {
                continue;
            }
        }
        if adds {
            total += entry.parse::<i32>().unwrap(); // If there is no '-', it must be a number. So we
        } else {
            total -= entry.parse::<i32>().unwrap();
            adds = true; // After subtracting, reset adds to true.
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - -1"), 2);
    }

    #[test]
    fn nine_plus_nine_minus_nine_minus_nine_is_zero() {
        assert_eq!(math("9+9-9-9"), 0); // This is a new test
    }

    #[test]
    fn eight_minus_nine_plus_nine_is_eight_even_with_characters_on_the_end() {
        assert_eq!(math("8    -  9     +9------++++"), 8); // This is a new test
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + please add seven");
    }
}
