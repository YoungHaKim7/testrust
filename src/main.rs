// cycle  계속 똑같은 것만 나오는거임 . iterator 끝나지 않는 trait
// iterator를 만든다음에 완전 다른 타입이 나온다.clcle iterator로 만든다.

fn main() {
    // .take(6)
    let even_odd = vec!["even", "odd"].into_iter().cycle();

    let even_odd_vec = (0..6) // Ranges are iterators
        .zip(even_odd)
        .collect::<Vec<(i32, &str)>>();

    println!("{even_odd_vec:?}");
}
