mod aoc2022;
mod aoc2023;
mod aoc2024;
mod soultions;
mod shared;

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

    let mut test = false;
    let mut bench = false;
    let mut days: Vec<u8> = Vec::new();
    let mut year: u16 = 2024;

    let mut iter = args.iter().peekable();
    iter.next(); // ignrore path arguments
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-t" | "--test" => test = true,
            "-b" | "--bench" => bench = true,
            "-d" | "--days" => {
                while let Some(Ok(_)) = iter.peek().map(|s| s.parse::<u8>()) {
                    days.push(iter.next().unwrap().parse().unwrap());
                }
            }
            "-y" | "--year" => {
                if let Some(Ok(i)) = iter.peek().map(|s| s.parse::<u16>()) {
                    iter.next();
                    year = i;
                } else {
                    println!("Uh Oh year isnt given defualting to {}", year);
                }
            }
            a => println!("Unknown argument of {:?}", a)
        }
    }

    assert!(year >= 2022 && year <= 2024);

    match (year, bench) {
        (2022, _) => aoc2022::run(&days),
        (2023, _) => aoc2023::run(&days),
        (2024, false) => aoc2024::run(&days, test),
        (2024, true) => aoc2024::bench(&days, test),
        _ => unimplemented!(),
    }
}