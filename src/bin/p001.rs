fn main() {
    println!("{}", multiple_sum(1000));
}

fn is_valid_multiple(&u: &u64) -> bool {
    u % 3 == 0 || u % 5 == 0
}

fn multiple_sum(limit: u64) -> u64 {
    (0..limit).filter(is_valid_multiple).sum()
}

#[test]
fn below_10() {
    assert_eq!(multiple_sum(10), 3 + 5 + 6 + 9);
}
