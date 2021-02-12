use std::iter::Iterator;

fn main() {
    println!("{}", even_fibonacci_sum(4 * 10u64.pow(6)));
}

struct Fibonacci(u64, u64);

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.0 + self.1;
        self.0 = self.1;
        self.1 = sum;

        Some(self.0)
    }
}

fn is_even(&u: &u64) -> bool {
    u % 2 == 0
}

fn even_fibonacci_sum(limit: u64) -> u64 {
    Fibonacci(0, 1)
        .filter(is_even)
        .take_while(|&n| n <= limit)
        .sum()
}
