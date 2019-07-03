mod vig {
	pub fn fibonacci(n:u64)->u64 {
		if n <=1 {
			return 1;
		}
		fibonacci(n-1)+fibonacci(n-2)
	}
}
fn main(){
	assert_eq!(vig::fibonacci(0), 1);
	assert_eq!(vig::fibonacci(4), 5);
	assert_eq!(vig::fibonacci(9), 55);
	assert_eq!(vig::fibonacci(10), 89);
}
