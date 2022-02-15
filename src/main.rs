// .inspect ->무조건 unit type이 나와야 한다. () 이거

fn main() {
    let new_vec = [8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|first_item| {
            dbg!(first_item);
        }) // ()
        .map(|x| x * 2)
        .inspect(|next_item| {
            dbg!(next_item);
        })
        .filter(|num| *num > 17)
        .collect::<Vec<_>>();
}
