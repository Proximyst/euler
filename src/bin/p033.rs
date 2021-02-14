use num::{BigInt, BigRational, One as _};

fn main() {
    let mut product = BigRational::new(BigInt::one(), BigInt::one());
    for numerator in 10..100 {
        if (numerator / 10) * 10 == numerator {
            continue;
        }

        for denominator in numerator + 1..100 {
            if (denominator / 10) * 10 == denominator {
                continue;
            }

            let digits_num = digits(numerator);
            let digits_den = digits(denominator);

            let num = digits_num
                .iter()
                .filter(|i| !digits_den.contains(i))
                .map(|i| i.to_string())
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(numerator as i32);
            if num == numerator as i32 {
                continue;
            }

            let den = digits_den
                .iter()
                .filter(|i| !digits_num.contains(i))
                .map(|i| i.to_string())
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(0);
            if den == 0 {
                continue;
            }

            if BigRational::new(BigInt::from(num), BigInt::from(den))
                == BigRational::new(BigInt::from(numerator), BigInt::from(denominator))
            {
                product *= BigRational::new(BigInt::from(num), BigInt::from(den));
            }
        }
    }

    println!("{}", product.denom());
}

fn digits(mut n: u32) -> Vec<u32> {
    let mut digits = vec![];

    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}
