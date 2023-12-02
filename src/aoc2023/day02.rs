use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day2").unwrap();
    let time = Instant::now();

    let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let p1: usize = text
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let mut max_r_count = 0;
            let mut max_g_count = 0;
            let mut max_b_count = 0;
            let (_, right) = line.split_once(":").unwrap();
            right.split(";").for_each(|section| {
                section.split(",").for_each(|sectioner| {
                    let stuff: Vec<&str> = sectioner.split_whitespace().collect();
                    for idx in (0..stuff.len() - 1).step_by(2) {
                        let int: u64 = stuff[idx].parse().unwrap();
                        let colour = stuff[idx + 1];
                        match colour {
                            "red" => max_r_count = max_r_count.max(int),
                            "green" => max_g_count = max_g_count.max(int),
                            "blue" => max_b_count = max_b_count.max(int),
                            _ => panic!(),
                        }
                    }
                });
            });
            (max_r_count, max_g_count, max_b_count, idx)
        })
        .filter(|(r_count, g_count, b_count, _)| r_count <= &12 && g_count <= &13 && b_count <= &14)
        .map(|(_, _, _, idx)| idx + 1)
        .sum();

    let p2: u64 = text
        .lines()
        .map(|line| {
            let mut max_r_count = 0;
            let mut max_g_count = 0;
            let mut max_b_count = 0;
            let (_, right) = line.split_once(":").unwrap();
            right.split(";").for_each(|section| {
                section.split(",").for_each(|sectioner| {
                    let stuff: Vec<&str> = sectioner.split_whitespace().collect();
                    for idx in (0..stuff.len() - 1).step_by(2) {
                        let int: u64 = stuff[idx].parse().unwrap();
                        let colour = stuff[idx + 1];
                        match colour {
                            "red" => max_r_count = max_r_count.max(int),
                            "green" => max_g_count = max_g_count.max(int),
                            "blue" => max_b_count = max_b_count.max(int),
                            _ => panic!(),
                        }
                    }
                });
            });
            (max_r_count, max_g_count, max_b_count)
        })
        .map(|(r, g, b)| r * g * b)
        .sum();


    let sol1: u64 = p1 as u64;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}