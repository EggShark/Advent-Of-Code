use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day11").unwrap();
    let time = Instant::now();

    let lines: Vec<Vec<char>> = text
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut galaxy_pos = Vec::new();

    let mut y_diffs = Vec::new();
    let mut x_diffs = Vec::new();

    let mut idx = 0;
    let mut r_start = 0;
    while idx < lines.len() {
        if lines[idx].iter().all(|c| *c == '.') {
            y_diffs.push(r_start..idx+1);
            r_start = idx+1;
        }
        idx += 1;
    }

    let last = y_diffs.last().unwrap().end;
    y_diffs.push(last..lines.len());

    let mut x = 0;
    let mut r_start = 0;
    while x < lines[0].len() {
        if (0..lines.len()).all(|y| lines[y][x] == '.') {
            x_diffs.push(r_start..x+1);
            r_start = x+1;
        }
        x += 1;
    }

    let last = x_diffs.last().unwrap().end;
    x_diffs.push(last..lines[0].len());

    for idx in 0..lines.len() {
        lines[idx]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .for_each(|(x, _)| {
                let x_to_add = x_diffs
                    .iter()
                    .enumerate()
                    .find(|(_, range)| range.contains(&x))
                    .map(|(idx, _)| idx)
                    .unwrap();
                

                let y_to_add = y_diffs
                    .iter()
                    .enumerate()
                    .find(|(_, range)| range.contains(&idx))
                    .map(|(idx, _)| idx)
                    .unwrap();

                galaxy_pos.push(((x + x_to_add) as i32, (idx + y_to_add) as i32));
            });
    }

    let mut p1_sum = 0;
    for num in 0..galaxy_pos.len() {
        let p1 = galaxy_pos[num];
        for idx in num+1..galaxy_pos.len() {
            let p2 = galaxy_pos[idx];
            let distance = i32::abs(p1.0 - p2.0) + i32::abs(p1.1 - p2.1);
            p1_sum += distance as i32;
        }
    }

    // resets galaxy
    let mut galaxy_pos = Vec::new();
    for idx in 0..lines.len() {
        lines[idx]
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '#')
            .for_each(|(x, _)| {
                let x_to_add = x_diffs
                    .iter()
                    .enumerate()
                    .find(|(_, range)| range.contains(&x))
                    .map(|(idx, _)| idx)
                    .unwrap() * 999999;

                let y_to_add = y_diffs
                    .iter()
                    .enumerate()
                    .find(|(_, range)| range.contains(&idx))
                    .map(|(idx, _)| idx)
                    .unwrap() * 999999;

                galaxy_pos.push(((x + x_to_add) as i32, (idx + y_to_add) as i32));
            });
    }

    let mut p2_sum = 0;
    for num in 0..galaxy_pos.len() {
        let p1 = galaxy_pos[num];
        for idx in num+1..galaxy_pos.len() {
            let p2 = galaxy_pos[idx];
            let distance = i32::abs(p1.0 - p2.0) + i32::abs(p1.1 - p2.1);
            p2_sum += distance as i64;
        }
    }

    let sol1: i32 = p1_sum;
    let sol2: i64 = p2_sum;

    let solution = (SolutionType::I32(sol1), SolutionType::I64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}