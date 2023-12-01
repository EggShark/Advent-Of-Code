use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day1").unwrap();
    let time = Instant::now();

    let lines: u64 = text.lines().map(|line| {
        let x: Vec<u64> = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        10 * x.first().unwrap() + x.last().unwrap()
    }).sum();

    let p2: u64 = text.lines().map(|line| {
        let mut nums: Vec<u64> = Vec::new();

        for idx in 0..line.len() {
            let num = &line[idx..idx+1].parse::<u64>();
            match num {
                Err(_) => {},
                Ok (num) => {
                    nums.push(*num);
                    continue;
                }
            }

            for (n_idx, name) in NUMBERS.iter().enumerate() {
                if idx + name.len() - 1 >= line.len() || &&line[idx..(idx+name.len())] != name {
                    continue;
                }

                nums.push(n_idx as u64 + 1);
                break;
            }
        }
        10 * nums.first().unwrap() + nums.last().unwrap()
    }).sum();

    let sol1: u64 = lines;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}