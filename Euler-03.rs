fn max<T:Ord>(a: T, b: T) -> T {
	if a > b {
		a
	} else {
		b
	}
}

fn main() {
	let mut n = 600_851_475_143u64;
	let mut largest_factor = 1;
	let mut factor = 2;
	while n > 1 {
		if n % factor == 0 {
			n /= factor;
			largest_factor = max(largest_factor, factor);
		} else {
			factor += 1;
		}
	}
	println!("{}", largest_factor);
}
