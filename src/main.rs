// BinaryHeap - 가장 큰 숫자가 먼저 출력되는게 BinaryHeap의 특징이다.
// 돈을 가장 많이 낸 사람이 먼저 출력 받을 수 있게 활용이 가능함. 최고!! vip관리할때 쓰는 BinaryHeap
// priority queue
use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        println!(
            "Popped off {}. Remaining numbers are : {:?}",
            number,
            show_remainder(&my_heap)
        );
    }
}
