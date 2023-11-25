use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day06.txt").unwrap();
    let time = Instant::now();
    let jump = 4;
    let mut start = 0;
    let mut end = jump;

    while end < text.len() {
        start += 1;
        end = start + jump;
        let block = &text[start..end];
        let mut buffer = String::new();
        let mut repition = 0;
        for char in block.chars() {
            if !buffer.contains(char) {
                buffer.push(char);
            } else {
                repition += 1;
                buffer.push(char);
            }
        }
        if repition == 0 {
            break;
        }
    }

    let mut start2 = 0;
    let jump2 = 14;
    let mut end2 = jump2;
    while end < text.len() {
        start2 += 1;
        end2 = start2 + jump2;
        let block = &text[start2..end2];
        let mut buffer = String::new();
        let mut repition = 0;
        for char in block.chars() {
            if !buffer.contains(char) {
                buffer.push(char);
            } else {
                repition += 1;
                buffer.push(char);
            }
        }
        if repition == 0 {
            break;
        }
    }

    let sol1: usize = start + jump;
    let sol2: usize = start2 + jump2;

    let solution = (SolutionType::Usize(sol1), SolutionType::Usize(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}