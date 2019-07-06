mod vig {
    /// dfs return u64
    /// prune
    pub fn dfs(p: &Vec<(u64, u64)>, index: usize, now: u64, m: u64) -> u64 {
        let mut res = 0;
        if index==p.len() {
            return 1;
        }
        let (div, exp) = p[index];
        let mut n_now = now;
        for _ in 0..exp+1 {
            res+=dfs(&p, index+1, n_now, m);
            n_now*=div;
            if n_now > m {
                break;
            }
        }
        res
    }
}

fn main() {
    let p = vec![(2,4), (3, 2)];
    let m = 50;
    let ans = vig::dfs(&p, 0, 1, m);
    println!("{}", ans); // 13
}
// https://yukicoder.me/problems/no/847