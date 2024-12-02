use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2024/inputs/day2").unwrap();
    let time = Instant::now();

    let input = text.
        lines()
        .map(|line| line.split(' ').map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut count = 0;
    let mut count_p2 = 0;
    for line in input {
        let mut is_valid = true;
        let prev_decrease = line[0] > line[1];
        let mut bad_level = 0;
        for i in 0..line.len() - 1 {
            let diff = line[i] - line[i + 1];
            if line[i] < line[i + 1] && prev_decrease {
                bad_level += 1;
                is_valid = false;
                continue;
            }

            if line[i] > line[i + 1] && !prev_decrease {
                is_valid = false;
                bad_level += 1;
                continue;
            }

            if i32::abs(diff) > 3 || i32::abs(diff) < 1 {
                bad_level += 1;
                is_valid = false;
                continue;
            }
        }

        if is_valid {
            println!("valid line: {:?}", line);
            count += 1;
            count_p2 += 1;
        } else if bad_level <= 1 {
            count_p2 += 1;
        }
    }

    let sol1: u64 = count;
    let sol2: u64 = count_p2;;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}