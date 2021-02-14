const THOUSAND: &'static str = "thousand";
const HUNDRED: &'static str = "hundred";
const VALUES: [&'static str; 28] = [
    "placeholder to make indexing easy",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

fn main() {
    println!("{}", count_to(1000));
}

fn name(n: usize) -> String {
    if n == 0 {
        return String::new();
    }

    if n <= 20 {
        return VALUES[n].into();
    }

    if n < 100 && (n / 10) * 10 == n {
        // This is a whole number like 30, 40, 50, ...
        return VALUES[(n / 10) + 18].into();
    }

    if n < 100 {
        return format!("{}{}", VALUES[(n / 10) + 18], VALUES[n - ((n / 10) * 10)]);
    }

    if n >= 1000 {
        return format!("{}{}{}", name(n / 1000), THOUSAND, name(n % 1000));
    }

    if n >= 100 {
        let mut extra_part = name(n % 100);
        if !extra_part.is_empty() {
            extra_part.insert_str(0, "and");
        }
        return format!("{}{}{}", name(n / 100), HUNDRED, extra_part);
    }

    unreachable!("unchecked region of values?")
}

fn count_to(n: usize) -> usize {
    let mut s = String::new();

    for i in 1..=n {
        s.push_str(&name(i));
    }

    return s.chars().count();
}

#[test]
fn test_static_names() {
    assert_eq!("", name(0));

    for i in 1..=20 {
        assert_eq!(VALUES[i], name(i));
    }

    for i in (30..=90).step_by(10) {
        assert_eq!(VALUES[18 + (i / 10)], name(i));
    }

    assert_eq!(format!("{}{}", VALUES[1], HUNDRED), name(100));
    assert_eq!(format!("{}{}", VALUES[1], THOUSAND), name(1000));
}

#[test]
fn test_built_names() {
    assert_eq!("three", name(3));
    assert_eq!("eighteen", name(18));
    assert_eq!("forty", name(40));
    assert_eq!("twentyone", name(21));
    assert_eq!("fivehundred", name(500));
    assert_eq!("fivehundredandthree", name(503));
    assert_eq!("onehundred", name(100));
    assert_eq!("onethousand", name(1000));
    assert_eq!(
        "seventeenthousandthreehundredandthirtyseven",
        name(17_000 + 300 + 37)
    );
}

#[test]
fn test_length() {
    assert_eq!(19, count_to(5));
}
