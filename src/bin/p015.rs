use num::{ One as _, BigUint };

fn main() {
    println!("{}", calc(20));
}

fn factorial(n: &BigUint) -> BigUint {
    if n <= &BigUint::one() {
        return BigUint::one();
    }

    n * factorial(&(n - &1u32))
}

fn ncr(n: BigUint, k: BigUint) -> BigUint {
    factorial(&n) / (factorial(&k) * factorial(&(&n - &k)))
}

fn calc(size: u128) -> BigUint {
    let size = BigUint::from(size);

    // nCr(2n, n)
    ncr(2u32 * &size, size)
}

#[test]
fn test_simple() {
    assert_eq!(BigUint::from(6u32), calc(2));
}
