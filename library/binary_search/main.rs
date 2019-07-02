mod vig {
    pub trait S {
        fn binary_search(&self, key: u64) -> usize;
    }
    impl S for Vec<u64> {
        fn binary_search(&self, key: u64) -> usize {
            let mut ng: i64 = -1;
            let mut ok: i64 = self.len() as i64;
            loop {
                if (ok - ng).abs() <= 1 {
                    break;
                }
                let mid = ((ok + ng) / 2) as usize;
                if self[mid] >= key {
                    ok = mid as i64;
                } else {
                    ng = mid as i64;
                }
            }
            ok as usize
        }
    }
}

fn main() {
    let a: Vec<u64> = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    println!("{}", vig::S::binary_search(&a, 51));
    println!("{}", vig::S::binary_search(&a, 1));
    println!("{}", vig::S::binary_search(&a, 910));
    println!("{}", vig::S::binary_search(&a, 52));
    println!("{}", vig::S::binary_search(&a, 0));
    println!("{}", vig::S::binary_search(&a, 911));
}
