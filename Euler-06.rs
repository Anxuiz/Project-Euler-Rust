use std::iter::AdditiveIterator;
use std::iter::range_inclusive;

fn sum_square_difference(n: u64) -> u64 {
    let sum = range_inclusive(1, n).sum();
    let squared_sum = sum * sum;
    let sum_squares = range_inclusive(1, n).map(|x| x * x).sum();
    squared_sum - sum_squares
}

fn main() {
    println!("{}", sum_square_difference(100));
}
