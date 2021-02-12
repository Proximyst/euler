fn main() {
    println!("{}", diff(100));
}

fn diff(u: u64) -> u64 {
    (1..=u).sum::<u64>().pow(2) - (1..=u).map(|n| n * n).sum::<u64>()
}

#[test]
fn test_diff() {
    assert_eq!(2640, diff(10));
}
