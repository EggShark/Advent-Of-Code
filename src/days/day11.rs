use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text1 = read_to_string("./texts/day11.txt").unwrap();
    let time = Instant::now();

    let mut monkeys = text1.split("\n\n").map(|monkey| Monkey::from_str(monkey)).collect::<Vec<Monkey>>();
    let mut m2 = text1.split("\n\n").map(|monkey| Monkey::from_str(monkey)).collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            monkeys[i].items_inspected += items.len() as i64;
            for mut item in items {
                item = match monkeys[i].op {
                    Operations::Add(int) => item + int,
                    Operations::Mul(int) => item * int,
                    Operations::Expontential => item * item,
                };
                item = item/3;
                if item % monkeys[i].test == 0{
                    let target = monkeys[i].pass;
                    monkeys[target].items.push(item);
                } else {
                    let target = monkeys[i].fail;
                    monkeys[target].items.push(item);
                }
            }
        }
    }

    let monkey_lcm: i64 = m2.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for i in 0..m2.len() {
            let items = std::mem::take(&mut m2[i].items);
            m2[i].items_inspected += items.len() as i64;
            for mut item in items {
                item = match m2[i].op {
                    Operations::Add(int) => (item + int) % monkey_lcm,
                    Operations::Mul(int) => (item * int) % monkey_lcm,
                    Operations::Expontential => (item * item) % monkey_lcm,
                };
                if item % m2[i].test == 0{
                    let target = m2[i].pass;
                    m2[target].items.push(item);
                } else {
                    let target = m2[i].fail;
                    m2[target].items.push(item);
                }
            }
        }
    }


    monkeys.sort_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    m2.sort_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    let sol1 = monkeys[0].items_inspected * monkeys[1].items_inspected;
    let sol2 = m2[0].items_inspected * m2[1].items_inspected;


    let sol1: i64 = sol1;
    let sol2: i64 = sol2;

    let solution = (SolutionType::I64(sol1), SolutionType::I64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug, Clone, Copy)]
enum Operations {
    Add(i64),
    Mul(i64),
    Expontential,
}

impl Operations {
    pub fn from_str(s: &str) -> Self {
        let (_, right) = s.split_once("= ").unwrap();
        let right = right.to_string();
        let digit = right[right.len() - 1..right.len()].parse::<i64>();
        match digit {
            Result::Err(_) => {
                Self::Expontential
            },
            Ok(int) => {
                if right.contains('+') {
                    Self::Add(int)
                } else {
                    Self::Mul(int)
                }
            },
        }
    }
}

#[derive(Debug)]
struct Monkey {
    test: i64,
    fail: usize,
    pass: usize,
    op: Operations,
    items: Vec<i64>,
    items_inspected: i64,
}

impl Monkey {
    pub fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let _skip = lines.next().unwrap(); // should skip over the name
        let starting_items = lines.next().unwrap();
        let items = {
            let (_, items) = starting_items.split_once(": ").unwrap();
            items.split(", ").map(|num| num.parse().unwrap()).collect::<Vec<i64>>()
        };
        let op = Operations::from_str(lines.next().unwrap());
        let test = lines.next().unwrap().split(" ").last().unwrap().parse::<i64>().unwrap();
        let pass = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let fail = lines.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();

        Self {
            test,
            fail,
            pass,
            op,
            items,
            items_inspected: 0,
        }
    }
}