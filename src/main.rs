// and
// or

fn main() {
    let first_try = vec![
        Some("sucess!"),
        None,
        Some("sucess!"),
        Some("sucess!"),
        None,
    ];
    let second_try = vec![
        None,
        Some("sucess!"),
        Some("sucess!"),
        Some("sucess!"),
        Some("sucess!"),
        Some("sucess!"),
        None,
    ];
    let third_try = vec![
        Some("sucess!"),
        Some("sucess!"),
        Some("sucess!"),
        Some("sucess!"),
        None,
    ];

    for index in 0..first_try.len() {
        println!(
            "{:?}",
            first_try[index]
                .and(second_try[index])
                .and(third_try[index])
        );
    }
}
