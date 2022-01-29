// closure = anomymous functions that capture their environment
// a|nonymous = no name
// enclose => 주변에 변수가 있으면 맘대로 쓸 수 있다.
// enclose 쓰는 방법은 || = pipes

fn main () {
    
    let my_closure = || {
        let my_number = 7;
        let other_number = 10;
        println!("The two numbers are {my_number} and {other_number}");
        my_number + other_number
    };

    let my_var = my_closure();
    println!("{my_var}");
}