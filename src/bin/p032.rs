use num::{BigUint, Zero as _};
use std::collections::HashSet;

fn main() {
    let mut nums = HashSet::new();

    for i in 1..=10u64.pow(4) {
        let i = BigUint::from(i);
        let len = count_digits(&i) as u32;
        for j in 10u64.pow(4 - len)..=10u64.pow(6 - len) {
            let j = BigUint::from(j);

            let product = &i * &j;
            let concat = concatenate_number(&i, &j);
            let concat = concatenate_number(&concat, &product);

            if is_pandigital(&concat) {
                nums.insert(product);
            }
        }
    }

    println!("{}", nums.into_iter().sum::<BigUint>());
}

fn count_digits(n: &BigUint) -> usize {
    let mut n = n.clone();
    let mut count = 0;

    while !n.is_zero() {
        count += 1;
        n /= 10u32;
    }

    count
}

fn is_pandigital(n: &BigUint) -> bool {
    let mut n = n.clone();
    let mut array = [0; 9];

    while !n.is_zero() {
        let digit = &n % &10u32;
        if digit.is_zero() {
            return false;
        }

        array[digit.to_u32_digits()[0] as usize - 1] += 1;
        n /= 10u64;
    }

    array.iter().all(|&i| i == 1)
}

fn concatenate_number(a: &BigUint, b: &BigUint) -> BigUint {
    b + a * &10u64.pow(count_digits(b) as u32)
}

#[test]
fn test() {
    assert!(is_pandigital(&123456789u32.into()));
    assert!(!is_pandigital(&123456779u32.into()));
    assert!(!is_pandigital(&1234567890u32.into()));
    assert_eq!(
        concatenate_number(&12u32.into(), &34u32.into()),
        1234u32.into()
    );
    assert_eq!(
        concatenate_number(&1234u32.into(), &5678u32.into()),
        12345678u32.into()
    );
}
