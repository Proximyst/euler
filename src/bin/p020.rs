use num::{BigUint, One as _, Zero as _};

fn main() {
    println!("{}", sum_fac(100));
}

fn factorial(n: BigUint) -> BigUint {
    if &n <= &BigUint::one() {
        return BigUint::one();
    }

    &n * factorial(&n - &BigUint::one())
}

fn sum_digits(n: BigUint) -> BigUint {
    let mut sum = BigUint::zero();
    let mut n = n.clone();

    while !n.is_zero() {
        sum += &n % &10u32;
        n /= 10u32;
    }

    sum
}

fn sum_fac(n: u64) -> BigUint {
    sum_digits(factorial(n.into()))
}

#[test]
fn test() {
    assert_eq!(BigUint::from(27u64), sum_fac(10));
}
