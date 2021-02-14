use big_rational_str::big_rational_to_string;
use num::{BigInt, BigRational, One as _};
use rayon::prelude::*;

fn main() {
    println!(
        "{}",
        (7..1000)
            .into_par_iter()
            .max_by_key(|&i| length_for_recip(i))
            .unwrap()
    );
}

fn length_of_recurring(s: &str) -> usize {
    let start = match s.find('(') {
        Some(i) => i,
        None => return 0,
    };
    let end = match s.find(')') {
        Some(i) => i,
        None => return 0,
    };

    (&s[start + 1..end]).len()
}

fn length_for_recip(i: i32) -> usize {
    length_of_recurring(&big_rational_to_string(BigRational::new(
        BigInt::one(),
        BigInt::from(i),
    )))
}
