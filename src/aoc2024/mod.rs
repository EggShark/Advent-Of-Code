pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

use std::fs;
use crate::Solve;

pub fn run(days: &[u8]) {
    let mut runtime = 0.0;

    for day in days {
        let (p1, p2) = get_funcs(*day);
    
        runtime += solve_day(*day, p1, p2);
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn solve_day(day: u8, part1: fn(&str) -> Solve, part2: fn(&str) -> Solve) -> f64 {
    let text: String = fs::read_to_string(format!("src/aoc2024/inputs/day{day}")).unwrap();
    let p1 = part1(&text);
    let p2 = part2(&text);
    println!("\n=== Day {:02} ===", day);
    println!("  . Part 1:      {}", p1.solution);
    println!("  . Part 1 Time: {:.4} ms", p1.time_ms);
    println!("  . Part 2:      {}", p2.solution);
    println!("  . Part 2 Time: {:.4} ms", p2.time_ms);
    println!("Total time for Day {:02}: {:.4} ms", day, p1.time_ms + p2.time_ms);

    p1.time_ms + p2.time_ms
}

fn get_funcs(day: u8) -> (fn(&str) -> Solve, fn(&str) -> Solve) {
    match day {
        1 => (day01::part1, day01::part2),
        2 => (day02::part1, day02::part2),
        3 => (day03::part1, day03::part2),
        4 => (day04::part1, day04::part2),
        5 => (day05::part1, day05::part2),
        6 => (day06::part1, day06::part2),
        7 => (day07::part1, day07::part2),
        8 => (day08::part1, day08::part2),
        9 => (day09::part1, day09::part2),
       10 => (day10::part1, day10::part2),
       11 => (day11::part1, day11::part2),
       12 => (day12::part1, day12::part2),
       13 => (day13::part1, day13::part2),
       14 => (day14::part1, day14::part2),
       15 => (day15::part1, day15::part2),
       16 => (day16::part1, day16::part2),
       17 => (day17::part1, day17::part2),
       18 => (day18::part1, day18::part2),
       19 => (day19::part1, day19::part2),
       20 => (day20::part1, day20::part2),
       21 => (day21::part1, day21::part2),
       22 => (day22::part1, day22::part2),
       23 => (day23::part1, day23::part2),
       24 => (day24::part1, day24::part2),
       25 => (day25::part1, day25::part2),
       _ => unreachable!(),
    }
}