use crate::{Solve, DOUBLE_NEW_LINE};
use std::{collections::HashMap, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let (rules_text, order) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();
    for (left, right) in rules_text.lines().map(|l| l.split_once('|').unwrap()).map(|(left, right)| (left.parse::<u16>().unwrap(), right.parse::<u16>().unwrap())) {
        let h = rules.entry(left).or_default();
        h.push(right);
    }

    let solve: u32 = order.lines()
    .filter_map(|line| {
        let line = line
            .split(',')
            .map(|i| i.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        let mut valid = true;
        'o: for i in 1..line.len() {
            let rule = rules.get(&line[i]);
            if rule.is_none() {
                continue;
            }

            let rule = rule.unwrap();

            for x in 0..i {
                if rule.contains(&line[x]) {
                    //println!("found: {}, violating rule {}:{:?}", line[x], line[i], rule);
                    valid = false;
                    break 'o;
                }
            }
        }

        if valid {
            Some(line[line.len() / 2] as u32)
        } else {
            None
        }
    })
    .sum();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let (rules_text, order) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();
    for (left, right) in rules_text.lines().map(|l| l.split_once('|').unwrap()).map(|(left, right)| (left.parse::<u16>().unwrap(), right.parse::<u16>().unwrap())) {
        let h = rules.entry(left).or_default();
        h.push(right);
    }

    let solve: u32 = order.lines()
    .filter_map(|line| {
        let mut line = line
            .split(',')
            .map(|i| i.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        let mut valid = true;
        for i in 1..line.len() {
            let rule = rules.get(&line[i]);
            if rule.is_none() {
                continue;
            }

            let rule = rule.unwrap();

            for x in 0..i {
                if rule.contains(&line[x]) {
                    valid = false;
                    line.swap(x, i);
                }
            }
        }

        if valid {
            None
        } else {
            Some(line[line.len() / 2] as u32)
        }
    })
    .sum();

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}