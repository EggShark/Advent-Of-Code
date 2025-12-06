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
    let mut solve = 0;
    
    const COL_SIZE: usize = 3;

    let ops = data_in.lines().last().unwrap().bytes().filter(|b| *b != b' ').collect::<Vec<_>>();
    let digits = data_in
        .lines()
        .take_while(|l| !(l.starts_with("+")||l.starts_with("*")))
        .map(|l| l.split(' ').map(|c| c.bytes().collect::<Vec<_>>()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", digits);
    assert_eq!(digits[0].len()/COL_SIZE, ops.len());

    for x in (0..digits[0].len()).rev() {
        let op_col = x / COL_SIZE;
        let c_score = if ops[op_col] == b'+' {
            0
        } else {
            1
        };
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
