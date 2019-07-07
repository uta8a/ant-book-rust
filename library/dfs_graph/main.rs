/// verify at 'https://atcoder.jp/contests/abc133/tasks/abc133_e'
mod vig {
    pub const MOD:u64 = 1_000_000_007;
    /// 0-indexed
    pub fn dfs(graph: &Vec<Vec<u64>>, k: u64, now: u64, from:i64) -> u64 {
        let mut color_number = k-2;
        if from ==-1 {
            color_number = k-1;
        }
        if k< graph[now as usize].len() as u64 {
            return 0;
        }
        let mut pp=1;
        for leaf in &graph[now as usize] {
            if *leaf as i64 == from {
                continue;
            }
            pp*=color_number;
            color_number-=1;
            pp%=MOD;
        }
        for leaf in &graph[now as usize] {
            if *leaf as i64 == from {
                continue;
            }
            pp*=dfs(graph,k, *leaf, now as i64);
            pp%=MOD;
        }
        pp
    }
}

fn main(){}