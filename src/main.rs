// map
// and_then 맵이랑 비슷한데 조금 다음
// 실전에서 많이 쓰는 기능 map & and_then

// .map(|some_thing|some_thing+1) None 이면 None이 나옴.

fn main() {
    let some_output: Option<Vec<i32>> = None; // Some(vec![8, 9, 10]);

    let first =
        some_output.map(|some_vec| some_vec.iter().map(|num| num + 1).collect::<Vec<i32>>());

    println!("{first:?}");
}
