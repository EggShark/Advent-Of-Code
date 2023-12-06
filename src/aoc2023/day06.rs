use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day6").unwrap();
    let time = Instant::now();

    // let text = "Time:      7  15   30
    // Distance:  9  40  200
    // ";

    let mut lines = text.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let required_speeds = times.iter()
        .zip(distances.clone())
        .map(|(times, distance)| distance/times)
        .collect::<Vec<u64>>();


    let mut p1 = 1;
    for idx in 0..4 {
        let req_distance = distances[idx];
        let tot_time = times[idx];
        let base_speed = required_speeds[idx];
        let mut sum = 0;
        for speed in base_speed..tot_time {
            let distance_travled = speed * (tot_time - speed);
            if distance_travled > req_distance {
                sum += 1;
            }
        }
        p1 *= sum;
    }

    let mut p2_text = text.lines();
    let race_time = p2_text
        .next()
        .unwrap()
        .split_once(":").unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = p2_text
        .next()
        .unwrap()
        .split_once(":").unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let req_speed= race_time/distance;

    let mut p2 = 0;

    for speed in req_speed..race_time {
        let distance_travled = (race_time-speed) * speed;
        if distance_travled > distance {
            p2 += 1;
        }
    }

    // leaving this here bc its so much slower but looks better imo
    // let p2 = (req_speed..race_time)
    //     .filter(|speed| (race_time-speed) * speed > distance)
    //     .count();

    let sol1: u64 = p1;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}