// find - Option<Self::Item> " I'll try to get it for you"
// position - Option<usize> " I'll try to find the position for you"
// cycle

fn main() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&n| n % 3 == 0));
    println!("{:?}", num_vec.iter().find(|&number| number % 2 == 30));
    println!("{:?}", num_vec.iter().position(|&num| num % 3 == 0));
    println!("{:?}", num_vec.iter().position(|&num| num * 2 == 30));
}
