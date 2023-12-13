use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day12").unwrap();
    let time = Instant::now();

//     let text = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";

    let lines: u64 = text
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| {
            count(
                &left.chars().collect::<Vec<char>>(),
                &right.split(",").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            )
        }).sum();

    // println!("{:?}", lines);

    let sol1: u64 = lines;
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn count(cfg: &[char], nums: &[usize]) -> u64 {
    if cfg.is_empty() {
        if nums.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if nums.is_empty() {
        if cfg.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    }

    let mut result = 0;

    if ".?".contains(cfg[0]) {
        result += count(&cfg[1..], nums)
    }

    // start of a block
    if "#?".contains(cfg[0]) {
        if nums[0] <= cfg.len() && !cfg[..nums[0]].contains(&'.') && (nums[0] == cfg.len() || cfg[nums[0]] != '#') {
            // starts a new block at cfg[0] that is the lenging of nums[0] + bounds check
            if !(nums[0]+1 > cfg.len() - 1) {
                result += count(&cfg[nums[0]+1..], &nums[1..])
            } else {
                result += count(&[], &nums[1..])
            }
        }
    }

    result
}