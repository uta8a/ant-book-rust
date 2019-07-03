// p31 memo
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
// ref: https://muunyblue.github.io/115c51eb37365df2d4f4e2482b964822.html#_6
mod vig {
    pub const MAX_N: usize = 1000;
    struct FibMemo {
        memo: [i32; MAX_N],
    }

    impl FibMemo {
        fn new() -> Self {
            FibMemo { memo: [0; MAX_N] }
        }

        fn calc(&mut self, n: i32) -> i32 {
            if n < 2 {
                return n;
            }
            if self.memo[n as usize] != 0 {
                return self.memo[n as usize];
            }
            self.memo[n as usize] = self.calc(n - 2) + self.calc(n - 1);
            self.memo[n as usize]
        }
    }

    pub fn fib_memo(n: i32) -> i32 {
        let mut f = FibMemo::new();
        f.calc(n)
    }
}
fn main() {
    input! {
        n:i32, // n<47
    }
	println!("{}", vig::fib_memo(n));
	println!("{}", vig::MAX_N);
}
