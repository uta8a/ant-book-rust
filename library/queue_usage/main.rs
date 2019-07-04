use std::collections::VecDeque;
fn main() {
    let mut v = VecDeque::<u64>::new();
    // **queue.push**
    v.push_back(1);
    v.push_back(10);
    v.push_back(100);
    v.push_back(13);
    v.push_back(91);
    println!("{:?}", v);

    // **queue.pop**
    if let Some(front) = v.pop_front() {
        println!("{}", front);
    }
    println!("{:?}", v);
}
