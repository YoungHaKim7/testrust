// dbg! // debug = quick print
// AWS
fn main() {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);

    dbg!();

    let new_vec = dbg!(vec![8, 9, 10]);

    dbg!();

    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
}
