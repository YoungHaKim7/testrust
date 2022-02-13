// peekable
// .peek()   .next()와 비슷한데 다음 아이템이 뭔지 미리 본다. 아이템을 미리 뺀다?

fn main() {
    // enuberate
    let just_numbers = vec![1, 5, 100];

    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap()); // 1번만 계속 나옴 next() 나올때까지 주구장창 1번 반복
        println!("I really love the number {}", number_iter.peek().unwrap());
        number_iter.next();
    }
}
