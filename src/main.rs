// .inspect

fn main() {
    let new_vec = [8, 9, 10];

    let double_vec = new_vec
        .iter()
        .map(|x| x * 2)
        .filter(|&num| num > 17)
        .collect::<Vec<_>>();

    dbg!(double_vec);
}
