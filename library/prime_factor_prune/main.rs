mod vig {
    /// prune prime_factor
    /// factor over m, prune
    pub fn prime_factor(mut n: u64, k:u64, m:u64) -> Vec<(u64, u64)> {
        let mut ans: Vec<(u64,u64)> = vec![];
        for i in 2..((n as f64).sqrt() as u64 + 2) {
            if n%i==0 {
                let div = i;
                let mut exp = 0;
                if div > m {
                    continue;
                }
                loop {
                    if n%i!=0 {
                        break;
                    }
                    n /=div;
                    exp+=1;
                }
                
                ans.push((div, exp*k));
            }
        }
        if n>1  && n <=m {
            ans.push((n, k));
        }
        ans
    }
}
fn main() {
    let n = 12;
    let ans = vig::prime_factor(n, 3, 50);
    println!("{:?}", ans); // [(2, 6), (3, 3)]
}