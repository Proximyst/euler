use num::BigUint;
use std::collections::HashSet;

fn main() {
    println!("{}", terms(2, 100, 2, 100).len());
}

fn terms(a_from: u32, a_to: u32, b_from: u32, b_to: u32) -> HashSet<BigUint> {
    let mut set = HashSet::new();

    for a in a_from..=a_to {
        let a = BigUint::from(a);
        for b in b_from..=b_to {
            set.insert(a.pow(b));
        }
    }

    set
}

#[test]
fn test() {
    assert_eq!(15, terms(2, 5, 2, 5).len());
}
