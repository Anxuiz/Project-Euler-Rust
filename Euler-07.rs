use std::mem::replace;

struct Primes {
    curr: uint,
    primes: Vec<uint>,
}

impl Primes {
    fn is_prime(&self, n: uint) -> bool {
        !self.primes.iter().any(|&prime| n % prime == 0)
    }

    fn next_prime(&self) -> uint {
        if self.curr == 2 {
            3
        } else {
            let mut n = self.curr + 2;
            while !self.is_prime(n) {
                n += 2;
            }
            n
        }
    }

}

impl Iterator<uint> for Primes {
    fn next(&mut self) -> Option<uint> {
        let next = self.next_prime();
        self.primes.push(next);
        Some(replace(&mut self.curr, next))
    }
}

fn primes() -> Primes {
    Primes { curr: 2, primes: Vec::new() }
}

fn main() {
    println!("{}", primes().nth(10_001 - 1).unwrap());
}
