use crate::{Solution, SolutionType, DOUBLE_NEW_LINE};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Split;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day19").unwrap();
    let time = Instant::now();

    let (instruct, items) = text.split_once(DOUBLE_NEW_LINE).unwrap();

    let instructions = instruct
        .lines()
        .map(|line| WorkFlow::new(line))
        .collect::<HashMap<String, WorkFlow>>();

    let items = items
        .lines()
        .map(|line| {
            let line = &line[1..line.len()-1];
            let s = line.split(',');
            Item::new(s)
        })
        .collect::<Vec<Item>>();

    let mut sum = 0;
    for item in items.iter() {
        let mut current_check = instructions.get("in").unwrap();
        loop {
            match current_check.check(&item) {
                Result::Accepted => {
                    sum += (item.aero + item.cool + item.shiny + item.musical) as u64;
                    break;
                },
                Result::Rejected => break,
                Result::None => unreachable!(),
                Result::WorkFlow(name) => current_check = instructions.get(&name).unwrap(),
            }
        }
    }

    // cool musical aero shiny
    let ranges = [Range{start: 1, end: 4000}; 4];

    let p2 = instructions["in"].count_ranges(ranges, &instructions);

    let sol1: u64 = sum;
    let sol2: u64 = p2 as u64;

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

#[derive(Debug)]
struct WorkFlow {
    rules: Vec<Rule>
}

impl WorkFlow {
    fn new(line: &str) -> (String, Self) {
        let (name, right) = line.split_once('{').unwrap();
        let mut workflow = WorkFlow { rules: Vec::new() };
        for rule in right[..right.len() - 1].split(',') {
            workflow.rules.push(Rule::new(rule));
        }
        (name.into(), workflow)
    }

    fn check(&self, item: &Item) -> Result {
        // finds first true output
        self.rules
            .iter()
            .map(|rule| rule.eval(item))
            .find(|result| !matches!(result, Result::None))
            .unwrap()
    }

    fn count_ranges(&self, mut items: [Range; 4], instructions: &HashMap<String, WorkFlow>) -> i64 {
        let mut count = 0;
        for rule in self.rules.iter() {
            let (c, new_ranges) = rule.eval_range(items, instructions);
            count += c;
            items = new_ranges;
        }

        count
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn value(&self) -> i64 {
        (self.end-self.start + 1).max(0)
    }

    fn product(ranges: [Range; 4]) -> i64 {
        ranges  
            .iter()
            .map(|r| r.value())
            .product()
    }
}

#[derive(Debug)]
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
                Self::LessThan(stat, value, name.into())
            } else {
                Self::GreaterThan(stat, value, name.into())
            }
        } else {
            Self::NoOp(s.into())
        }
    }

    fn eval(&self, item: &Item) -> Result {
        match self {
            Self::NoOp(s) => {
                match s.as_str() {
                    "R" => Result::Rejected,
                    "A" => Result::Accepted,
                    name => Result::WorkFlow(name.to_string()),
                }
            },
            Self::LessThan(kind, value, where_to) => {
                let s = match kind {
                    Stats::Aero => item.aero < *value,
                    Stats::Cool => item.cool < *value,
                    Stats::Musical => item.musical < *value,
                    Stats::Shiny => item.shiny < *value,
                };
                if s {
                    match where_to.as_str() {
                        "A" => Result::Accepted,
                        "R" => Result::Rejected,
                        _ => Result::WorkFlow(where_to.to_string())
                    }
                } else {
                    Result::None
                }
            }
            Self::GreaterThan(kind, value, where_to) => {
                let s = match kind {
                    Stats::Aero => item.aero > *value,
                    Stats::Cool => item.cool > *value,
                    Stats::Musical => item.musical > *value,
                    Stats::Shiny => item.shiny > *value,
                };
                if s {
                    match where_to.as_str() {
                        "A" => Result::Accepted,
                        "R" => Result::Rejected,
                        _ => Result::WorkFlow(where_to.to_string())
                    }
                } else {
                    Result::None
                }
            }
        }
    }

    fn eval_range(&self, mut ranges: [Range; 4], instructions: &HashMap<String, WorkFlow>) -> (i64, [Range; 4]) {
        match self {
            Rule::LessThan(stat, value, where_to) => {
                let range: Range = ranges[usize::from(stat)];
                // spliting the range if the value is in the middle (smaller than the end of the part range)
                // we take that as our new max
                let (matched_start, matched_end) = (range.start, range.end.min(*value as i64 - 1));
                let mut matched_ranges = ranges;

                // splits the ranges
                matched_ranges[usize::from(stat)] = Range{start: matched_start, end: matched_end};
                ranges[usize::from(stat)] = Range{start: range.start.max(*value as i64), end: range.end};
                if where_to == "A" {
                    // all the parts < N made it woo hoo, so all the other ranges still need to be checked
                    (Range::product(matched_ranges), ranges)
                } else if where_to == "R" {
                    // all the parts < N are invalid, but we still can sheck all the parts > N :)
                    (0, ranges)
                } else {
                    // all the ranges < N move on to whats next the rest we'll buble up
                    (instructions[where_to].count_ranges(matched_ranges, instructions), ranges)
                }
            },
            Rule::GreaterThan(stat, value, where_to) => {
                let range: Range = ranges[usize::from(stat)];
                // spliting the range if the value is in the middle (smaller than the end of the part range)
                // we take that as our new max
                let (matched_start, matched_end) = (range.start.max(*value as i64 + 1), range.end);
                let mut matched_ranges = ranges;

                // splits the ranges
                matched_ranges[usize::from(stat)] = Range{start: matched_start, end: matched_end};
                ranges[usize::from(stat)] = Range{start: range.start, end: range.end.min(*value as i64)};
                if where_to == "A" {
                    // all the parts > N made it woo hoo, so all the other ranges still need to be checked
                    (Range::product(matched_ranges), ranges)
                } else if where_to == "R" {
                    // all the parts > N are invalid, but we still can sheck all the parts > N :)
                    (0, ranges)
                } else {
                    // all the ranges > N move on to whats next the rest we'll buble up
                    (instructions[where_to].count_ranges(matched_ranges, instructions), ranges)
                }
            }
            Rule::NoOp(where_to) => {
                // the whole range either makes it or it doesnt
                if where_to == "A" {
                    (Range::product(ranges), Default::default())
                } else if where_to == "R" {
                    (0, Default::default())
                } else {
                    (instructions[where_to].count_ranges(ranges, instructions), Default::default())
                }
            }
        }

        // todo!()
    }
}

enum Result {
    WorkFlow(String),
    Accepted,
    Rejected,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stats {
    Cool,
    Musical,
    Aero,
    Shiny,
}

impl From<&Stats> for usize {
    fn from(value: &Stats) -> Self {
        match value {
            Stats::Cool => 0,
            Stats::Musical => 1,
            Stats::Aero => 2,
            Stats::Shiny => 3,
        }
    }
}