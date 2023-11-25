use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("").unwrap();
    let time = Instant::now();

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}