use rayon::prelude::*;

fn main() {
    println!(
        "{:#?}",
        (2..10u64.pow(6))
            .into_par_iter()
            .filter(|&i| sum_digits(i, 5) == i)
            .sum::<u64>()
    );
}

fn sum_digits(mut n: u64, pow: u32) -> u64 {
    let mut sum = 0;

    while n != 0 {
        sum += (n % 10).pow(pow);
        n /= 10;
    }

    sum
}

#[test]
fn test() {
    assert_eq!(sum_digits(1634, 4), 1634);
    assert_eq!(sum_digits(8208, 4), 8208);
    assert_eq!(sum_digits(9474, 4), 9474);
}
