mod vig {
    pub struct A {
        pub dp: [u64; 5],
    }
    impl A {
        fn new() -> Self {
            A { dp: [0; 5] }
        }
        pub fn ret(&self, n: usize) -> u64 {
            self.dp[n]
        }
    }
    pub fn retall(n: usize) -> u64 {
        let a = A::new();
        a.ret(n)
    }
}

fn main() {
    let case1 = vig::retall(0);
    println!("{}", case1); // 0 
    let a2 = vig::A {
        dp: [1337, 0, 0, 0, 0],
    };
    println!("{}", case1); // 0
    let case2 = vig::retall(0);
    println!("{}", case2); // 0
    println!("{}", a2.ret(0)); // 1337 initialized by a2
}
