use std::collections::HashSet;

fn main() {
    let mut nums = HashSet::new();

    for i in 1..=10u32.pow(4) {
        let len = count_digits(i) as u32;
        for j in 10u32.pow(4 - len)..=10u32.pow(6 - len) {
            let product = i * j;

            let concat = concatenate_number(i, j);
            let concat = concatenate_number(concat, product);

            if is_pandigital(concat) {
                nums.insert(product);
            }
        }
    }

    println!("{}", nums.into_iter().sum::<u32>());
}

fn count_digits(mut n: u32) -> usize {
    let mut count = 0;

    while n != 0 {
        count += 1;
        n /= 10;
    }

    count
}

fn is_pandigital(mut n: u32) -> bool {
    let mut array = [0; 9];

    while n != 0 {
        let digit = n % 10;
        if digit == 0 {
            return false;
        }

        array[digit as usize - 1] += 1;
        n /= 10;
    }

    array.iter().all(|&i| i == 1)
}

fn concatenate_number(a: u32, b: u32) -> u32 {
    b + a * 10u32.pow(count_digits(b) as u32)
}

#[test]
fn test() {
    assert!(is_pandigital(123456789));
    assert_eq!(concatenate_number(12, 34), 1234);
    assert_eq!(concatenate_number(1234, 5678), 12345678);
}
