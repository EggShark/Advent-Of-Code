use crate::Solve;

use std::{time::Instant, usize};

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let nums = data_in.lines()
        .take_while(|d| !(d.starts_with("+") || d.starts_with("*")))
        .map(|l| l.split(' ').filter(|c| !c.is_empty()).map(|c| c.parse::<u64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (idx, op) in data_in.lines().last().unwrap().split(" ").filter(|c| !c.is_empty()).enumerate() {
        let mut y_score = if op == "+" {
            0
        } else {
            1
        };
        for y in 0..nums.len() {
            if op == "+" {
                y_score += nums[y][idx];
            }
            else {
                y_score *= nums[y][idx];  
            }
        }
        solve += y_score;
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve: u64 = 0;

    let grid = data_in.lines()
        .take_while(|l| !(l.starts_with('+') || l.starts_with("*")))
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ops = data_in.lines().last().unwrap().split_ascii_whitespace().collect::<Vec<_>>();

    let mut i = 0;
    let mut c_score = if ops[0] == "*" {
        1
    } else {
        0
    };

    for x in 0..grid[0].len() {
        let mut num = 0;
        for y in 0..grid.len() {
            if grid[y][x] == b' ' {
                continue;
            }
            num *= 10;
            num += (grid[y][x] - b'0') as u64;
        }
        if num == 0 {
            if i + 1 == ops.len() {
                break;
            }
            i += 1;
            solve += c_score;
            c_score = if ops[i] == "*" {
                1
            } else {
                0
            };
            continue;
        }
        if ops[i] == "*" {
            c_score *= num;
        } else {
            c_score += num;
        }
    }

    solve += c_score;

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
