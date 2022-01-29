//.iter().map(|item| item +1).collect()
//.iter().map(|item| {
//  let my_number = 7;
//  item + my_number
//  })
// .collect()

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