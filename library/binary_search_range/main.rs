mod vig {
    /// usage: f is reference of closure which return bool if f(low) is false/true and f(high) is true, return min x which f(x)==true
    /// vig::S::binary_search(low, high, &|x|{x>=k})
    pub fn binary_search(low: u64, high: u64, f: &Fn(u64) -> bool) -> u64 {
        let mut ng: u64 = low;
        let mut ok: u64 = high;
        loop {
            if ok - ng <= 1 {
                break;
            }
            let mid = (ok + ng) / 2;
            if f(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}
fn main() {
    let res = vig::binary_search(0, 10, &|x| {x >= 5});
    println!("{}", res); // 5
}