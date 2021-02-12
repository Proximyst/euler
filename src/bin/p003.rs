use num::integer::Roots;

fn main() {
    println!("{}", prime_factor(600851475143));
}

fn prime_factor(u: u64) -> u64 {
    (3u64..=u.sqrt())
        .rev()
        .step_by(1)
        .filter(is_prime)
        .find(|&i| u % i == 0)
        .expect("no prime factors")
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
fn factor_13195() {
    assert_eq!(29, prime_factor(13195));
}
