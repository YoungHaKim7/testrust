fn main() {
    let my_vec = vec![2, 3, 4];

    // if let
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is : {}", number);
        }
    }
}

//         match my_vec.get(index) {
//             Some(number) => println!("The number is : {}", number),
//             None => {}
//         }
//     }
// }
