use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day04.txt").unwrap();
    let time = Instant::now();


    let mut vec1 = Vec::new();
    for line in text.lines() {
        let z = line.split(",");
        for s in z {
            vec1.push(s);
        }
    }

    let mut vec2 = Vec::new();
    for s in vec1.iter() {
        let z = s.split("-").collect::<Vec<&str>>();
        vec2.push([z[0].parse::<u32>().unwrap(), z[1].parse::<u32>().unwrap()]);
    }
    drop(vec1);

    let mut pos_counter = 0;
    let mut counter = 0;
    let mut counter2 = 0;

    while pos_counter < vec2.len() - 1 {
        let a = vec2[pos_counter][0];
        let b = vec2[pos_counter][1];
        let c = vec2[pos_counter+1][0];
        let d = vec2[pos_counter+1][1];
        if (a <= c && b >= d) || (a >= c && b <= d) {
            counter += 1;
        }
        if b >= c && d >= a {
            counter2 += 1;
        }
        pos_counter += 2;
    }

    let sol1: u64 = counter;
    let sol2: u64 = counter2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}