mod vig {
	const SIZE:usize = 1000;
	struct FibonacciMemo {
		memo: [u64;SIZE],
  	}
	impl FibonacciMemo{
		fn new() -> Self {
			FibonacciMemo{ memo: [0;SIZE] }
		}
		fn calculate(&mut self, n: usize) -> u64 {
			if n < 2 {
				return n as u64;
			}
			if self.memo[n] != 0 {
				return self.memo[n];
			}
			self.memo[n] = self.calculate(n-2)+self.calculate(n-1);
			self.memo[n]
		}
	}
	pub fn fibonacci(n:usize) -> u64 {
		let mut fib = FibonacciMemo::new();
		fib.calculate(n)
	}
}

fn main() {
	assert_eq!(vig::fibonacci(0), 0);
	assert_eq!(vig::fibonacci(5), 5);
	assert_eq!(vig::fibonacci(10), 55);
	assert_eq!(vig::fibonacci(11), 89);
}
