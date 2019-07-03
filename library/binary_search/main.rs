mod vig {
    pub trait S {
        fn binary_search(&self, key: i64) -> i64;
    }
    impl S for Vec<i64> {
        fn binary_search(&self, key: i64) -> i64 {
            let mut ng: i64 = -1;
            let mut ok: i64 = self.len() as i64;
            loop {
                if (ok - ng).abs() <= 1 {
                    break;
                }
                let mid = (ok + ng) / 2;
                if self[mid as usize] >= key {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        }
    }
}

fn main() {
    let a: Vec<i64> = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    assert_eq!(vig::S::binary_search(&a, 51), 3);
    assert_eq!(vig::S::binary_search(&a, 1), 0);
    assert_eq!(vig::S::binary_search(&a, 910), 9);
    assert_eq!(vig::S::binary_search(&a, 52), 6);
    assert_eq!(vig::S::binary_search(&a, 0), 0);
    assert_eq!(vig::S::binary_search(&a, 911), 10);
	
}
