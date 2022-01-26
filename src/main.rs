// Iterator = a collection of things that you can call .next() on
// Iterator많이 쓰는 기능!!!
// .iter() - iterator of references &T
// .iter_mut() - iterator of mutable references &mut T
// .into_iter() - consuming iterator 이터레이터가 사라짐? - iterator를 계속 안 쓰게 된다면, 이게 젤 편함references를 안봐도 되기 때문!!
// map이라는 기능은 vec안에 있는 것을 하나씩 가지고 자기 맘대로 할 수 있다.
// map |x|  이름을 정해주는 거임.  |num| 이렇게 해도 됨

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();

    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|num| *num += 100);

    println!("{vector1_a:?}");
    println!("{vector2:?}");
    println!("{:?}", vector1_b);
}
