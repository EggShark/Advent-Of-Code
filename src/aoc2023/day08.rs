use crate::{Solution, SolutionType};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day8").unwrap();
    let time = Instant::now();

    let mut lines = text.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let _ = lines.next().unwrap();

    let mut nodes = HashMap::new();

    for line in lines {
        let (name, connections) = line.split_once(" = ").unwrap();
        let (left, right) = connections.split_once(", ").unwrap();
        let node = Node {
            conections: (left[1..].to_string(), right[0..3].to_string())
        };
        nodes.insert(name.to_string(), node);
    }

    let mut current_node_name = "AAA";
    let mut steps = 0;

    for instruction in instructions.iter().cycle() {
        steps += 1;
        let current_node = nodes.get(current_node_name).unwrap();
        match instruction {
            'R' => current_node_name = &current_node.conections.1,
            'L' => current_node_name = &current_node.conections.0,
            _ => unreachable!(),
        }
        if current_node_name == "ZZZ" {
            break;
        }
    }

    let starting_nodes = nodes
        .iter()
        .map(|(key, _)| key)
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<&String>>();

    let p2_steps = starting_nodes.into_iter().map(|name| {
        let mut name = name;
        let mut steps_taken = 0;
        for instruction in instructions.iter().cycle() {
            steps_taken += 1;
            let current_node = nodes.get(name).unwrap();
            match instruction {
                'R' => name = &current_node.conections.1,
                'L' => name = &current_node.conections.0,
                _ => unreachable!(),
            }
            if name.ends_with('Z') {
                break;
            }
        }
        steps_taken
    }).fold(1, |mut acc, step| {
        acc = lcm(acc, step);
        acc
    });

    let sol1: u64 = steps;
    let sol2: u64 = p2_steps;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug)]
struct Node {
    conections: (String, String),
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a;
    let mut min = b;

    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min
        }
        max = min;
        min = res;
    }
}