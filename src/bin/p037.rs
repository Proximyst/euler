use once_cell::sync::Lazy;
use rayon::prelude::*;

const MAX: usize = 10usize.pow(6);

static PRIMES: Lazy<Vec<u64>> = Lazy::new(|| {
    (10..MAX as u64)
        .into_par_iter()
        .filter(|&i| is_truncatable_prime(i))
        .collect()
});

fn main() {
    println!("{}", PRIMES.iter().take(11).sum::<u64>());
}

const fn is_truncatable_prime(u: u64) -> bool {
    if !is_prime(u) {
        return false;
    }

    let mut count = 0u32;
    let mut n = u;
    loop {
        n /= 10;
        count += 1;
        if n == 0 {
            break;
        }

        if !is_prime(n) {
            return false;
        }
    }

    let mut n = u;
    loop {
        n -= ((n / 10u64.pow(count - 1)) % 10) * 10u64.pow(count - 1);
        count -= 1;
        if n == 0 {
            break;
        }

        if !is_prime(n) {
            return false;
        }
    }

    true
}

const fn is_prime(u: u64) -> bool {
    if u <= 3 {
        return u > 1;
    }

    if u % 2 == 0 || u % 3 == 0 {
        return false;
    }

    let mut i = 5u64;
    while i.pow(2) <= u {
        if u % i == 0 || u % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}

#[test]
fn test() {
    assert!(is_truncatable_prime(3797));
}
