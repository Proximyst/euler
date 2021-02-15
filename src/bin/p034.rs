use num::{BigUint, One as _, Zero as _};
use rayon::prelude::*;

fn main() {
    println!(
        "{}",
        (10u64..1_000_000)
            .into_par_iter()
            .filter(|&n| factorial_digits(BigUint::from(n)) == BigUint::from(n))
            .sum::<u64>()
    );
}

fn factorial(n: BigUint) -> BigUint {
    if &n < &BigUint::one() {
        return BigUint::one();
    }

    &n * factorial(&n - BigUint::one())
}

fn factorial_digits(mut n: BigUint) -> BigUint {
    let mut sum = BigUint::zero();

    while !n.is_zero() {
        sum += factorial(&n % &10u32);
        n /= 10u32;
    }

    sum
}

#[test]
fn test() {
    assert_eq!(
        BigUint::from(145u32),
        factorial_digits(BigUint::from(145u32))
    );
}
