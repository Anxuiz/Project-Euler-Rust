fn main() {
	let mut sum = 0;
	for i in range(1u, 1000) {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
	}
	println!("{}", sum);
}
