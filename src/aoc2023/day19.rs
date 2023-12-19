use crate::{Solution, SolutionType, DOUBLE_NEW_LINE};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Split;
use std::time::Instant;
use std::ops::Range;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day19").unwrap();
    let time = Instant::now();

    let (instruct, items) = text.split_once(DOUBLE_NEW_LINE).unwrap();

    let instructions = instruct
        .lines()
        .map(|line| Check::new(line))
        .collect::<HashMap<String, Check>>();

    let items = items
        .lines()
        .map(|line| {
            let line = &line[1..line.len()-1];
            let s = line.split(',');
            Item::new(s)
        })
        .collect::<Vec<Item>>();

    let mut sum = 0;

    for item in items {
        let mut current_instruction = instructions.get("in").unwrap();
        'i: loop {
            'j: for cmp in current_instruction.comparisions.iter() {
                let out_come = match (cmp.stat, cmp.greater_than) {
                    (Stats::Cool, true) => item.cool > cmp.number,
                    (Stats::Cool, false) => item.cool < cmp.number,
                    (Stats::Aero, true) => item.aero > cmp.number,
                    (Stats::Aero, false) => item.aero < cmp.number,
                    (Stats::Musical, true) => item.musical > cmp.number,
                    (Stats::Musical, false) => item.musical < cmp.number,
                    (Stats::Shiny, true) => item.shiny > cmp.number,
                    (Stats::Shiny, false) => item.shiny < cmp.number,
                };

                let result = if out_come {
                    &cmp.sucsess_result
                } else {
                    &cmp.fail_result
                };

                match result {
                    Result::Accpected => {
                        sum += (item.cool + item.aero + item.musical + item.shiny) as u64;
                        break 'i;
                    },
                    Result::Rejected => {
                        break 'i;
                    },
                    Result::GoTo(s) => {
                        current_instruction = instructions.get(s).unwrap();
                        break 'j;
                    },
                    Result::Next => continue 'j,
                }
            }
        }
    }

    let sol1: u64 = sum;
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}
#[derive(Debug)]
struct Item {
    cool: u32,
    musical: u32,
    aero: u32,
    shiny: u32,
}

impl Item {
    fn new(mut things: Split<'_, char>) -> Self {
        let cool: u32 = things.next().unwrap()[2..].parse().unwrap();
        let musical: u32 = things.next().unwrap()[2..].parse().unwrap();
        let aero: u32 = things.next().unwrap()[2..].parse().unwrap();
        let shiny_str = things.next().unwrap();
        let shiny: u32 = shiny_str[2..shiny_str.len()].parse().unwrap();
        Self {
            cool,
            musical,
            aero,
            shiny,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Result {
    Accpected,
    Rejected,
    Next,
    GoTo(String),
}

#[derive(Debug)]
struct Check {
    comparisions: Vec<Compare>,
}

impl Check {
    fn new(line: &str) -> (String, Self) {
        let (name, right) = line.split_once('{').unwrap();
        let mut comparisions = Vec::new();
        Compare::new(&mut comparisions, &right[0..right.len()]);
        (
            name.to_string(),
            Self {
                comparisions,
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Compare {
    number: u32,
    stat: Stats,
    greater_than: bool,
    sucsess_result: Result,
    fail_result: Result,
}

impl Compare {
    fn new(vec: &mut Vec<Compare>, string: &str) {
        let (comp, right) = string.split_once(':').unwrap();
        let stat = match &comp[0..1] {
            "a" => Stats::Aero,
            "x" => Stats::Cool,
            "m" => Stats::Musical,
            "s" => Stats::Shiny,
            _ => unreachable!(),
        };
        let greater_than = &comp[1..2] == ">";
        let number: u32 = comp[2..].parse().unwrap();
        let (first_result, second_result) = right.split_once(',').unwrap();
        
        let sucsess_result = if &first_result[0..1] == "A" {
            Result::Accpected
        } else if &first_result[0..1] == "R" {
            Result::Rejected
        } else {
            Result::GoTo(first_result.to_string())
        };

        let fail_result = if &second_result[0..1] == "A" {
            Result::Accpected
        } else if &second_result[0..1] == "R" {
            Result::Rejected
        } else if &second_result[1..2] == ">" || &second_result[1..2] == "<" {
            Result::Next
        } else {
            if &second_result[second_result.len()-1..second_result.len()] == "}" {
                Result::GoTo(second_result[0..second_result.len()-1].to_string())
            } else {
                Result::GoTo(second_result[0..second_result.len()].to_string())
            }
        };

        if fail_result == Result::Next {
            vec.push(Self {
                number,
                stat,
                greater_than,
                sucsess_result,
                fail_result,
            });
            Compare::new(vec, second_result);
        } else {
            vec.push(Self {
                number,
                stat,
                greater_than,
                sucsess_result,
                fail_result,
            });
        }
    }
}

enum Rule {
    LessThan(Stats, u32, String),
    GreaterThan(Stats, u32, String),
    NoOp(String),
}

impl Rule {
    fn new(s: &str) -> Self {
        if let Some((cond, name)) = s.split_once(':') {
            let bytes = cond.as_bytes();
            let stat = match bytes[0] {
                b'x' => Stats::Cool,
                b'm' => Stats::Musical,
                b'a' => Stats::Aero,
                b's' => Stats::Shiny,
                _ => unreachable!(),
            };

            let value: u32 = cond[2..].parse().unwrap();
            if bytes[1] == b'<' {

            }

            todo!();

        } else {
            Self::NoOp(s.into())
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stats {
    Cool,
    Musical,
    Aero,
    Shiny,
}