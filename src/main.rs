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
    fn clear(&mut self) {
        self.current_input.clear();
    }
    fn push_char(&mut self, character: char) {
        self.current_input.push(character);
    }
    fn math(mut self, input: &str) -> i32 {
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

        for character in input.chars() {
            match character {
                '+' => {
                    if !self.current_input.is_empty() {
                        // ""
                        self.results.push(self.current_input.clone());
                        self.clear();
                    }
                }
                '-' => {
                    if self.current_input.contains('-') || self.current_input.is_empty() {
                        self.push_char(character);
                    } else {
                        self.results.push(self.current_input.clone());
                        self.clear();
                        self.push_char(character);
                    }
                }
                number => {
                    if self.current_input.contains('-') {
                        self.results.push(self.current_input.clone());
                        self.clear();
                        self.current_input.push(number);
                    } else {
                        self.current_input.push(number);
                    }
                }
            }
        }
        self.results.push(self.current_input.clone());
        // vec!["1".to_string(), "-", "20"];

        let math_iter = self.results.clone().into_iter(); // todo! remove clone
        for entry in math_iter {
            // Iter through the item
            if entry.contains('-') {
                // If it has a - character, check if it's even or odd
                // --
                if entry.chars().count() % 2 == 1 {
                    self.adds = match self.adds {
                        true => false,
                        false => true,
                    };
                    continue; // Go to the next item
                } else {
                    continue;
                }
            }
            if self.adds {
                self.total += entry.parse::<i32>().unwrap(); // If there is no '-', it must be a number. So we
            } else {
                self.total -= entry.parse::<i32>().unwrap();
                self.adds = true; // After subtracting, reset adds to true.
            }
        }
        self.total
    }
}

// Let's see what breaks

fn main() {
    let mut calculator = Calculator::new();
    let res = calculator.math("1 + 1");
    println!("{res}");
}

const OKAY_CHARACTERS: &str = "1234567890+- ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one_is_two() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 - 2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("1 - -1"), 2);
    }

    #[test]
    fn nine_plus_nine_minus_nine_minus_nine_is_zero() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("9+9-9-9"), 0); // This is a new test
    }

    #[test]
    fn eight_minus_nine_plus_nine_is_eight_even_with_characters_on_the_end() {
        let mut calc = Calculator::new();
        assert_eq!(calc.math("8    -  9     +9------++++"), 8); // This is a new test
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        let mut calc = Calculator::new();
        calc.math("7 + please add seven");
    }
}
