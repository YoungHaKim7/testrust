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

// cargo clippy
fn main() {
    let my_vec = vec![8, 9, 10];
    print_vec_ref(&my_vec);

    let mut done = false;
    let mut counter = 0;

    while done == false {
        counter += 1;

        if counter > 10 {
            done = true;
        }
    }

    let some_variable = Some(9);

    match some_variable {
        Some(number) => println!("We got a {number}"),
        _ => {}
    }
}

