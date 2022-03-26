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
    println!("{input}");
    let mut result_vec = vec![];
    let mut push_string = String::new();
    for character in input.chars() {
        match character {
            '+' => {
                if !push_string.is_empty() {
                    // ""
                    result_vec.push(push_string.clone());
                    push_string.clear();
                }
            }
            '-' => {
                if push_string.contains('-') || push_string.is_empty() {
                    push_string.push(character);
                } else {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(character);
                }
            }
            number => {
                if push_string.contains('-') {
                    result_vec.push(push_string.clone());
                    push_string.clear();
                    push_string.push(number);
                } else {
                    push_string.push(number);
                }
            }
        }
    }
    result_vec.push(push_string);
    // vec!["1".to_string(), "-", "20"];
    let mut total = 0;
    let mut adds = true;
    let mut math_iter = result_vec.into_iter();
    while let Some(entry) = math_iter.next() {
        if entry.contains('-') {
            // --
            if entry.chars().count() % 2 == 1 {
                adds = match adds {
                    true => false,
                    false => true,
                };
                continue;
            } else {
                continue;
            }
        }
        if adds == true {
            total += entry.parse::<i32>().unwrap();
        } else {
            total -= entry.parse::<i32>().unwrap();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::math; // super = the space just above

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
    #[should_panic]
    fn panics_when_characters_not_right() {
        math("7 + please add seven");
    }

    fn main() {
        let my_number = math("7- + 9 -+ 10     +++----++++");
    }
}

fn main() {}
