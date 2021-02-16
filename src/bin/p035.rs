use once_cell::sync::Lazy;
use rayon::prelude::*;

const MAX: usize = 10usize.pow(6);

static PRIMES: Lazy<Vec<bool>> = Lazy::new(|| {
    let mut arr = Vec::with_capacity(MAX);

    (0..MAX)
        .into_par_iter()
        .map(|i| is_prime(i))
        .collect_into_vec(&mut arr);

    arr
});

fn main() {
    println!(
        "{}",
        PRIMES
            .par_iter()
            .enumerate()
            .filter(|&(idx, &b)| b && is_circular_prime(idx))
            .count()
    );
}

const fn is_prime(u: usize) -> bool {
    if u <= 3 {
        return u > 1;
    }

    if u % 2 == 0 || u % 3 == 0 {
        return false;
    }

    let mut i = 5usize;
    while i.pow(2) <= u {
        if u % i == 0 || u % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}

const fn count_digits(mut n: usize) -> usize {
    let mut count = 0;

    while n != 0 {
        count += 1;
        n /= 10;
    }

    count
}

const fn circulate(u: usize) -> usize {
    let digit = u % 10;
    let pow = count_digits(u);

    (u / 10) + digit * 10usize.pow(pow as u32 - 1)
}

fn is_circular_prime(u: usize) -> bool {
    let mut variant = u;
    loop {
        if !PRIMES[variant] {
            break false;
        }

        variant = circulate(variant);
        if variant == u {
            break true;
        }
    }
}

#[test]
fn test() {
    assert_eq!(719, circulate(197));
    assert_eq!(971, circulate(719));
    assert_eq!(197, circulate(971));

    assert!(is_circular_prime(197));
    assert!(is_circular_prime(971));
    assert!(is_circular_prime(719));
}
