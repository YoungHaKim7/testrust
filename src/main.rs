// .take_while // take
// .by_ref
// .skip_while
// .map_while
// .cloned
// lifetime 레퍼런스가 살아있는지 죽었는지 모를때 .cloned를 쓴다.
// --------------
// .chunks // 1,2,3,4,5,6,7,8,9
// .windows // 1,2,3 -이걸 쓰면 굉장한 크기의 숫자를 만들수 있다.

fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_vec.chunks(3) {
        println!("chunk is {chunk:?}");
    }

    for window in num_vec.windows(3) {
        println!("Windows is {window:?}");
    }
}
