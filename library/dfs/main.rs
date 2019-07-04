mod vig {
	pub fn dfs(a: &Vec<i64>, n: usize, k: i64, i: usize, sum: i64) -> bool {
		if i==n {
			if sum==k {
				return true;
			}else{
				return false;
			}
		}
		if dfs(a,n,k,i+1,sum) {
			return true;
		}
		if dfs(a,n,k,i+1, sum+a[i]) {
			return true;
		}
		false
	}
}

fn main() {
	let a = vec![1,2,4,7];
	let n = 4;
	assert_eq!(vig::dfs(&a, n, 13, 0, 0), true);
	assert_eq!(vig::dfs(&a, n, 15, 0, 0), false);
}
