use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day10.txt").unwrap();
    let time = Instant::now();
    let mut cur_cycle = 0;
    let mut x = 1; // this sucks why is it not 0
    let mut sum = 0;
    let mut drawing = String::from("\n");

    let instructions = text.lines()
        .map(|str| Instructions::from_str(str))
        .collect::<Vec<Instructions>>();

    for instruction in instructions.iter() {
        match instruction {
            &Instructions::Noop => {
                update_register(x, cur_cycle, &mut sum, &mut drawing);
            },
            &Instructions::Addx(num) => {
                update_register(x, cur_cycle, &mut sum, &mut drawing);
                cur_cycle += 1;
                update_register(x, cur_cycle, &mut sum, &mut drawing);
                x += num as i32;
            }
        }
        cur_cycle += 1;
    }



    let sol1: i32 = sum;
    let sol2: String = drawing;

    let solution = (SolutionType::I32(sol1), SolutionType::Str(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn update_register(register: i32, cur_cycle: i32, sum: &mut i32, line: &mut String) {
    if (cur_cycle + 20) % 40 == 0 {
        *sum += register * cur_cycle;
    }

    let cycle_mod = cur_cycle % 40;
    let reg_cycle_diff = (cycle_mod - register).abs();
    if reg_cycle_diff < 2 {
        line.push('█');
    } else {
        line.push('.');
    }
    if cycle_mod == 39 {
        line.push('\n');
    }
}

#[derive(Debug)]
enum Instructions {
    Noop,
    Addx(i8),
}

impl Instructions {
    pub fn from_str(s: &str) -> Self {
        let x = s.split_once(' ');
        match x {
            None => Self::Noop,
            Some((_, right)) => Self::Addx(right.parse::<i8>().unwrap()),
        }
    }
}