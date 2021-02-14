fn main() {
    let mut count = 1; // 1 for 2 pence
    for a in 0..=200 {
        for b in 0..=(200 / 2) {
            for c in 0..=(200 / 5) {
                for d in 0..=(200 / 10) {
                    for e in 0..=(200 / 20) {
                        for f in 0..=(200 / 50) {
                            for g in 0..=(200 / 100) {
                                if a + b * 2 + c * 5 + d * 10 + e * 20 + f * 50 + g * 100 == 200 {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
