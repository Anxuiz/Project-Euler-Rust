use std::iter::range_inclusive;

fn triplet(sum: uint) -> Option<uint> {
    for a in range_inclusive(0, sum - 1 - 2) {
        for b in range_inclusive(a + 1, sum - 2) {
            let c = sum - b - a;
            if c <= b {
                break;
            }

            if a * a + b * b == c * c {
                return Some(a * b * c);
            }
        }
    }
    None
}

fn main() {
    match triplet(1000) {
        Some(product) => println!("{}", product),
        None          => println!("No triplet found"),
    }
}
