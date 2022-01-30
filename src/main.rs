// closure 
// anonymous function
// zero cost abstractions = Rust 가장 중요한 개념 Rust에서 추구하는 목표임.
// .iter().map().filter().inspect().collect() 이렇게 되도 속도가 빠르다. 

fn main () {
    let my_vec = vec![8,9,10,9];
    //unwrap_or_else 는 closure || pipes 를 쓸 수 있다.!! 최고!!
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });
    // [3]_4번째를 출력하고 4번째가 없으면 [0]1번째를 출력해서 1번째가 없으면 그래도 &[0] 첫번째를 출력하라는 Code
    // 4번째를 만들면 4번째인 9가 출력되는 Code

    println!("{fourth}");

}