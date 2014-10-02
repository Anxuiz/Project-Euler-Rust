use std::cmp::min;
use std::iter::range_step;

fn gcd(a: int, b: int) -> int {
    let min = min(a, b);
    for i in range_step(min + 1, 2, -1) {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }
    1
}

fn main() {
    let mut result = 1;
    for i in range(1, 20) {
        let gcd = gcd(result, i);
        result *= i / gcd;
    }
    println!("{}", result);
}
