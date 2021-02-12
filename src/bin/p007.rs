fn main() {
    println!("{}", nth_prime(10_001));
}

fn nth_prime(n: usize) -> u64 {
    (1..).filter(is_prime).nth(n).expect("no more primes?")
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
fn primes() {
    assert_eq!(2, nth_prime(0));
    assert_eq!(3, nth_prime(1));
    assert_eq!(5, nth_prime(2));
    assert_eq!(7, nth_prime(3));
    assert_eq!(11, nth_prime(4));
    assert_eq!(13, nth_prime(5));
}
