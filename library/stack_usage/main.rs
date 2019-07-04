fn main() {
    let mut v = Vec::<u64>::new();
    // **stack.push**
    v.push(1);
    v.push(2);
    v.push(100);

    println!("{:?}", v);

    // **out of bounds**
    // let existq = Some(v[4]); // this is panick.
    // println!("{:?}", existq);

    // **get top element**
    // let Some(top) = v.pop(); // None is not considered.
    if let Some(top) = v.pop() {
        println!("{}", top);
    }
    println!("{:?}", v);
}
