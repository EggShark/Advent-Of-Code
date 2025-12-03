use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    for line in data_in.lines() {
        let mut max = 0;
        let digits = line.bytes().map(|c| (c-48) as u32).collect::<Vec<_>>();

        for i in 0..digits.len() - 1 {
            for j in i+1..digits.len() {
                max = std::cmp::max(max, digits[i]*10 + digits[j]);
            }
        }

        solve += max;
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn position_max_copy<T: Ord + Copy>(slice: &[T]) -> Option<(usize, &T)> {
    slice.iter().enumerate().rev().max_by_key(|(_, &value)| value)
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    for line in data_in.lines() {
        let mut max = 0;
        let digits = line.bytes().map(|c| (c-48) as u64).collect::<Vec<_>>();
        let mut slice: &[u64] = &digits;

        for i in 0..12 {
            let len = slice.len();
            max *= 10;
            let (max_idx, val) = position_max_copy(&slice[..len-(11-i)]).unwrap();
            max += val;
            slice = &slice[max_idx+1..];
        }

        solve += max;
    }



    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
