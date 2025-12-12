use crate::Solve;

use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut graph = data_in.lines().map(|l| {
        let (name, right) = l.split_once(": ").unwrap();
        let connections = right.split(' ').collect::<Vec<_>>();
        (name, connections)
    }).collect::<HashMap<&str, Vec<&str>>>();

    let mut start = graph.get(&"srv").unwrap().clone();

    while let Some(going_to) = start.pop() {
        if going_to == "out" {
            solve += 1;
            continue;
        }

        graph.get(going_to).unwrap().iter().for_each(|c| start.push(c));
    } 


    println!("p1 finished");

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve: u64 = 0;


    let graph = data_in.lines().map(|l| {
        let (name, right) = l.split_once(": ").unwrap();
        let connections = right.split(' ').collect::<Vec<_>>();
        (name, connections)
    }).collect::<HashMap<&str, Vec<&str>>>();

    let mut start = graph.get(&"svr").unwrap().clone().into_iter().map(|c| (c, HashSet::new())).collect::<Vec<_>>();

    while let Some((going_to, mut path)) = start.pop() {
        if going_to == "out" {
            if path.contains("dac") && path.contains("fft") {
                solve += 1;
            }
            continue;
        }


        if !path.insert(going_to) {
            continue;
        }

        graph.get(going_to).unwrap().iter().for_each(|c| start.push((c, path.clone())));
    } 


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
