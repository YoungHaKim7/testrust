// any

fn main() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut counter = 0;
    let mut big_iter = big_vec.into_iter().rev();

    loop {
        counter += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }

    println!("Final counter is : {counter}");
}
