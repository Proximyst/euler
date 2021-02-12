use rayon::prelude::*;

fn main() {
    println!(
        "{}",
        (1..(10u64.pow(6)))
            .into_par_iter()
            .max_by_key(|&u| collatz(u))
            .unwrap()
    );
}

fn collatz(mut u: u64) -> usize {
    let mut c = 1;
    while u != 1 {
        c += 1;
        if u & 1 == 0 {
            u /= 2;
        } else {
            u = u * 3 + 1;
        }
    }
    c
}

#[test]
fn test_simple() {
    assert_eq!(collatz(13), 10);
    assert_eq!(collatz(2), 2);
    assert_eq!(collatz(3), 8);
}
