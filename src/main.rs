// clippy = linter
//
fn print_vec_ref(input: &Vec<i32>) {
    if input.is_empty() {
        println!("Vec is empty");
    } else {
        for num in input {
            println!("{num}");
        }
    }
}

// cargo clippy
fn main() {
    let my_vec = vec![8, 9, 10];
    print_vec_ref(&my_vec);

    let mut done = false;
    let mut counter = 0;

    // while done == false 는 while true라는 뜻인데 //while done == false는 안 좋은 code
    while !done {
        counter += 1;

        if counter > 10 {
            done = true;
        }
    }

    let some_variable = Some(9);

    if let Some(number) = some_variable {
        println!("We got {number}");
    }
}

