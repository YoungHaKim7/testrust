use std::thread;

fn main() {
    // std::thread(|| {
    //     println!("One second has passed");
    // });
    let mut join_vec = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| println!("I am printing something"));
        join_vec.push(handle);
    }

    // 기다려야한다고 생각되면 밑에 코드를 추가
    join_vec
        .into_iter()
        .for_each(|handle| handle.join().expect("Join Handle"));
}



