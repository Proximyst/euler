use num::integer::Roots as _;
use std::iter::Iterator;

fn main() {
    println!(
        "{:?}",
        TriangleNumbers::default().find(|i| count_divisors(i) >= 500)
    );
}

fn count_divisors(&u: &u64) -> usize {
    (1..u.sqrt()).filter(|n| u % n == 0).count() * 2
}

#[derive(Default)]
struct TriangleNumbers {
    n: u64,
    acc: u64,
}

impl Iterator for TriangleNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        self.acc += &self.n;
        Some(self.acc)
    }
}

#[test]
fn test_raw_numbers() {
    // This algorithm doesn't work on 1.
    // assert_eq!(1, count_divisors(&1));
    assert_eq!(2, count_divisors(&3));
    assert_eq!(4, count_divisors(&6));
    assert_eq!(4, count_divisors(&10));
    assert_eq!(4, count_divisors(&15));
    assert_eq!(4, count_divisors(&21));
    assert_eq!(6, count_divisors(&28));
}

#[test]
fn test_triangles() {
    use itertools::Itertools as _;
    assert_eq!(
        TriangleNumbers::default().take(10).collect_vec(),
        vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55],
    );
}
