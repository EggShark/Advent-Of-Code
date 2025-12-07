use crate::{DOUBLE_NEW_LINE, Solve};

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let (ranges, numbers) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();
    let ranges = ranges
        .lines()
        .map(|l| l.split_once('-').map(|(l ,r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap())
        .collect::<Vec<(u64, u64)>>();

    for n in numbers.lines().map(|l| l.parse::<u64>().unwrap()) {
        for &(start, end) in ranges.iter() {
            if n >= start && n <= end {
                solve += 1;
                break;
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

    
    let (ranges, _) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();
    let mut ranges = ranges
        .lines()
        .map(|l| l.split_once('-').map(|(l ,r)| (l.parse().unwrap(), r.parse().unwrap())).unwrap())
        .collect::<Vec<(u64, u64)>>();

    ranges.sort_unstable_by_key(|k| k.0);

    let mut i = 0;
    while i < ranges.len() {
        if i + 1 < ranges.len() && ranges[i].1 >= ranges[i+1].0 {
            if ranges[i].1 < ranges[i+1].1 {
                ranges[i].1 = ranges[i + 1].1;
            }
            ranges.remove(i+1);
        }
        i += 1;
    }

    for (start, end) in ranges {
        solve += end - start + 1;
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
