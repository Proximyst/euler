use chrono::prelude::*;

fn main() {
    println!(
        "{}",
        (1901..=2000)
            .map(|y| (1..=12)
                .filter(|&m| Utc.ymd(y, m, 1).weekday() == Weekday::Sun)
                .count())
            .sum::<usize>()
    );
}
