fn main() {
    println!(
        "{}",
        (1..=1_000_000).filter(|&i| is_palindromic(i)).sum::<u64>()
    );
}

const fn is_palindromic_base10(u: u64) -> bool {
    if u % 10 == 0 {
        return false;
    }

    if u % 10 == u {
        return true;
    }

    let mut n = u;
    let mut rev = 0;

    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }

    u == rev
}

const fn is_palindromic_base2(u: u64) -> bool {
    if u & 1 == 0 {
        // Cannot start with 0 so cannot end with 0.
        return false;
    }

    if (u + 1).is_power_of_two() {
        // The entire number must be 1s.
        return true;
    }

    u.reverse_bits() >> u.leading_zeros() == u
}

const fn is_palindromic(u: u64) -> bool {
    is_palindromic_base2(u) && is_palindromic_base10(u)
}

#[test]
fn test() {
    assert!(is_palindromic(585));
    assert!(is_palindromic(1));
    assert!(is_palindromic(3));
    assert!(is_palindromic(5));
    assert!(!is_palindromic(2));
    assert!(!is_palindromic(4));
}
