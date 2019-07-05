mod vig {
    // near 8 field search example
    fn dfs(field: &mut Vec<Vec<char>>, x: i64, y: i64, n: usize, m: usize) {
        if field[x as usize][y as usize] == 'W' {
            field[x as usize][y as usize] = '.';
            for i in [-1, 0, 1].iter() {
                for j in [-1, 0, 1].iter() {
                    let xx = x + i;
                    let yy = y + j;
                    if 0 <= xx && xx < n as i64 && 0 <= yy && yy < m as i64 {
                        if Some(field[xx as usize][yy as usize]) == Some('W') {
                            dfs(field, xx, yy, n, m);
                        }
                    }
                }
            }
        }
    }
    pub fn solve(field: &mut Vec<Vec<char>>, n: usize, m: usize) -> u64 {
        let mut ans = 0;
        for i in 0..n {
            for j in 0..m {
                if field[i][j] == 'W' {
                    dfs(field, i as i64, j as i64, n, m);
                    ans += 1;
                }
            }
        }
        ans
    }

}
fn main() {}
