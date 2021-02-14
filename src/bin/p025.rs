use num::{BigUint, One as _, Zero as _};
use std::iter::Iterator;

fn main() {
    println!(
        "{}",
        Fibonacci::default()
            .map(|k| count_digits(k))
            .enumerate()
            .find(|(_, k)| k >= &BigUint::from(1000u32))
            .unwrap()
            .0 + 1
    );
}

fn count_digits(mut n: BigUint) -> BigUint {
    let mut count = BigUint::zero();

    while !n.is_zero() {
        count += BigUint::one();
        n /= 10u32;
    }

    count
}

struct Fibonacci {
    n: BigUint,
    k: BigUint,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Fibonacci {
            n: BigUint::zero(),
            k: BigUint::one(),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let res = &self.n + &self.k;
        std::mem::swap(&mut self.n, &mut self.k);
        self.k = res;

        Some(self.n.clone())
    }
}

#[test]
fn test() {
    assert_eq!(
        vec![1u32, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
            .into_iter()
            .map(BigUint::from)
            .collect::<Vec<_>>(),
        Fibonacci::default().take(12).collect::<Vec<_>>()
    );
    assert_eq!(
        BigUint::from(3u32),
        count_digits(Fibonacci::default().nth(11).unwrap())
    );
}
