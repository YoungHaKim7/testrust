// clippy = linter
//
fn print_vec_ref(input: &Vec<i32>) {
    if input.len() == 0 {
        println!("Vec is empty");
    } else {
        input.iter().for_each(|num| println!("{num}"));
        }
    
}

fn main() {
}

