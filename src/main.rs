// VecDeque 벡데크 Vec이랑 비슷하지만 벡이라 똑같이 쓰면 느리다. ㅜㅜ
// 어떤 경우는 Vec보다 더 빠르다!!
// index[0]지울때 Vec은 느리지만 빠른건 VecDeque빠르게 가능하다 큰 Vec일수록 더 빠름

use std::collections::VecDeque;

fn main() {
    // 0123  .remove(0)
    let mut my_vec = VecDeque::from(vec![0; 600_000]); // .pop 8,9 이렇게 줄고  .push()는 꽉차면 더 늘려서 넣어줌 Vec::with_capacity(10)
    for i in 0..600000 {
        my_vec.pop_front();
    }
}
