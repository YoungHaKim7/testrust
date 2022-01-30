// closure 
// anonymous function
// zero cost abstractions = Rust 가장 중요한 개념 Rust에서 추구하는 목표임.
// .iter().map().filter().inspect().collect() 이렇게 되도 속도가 빠르다. 

fn main () {
    let my_vec = vec![8,9,10];
    //unwrap_or_else 는 closure || pipes 를 쓸 수 있다.!! 최고!!
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{fourth}");

}