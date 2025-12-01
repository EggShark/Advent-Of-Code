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

use std::fs;
use crate::Solve;

pub fn bench(days: &[u8], test: bool) {
    for day in days {
        let mut time_p1 = 0.0;
        let mut time_p2 = 0.0;
        let text: String = fs::read_to_string(format!("src/aoc2025/inputs/{}/day{day}", if test {"test"} else {"real"})).unwrap();
        
        //todo multithread
        for _ in 0..10 {
            let (p1, p2) = get_funcs(*day);
            time_p1 += p1(&text).time_ms;
            time_p2 += p2(&text).time_ms;
        }

        let p1_average = time_p1 / 10.0;
        let p2_average = time_p2 / 10.0;

        println!("\n=== day {:0} ===", day);
        println!(" - Average Part 1 time: {:.4} ms", p1_average);
        println!(" - Average Part 2 time: {:.4} ms", p2_average);
    }
}

pub fn run(days: &[u8], test: bool) {
    let mut runtime = 0.0;

    for day in days {
        let (p1, p2) = get_funcs(*day);
    
        runtime += solve_day(*day, test, p1, p2);
    }

    println!("\nTotal runtime: {:.4} ms", runtime);
}

fn solve_day(day: u8, test: bool, part1: fn(&str) -> Solve, part2: fn(&str) -> Solve) -> f64 {
    let text: String = fs::read_to_string(format!("src/aoc2025/inputs/{}/day{day}", if test {"test"} else {"real"})).unwrap();
    let p1 = part1(&text);
    let p2 = part2(&text);
    println!("\n=== Day {:02} ===", day);
    println!("  . Part 1:      {}", p1.solution);
    println!("  . Part 2:      {}", p2.solution);
    println!("  . Part 1 Time: {:.4} ms", p1.time_ms);
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
       _ => unreachable!(),
    }
}
