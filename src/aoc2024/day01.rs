use crate::Solve;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data_in.lines() {
        let (left_s, right_s) = line.split_once("   ").unwrap();
        left.push(left_s.parse().unwrap());
        right.push(right_s.parse().unwrap());
    }

    left.sort();
    right.sort();

    let part_1: u64 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(left_n, right_n)| i32::abs(left_n - right_n) as u64)
        .sum();

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(part_1),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data_in.lines() {
        let (left_s, right_s) = line.split_once("   ").unwrap();
        left.push(left_s.parse().unwrap());
        right.push(right_s.parse().unwrap());
    }

    let part_2_sum: u64 = left
        .iter()
        .map(|num| (num, right.iter().filter(|right_num| * right_num == num).count()))
        .map(|(num, count)| count as u64 * *num as u64)
        .sum();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(part_2_sum),
        time_ms,
    }
}