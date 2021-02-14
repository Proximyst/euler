use num::{ Zero as _, BigUint };

fn main() {
    println!("{}", calc(1000));
}

fn calc(exp: u32) -> BigUint {
    let mut n = BigUint::from(2u32).pow(exp);
    let mut sum = BigUint::zero();

    while !n.is_zero() {
        sum += &n % &10u32;
        n /= 10u32;
    }

    sum
}

#[test]
fn test() {
    assert_eq!(BigUint::from(26u32), calc(15));
}
