// and
// or

fn main() {
    let first_try = vec![Some("s!"), None, Some("s!"), Some("s!"), None];
    let second_try = vec![
        None,
        Some("t!"),
        Some("t!"),
        Some("t!"),
        Some("t!"),
        Some("t!"),
        None,
    ];
    let third_try = vec![Some("u!"), Some("u!"), Some("u!"), Some("u!"), None];

    for index in 0..first_try.len() {
        println!(
            "{:?}",
            first_try[index]
                .and(second_try[index])
                .and(third_try[index])
        );
    }
}
