mod aoc2022;
mod aoc2023;
mod aoc2024;
mod soultions;

use std::env;
use soultions::{Solution, SolutionType, Solve};

#[cfg(target_os="windows")]
const DOUBLE_NEW_LINE: &str = "\r\n\r\n";

#[cfg(not(target_os="windows"))]
const DOUBLE_NEW_LINE: &str = "\n\n";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide the day(s) to run as a command-line argument.");
    }

    let year = args[1].parse::<u16>().unwrap();
    assert!(year >= 2022 && year <= 2024);

    let days: Vec<u8> = args[2..].iter()
        .map(|x| x.parse().unwrap_or_else(|v| panic!("Not a valid day: {}", v)))
        .collect();

    match year {
        2022 => aoc2022::run(&days),
        2023 => aoc2023::run(&days),
        2024 => aoc2024::run(&days),
        _ => unimplemented!(),
    }
}