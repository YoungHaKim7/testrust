// clippy = linter
//
fn print_vec_ref(input: &Vec<i32>) {
    if input.len() == 0 {
        println!("Vec is empty");
    } else {
        for num in input {
            println!("{num}");
        }
    }
}

fn main() {
    let my_vec = vec![8, 9, 10];
    print_vec_ref(&my_vec);
}

