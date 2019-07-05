macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
mod vig {
    fn dfs(field: &mut Vec<Vec<char>>, x: i64, y: i64, n: usize, m: usize) {
        if field[x as usize][y as usize] == 'W' {
            field[x as usize][y as usize] = '.';
            for i in [-1, 0, 1].iter() {
                for j in [-1, 0, 1].iter() {
                    let xx = x + i;
                    let yy = y + j;
                    if 0 <= xx
                        && xx < n as i64
                        && 0 <= yy
                        && yy < m as i64
                    {
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
fn main() {
    input! {
        n: usize,
        m: usize,
        infield: [chars;n],
    }
    let mut field = infield;
    let ans = vig::solve(&mut field, n, m);
    println!("{}", ans);
}
