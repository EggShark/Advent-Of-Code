use crate::Solve;
use std::{collections::HashMap, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut map = HashMap::new();
    let sum: u64 = data_in
        .split(' ')
        .map(|d| d.parse().unwrap()).map(|stone: u64| count_splits(stone, 25, &mut map))
        .sum();

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(sum),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    
    let mut map = HashMap::new();
    let sum: u64 = data_in
        .split(' ')
        .map(|d| d.parse().unwrap()).map(|stone: u64| count_splits(stone, 75, &mut map))
        .sum();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(sum),
        time_ms,
    }
}

fn count_splits(num: u64, times: u8, map: &mut HashMap<(u64, u8), u64>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(h) = map.get(&(num, times)) {
        return *h;
    } 

    let steps = if num == 0 {
        count_splits(1, times - 1, map)
    } else if (num.ilog10() + 1) % 2 == 0 {
        let split = 10_u64.pow((num.ilog10() + 1) / 2);
        count_splits(num / split, times - 1, map) + count_splits(num % split, times - 1, map)
    } else {
        count_splits(num * 2024, times - 1, map)
    };
    map.insert((num, times), steps);

    steps
}