// any

fn main() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    // rev() reverse
    let mut iterator = big_vec.iter().rev();
    assert_eq!(Some(&5), iterator.next());
    assert_eq!(Some(&6), iterator.next());
}
