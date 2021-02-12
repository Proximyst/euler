fn main() {
    println!("{}", sum_primes_below(2 * 10u64.pow(6)));
}

fn sum_primes_below(n: u64) -> u64 {
    (1..n).filter(is_prime).sum()
}

fn is_prime(&u: &u64) -> bool {
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
fn test_below_10() {
    assert_eq!(17, sum_primes_below(10));
}
