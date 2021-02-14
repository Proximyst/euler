use itertools::Itertools as _;
use std::collections::BTreeSet;

fn main() {
    let permutations = (0u64..=9)
        .permutations(10)
        .map(digits_to_value)
        .collect::<BTreeSet<_>>();
    println!("{}", permutations.into_iter().nth(1_000_000 - 1).unwrap());
}

fn digits_to_value(v: Vec<u64>) -> u64 {
    v.into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, digit)| acc + 10u64.pow(idx as u32) * digit)
}
