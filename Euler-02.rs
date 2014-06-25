fn main() {
	let mut n1 = 1;
	let mut n2 = 2;
	let mut sum = 2;
	while n1 + n2 <= 4_000_000u {
		let next = n1 + n2;
		if next % 2 == 0 {
			sum += next;
		}
		n1 = n2;
		n2 = next;
	}
	println!("{}", sum);
}
