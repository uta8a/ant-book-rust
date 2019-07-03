mod vig {
	pub fn fact(n:u64) -> u64 {
		if n<=1 {
			return 1;
		}
		(n * fact(n - 1)) as u64
	}
}

fn main(){
	assert_eq!(vig::fact(0), 1);
	assert_eq!(vig::fact(1), 1);
	assert_eq!(vig::fact(10), 3628800);
	assert_eq!(vig::fact(12), 479001600);
}
