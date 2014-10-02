fn is_palendrome(n: uint) -> bool {
    let asciis = n.to_string().into_ascii();
    let len = asciis.len();
    for i in range(0, len / 2) {
        if asciis[i] != asciis[len - i - 1] {
            return false
        }
    }
    true
}

fn main() {
    let mut largest_palendrome = 0;
    for a in range(100, 999) {
        for b in range(a, 999) {
            let n = a * b;
            if is_palendrome(n) && n > largest_palendrome {
                largest_palendrome = n;
            }
        }
    } 
    println!("{}", largest_palendrome)
}
