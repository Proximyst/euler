use num::BigUint;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashSet;

static ABUNDANT_NUMBERS: Lazy<HashSet<u64>> = Lazy::new(|| {
    // We know abundant numbers exist in [12, 28123)
    (12u64..28123)
        .into_par_iter()
        .filter(|&n| is_abundant(n))
        .collect()
});

fn main() {
    println!(
        "{}",
        // [28123,->) are all sums
        (1u64..28123)
            .into_par_iter()
            .filter(|&i| !is_abundant_sum(i))
            .sum::<u64>()
    );
}

fn sum_divisors(u: u64) -> u64 {
    (1..u / 2 + 1)
        .into_par_iter()
        .filter(|n| u % n == 0)
        .sum::<u64>()
}

fn is_abundant(n: u64) -> bool {
    let divisors = BigUint::from(sum_divisors(n));
    divisors > BigUint::from(n)
}

fn is_abundant_sum(n: u64) -> bool {
    (1..=n / 2 + 1)
        .into_par_iter()
        .filter(|i| ABUNDANT_NUMBERS.contains(i))
        .any(|i| is_abundant(n - i))
}

#[test]
fn test() {
    for i in 1..12 {
        assert!(!is_abundant(i));
    }
    assert!(is_abundant(12));

    assert!(is_abundant_sum(24));
    for i in 28123..28133 {
        assert!(is_abundant_sum(i));
    }
}
