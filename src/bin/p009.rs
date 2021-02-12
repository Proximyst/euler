fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if is_pythagorean_triplet(a, b, c) && a + b + c == 1000 {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}

fn is_pythagorean_triplet(a: u64, b: u64, c: u64) -> bool {
    if a >= b || b >= c {
        return false;
    }

    if a * a + b * b != c * c {
        return false;
    }

    true
}

#[test]
fn test_triplet() {
    assert!(is_pythagorean_triplet(3, 4, 5));
}
