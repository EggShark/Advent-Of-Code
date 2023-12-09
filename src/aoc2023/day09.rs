use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day9").unwrap();
    let time = Instant::now();

    let histories: i32 = text.lines().map(|line| {
        line.split_whitespace().map(|thing| thing.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    })
    .map(|mut history| get_next_value(&mut history, 0))
    .sum();

    let rev_histories = text.lines().map(|line| {
        line.split_whitespace().map(|thing| thing.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    })
    .map(|mut history| get_first_value(&mut history, 0))
    .sum();

    let sol1: i32 = histories;
    let sol2: i32 = rev_histories;

    let solution = (SolutionType::I32(sol1), SolutionType::I32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn get_next_value(array: &mut Vec<i32>, depth: i32) -> i32 {
    let mut vec = vec![0_i32; array.len()-1];

    for idx in 0..array.len()-1 {
        let diff = array[idx + 1] - array[idx];
        vec[idx] = diff;
    }

    let z = if vec.windows(2).all(|w| w[0] == w[1]) {
        vec[0]
    } else {
        get_next_value(&mut vec, depth + 1)
    };

    array.push(z + *array.last().unwrap());

    *array.last().unwrap()
}

fn get_first_value(array: &mut Vec<i32>, depth: i32) -> i32 {
    let mut vec = vec![0_i32; array.len()-1];

    for idx in 0..array.len()-1 {
        let diff = array[idx + 1] - array[idx];
        vec[idx] = diff;
    }

    let z = if vec.windows(2).all(|w| w[0] == w[1]) {
        vec[0]
    } else {
        get_first_value(&mut vec, depth + 1)
    };

    array.insert(0, *array.first().unwrap() - z);
    *array.first().unwrap()
}