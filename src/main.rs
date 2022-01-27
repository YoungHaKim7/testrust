// associated = goes together
// the type that goes together with the trait


fn main() {
    let my_vec = vec!['a','b','거', '魔'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a')); // 2개 같아야 패닉이 안됨. test할 때 쓰는 기능
    assert_eq!(my_vec_iter.next(), Some(&'b')); 
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'魔'));
    assert_eq!(my_vec_iter.next(), None); 
}
