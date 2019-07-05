fn main() {
    let a = 10;
    let b = 3;
    /// ceil, floor, round
    let ceil = a/b;
    let floor = (a+b-1)/b;
    let round = (a+(b/2))/b;
    println!("{} {} {}", ceil, floor, round);
}