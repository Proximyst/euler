fn main() {
    let mut max = 0;
    let mut product = 0;

    for a in -999..=999 {
        for b in -1000..=1000 {
            let mut count = 0;
            for n in 0.. {
                let value = eval(a, b, n);
                if !is_prime(value) {
                    break;
                }
                count += 1;
            }

            if count > max {
                max = count;
                product = a * b;
            }
        }
    }

    println!("{}", product);
}

const fn eval(a: i64, b: i64, n: i64) -> i64 {
    n * n + a * n + b
}

const fn is_prime(u: i64) -> bool {
    if u <= 3 {
        return u > 1;
    }

    if u % 2 == 0 || u % 3 == 0 {
        return false;
    }

    let mut i = 5i64;
    while i.pow(2) <= u {
        if u % i == 0 || u % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}
