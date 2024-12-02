use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2024/inputs/day1").unwrap();
    let time = Instant::now();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in text.lines() {
        let (left_s, right_s) = line.split_once("   ").unwrap();
        left.push(left_s.parse().unwrap());
        right.push(right_s.parse().unwrap());
    }

    let part_2_sum: u64 = left
        .iter()
        .map(|num| (num, right.iter().filter(|right_num| * right_num == num).count()))
        .map(|(num, count)| count as u64 * *num as u64)
        .sum();

    left.sort();
    right.sort();

    let part_1 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(left_n, right_n)| i32::abs(left_n - right_n) as u64)
        .sum();

    let sol1: u64 = part_1;
    let sol2: u64 = part_2_sum;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}