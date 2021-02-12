fn main() {
    println!("{}", smallest_divisible_number(1, 20));
}

fn smallest_divisible_number(from: u64, to: u64) -> u64 {
    (1..)
        .map(|i| i * from * to * (to - from).max(1))
        .find(|i| (from..=to).all(|k| i % k == 0))
        .expect("no divisible")
}

#[test]
fn test_smallest_divisible() {
    assert_eq!(2520, smallest_divisible_number(1, 10));
}
