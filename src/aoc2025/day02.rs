use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    for range in data_in.split(',') {
        let (l, r) = range.split_once('-').unwrap();
        let (start, end) = (
            l.trim().parse::<i64>().unwrap(),
            r.trim().parse::<i64>().unwrap(),
        );
        for i in start..=end {
            let num_digits = i.ilog10() + 1;
            if num_digits % 2 != 0 {
                continue;
            }

            let mut left = 0;
            let mut d = i;
            let mut right = 0;

            for _ in 0..num_digits / 2 {
                left *= 10;
                left += d % 10;
                d /= 10;
            }
            for _ in 0..num_digits / 2 {
                right *= 10;
                right += d % 10;
                d /= 10;
            }

            if left == right {
                solve += i;
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    for x in data_in.split(",") {
        let (l, r) = x.split_once("-").unwrap();
        let (start, end) = (
            l.trim().parse::<u64>().unwrap(),
            r.trim().parse::<u64>().unwrap(),
        );

        for i in start..=end {
            let si = i.to_string();
            if (1..=si.len() / 2).any(|k| {
                if si.len() % k != 0 {
                    false
                } else {
                    (k..si.len()).step_by(k).all(|x| si[x..x + k] == si[0..k])
                }
            }) {
                solve += i;
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

