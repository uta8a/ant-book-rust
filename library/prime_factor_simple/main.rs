mod vig {
    /// prime_factor
    /// usage: for (div, exp) in prime_factor(n) {...}
    pub fn prime_factor(mut n: u64) -> Vec<(u64, u64)> {
        let mut ans: Vec<(u64,u64)> = vec![];
        for i in 2..((n as f64).sqrt() as u64 + 2) {
            if n%i==0 {
                let div = i;
                let mut exp = 0;
                loop {
                    if n%i!=0 {
                        break;
                    }
                    n /=div;
                    exp+=1;
                }
                ans.push((div, exp));
            }
        }
        if n>1 {
            ans.push((n, 1));
        }
        ans
    }
}
fn main() {
    let n = 12;
    let ans = vig::prime_factor(n);
    println!("{:?}", ans); // [(2, 2), (3, 1)]
}