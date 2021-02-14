use rayon::prelude::*;

fn main() {
    println!(
        "{}",
        (1u64..10_000)
            .into_par_iter()
            .filter(is_amicable)
            .sum::<u64>()
    );
}

fn sum_divisors(u: u64) -> u64 {
    (1..u / 2 + 1).filter(|n| u % n == 0).sum()
}

fn is_amicable(&n: &u64) -> bool {
    let d = sum_divisors(n);
    let k = sum_divisors(d);

    d != n && k == n
}

#[test]
fn test() {
    assert_eq!(284, sum_divisors(220));
    assert_eq!(220, sum_divisors(284));
    assert!(is_amicable(&220));
}
