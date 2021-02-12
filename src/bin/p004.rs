use rayon::prelude::*;

fn main() {
    println!("{}", largest_palindromic_number(100, 999));
}

fn is_palindrome(&u: &u64) -> bool {
    let mut n = u;
    let mut rev = 0;

    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }

    u == rev
}

fn largest_palindromic_number(from: u64, to: u64) -> u64 {
    (from..=to)
        .into_par_iter()
        .flat_map(|i| {
            (from..=to)
                .into_par_iter()
                .map(|u| u * i)
                .filter(is_palindrome)
                .max()
        })
        .max()
        .expect("no palindromes")
}

#[test]
fn test_largest_palindromes() {
    assert_eq!(9009, largest_palindromic_number(10, 99));
    assert_eq!(9, largest_palindromic_number(0, 9));
}

#[test]
fn test_simple_palindromes() {
    assert!(is_palindrome(&9009));
    assert!(is_palindrome(&80008));
    assert!(is_palindrome(&1111));
    assert!(is_palindrome(&1221));

    assert!(!is_palindrome(&1234));
    assert!(!is_palindrome(&1231));
}
