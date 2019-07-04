mod vig {
	pub fn fibonacci(n:u64)->u64 {
		if n <=1 {
			return n;
		}
		fibonacci(n-1)+fibonacci(n-2)
	}
}
fn main(){
	assert_eq!(vig::fibonacci(1), 1);
	assert_eq!(vig::fibonacci(5), 5);
	assert_eq!(vig::fibonacci(10), 55);
	assert_eq!(vig::fibonacci(11), 89);
}
