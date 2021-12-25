// loops

fn main() {
    let mut counter = 0;
    let mut counter2 = 0;

    'first_loop: loop {
        // Give the first loop a name
        counter += 1;
        println!("The counter is: {}", counter);
        if counter == 9 {
            // Starts a second loop inside this loop
            println!("Now entering the second loop");

            'second_loop: loop {
                // now we are inside 'second_loop
                println!("The second counter is: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop; // Break out of 'first_loop so we can exit the program
                }
            }
        }
    }
}
