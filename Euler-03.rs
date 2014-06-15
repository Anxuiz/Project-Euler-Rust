fn max<T:Ord>(a: T, b: T) -> T {
	if a > b {
		a
	} else {
		b
	}
}

fn main() {
	let mut n = 600_841_475_143u64;
	let mut largestFactor = 1;
	let mut factor = 2;
	while n > 1 {
		if n % factor == 0 {
			n /= factor;
			largestFactor = max(largestFactor, factor);
		} else {
			factor += 1;
		}
	}
	println!("{}", largestFactor);
}
