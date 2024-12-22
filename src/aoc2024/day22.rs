use crate::Solve;
use std::{collections::{HashMap, HashSet}, time::Instant};
///////////////////////////////////////////////////////////////////////////////
const MOD: u64 = 0xFFFFFF;
pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = data_in
        .lines()
        .map(|d| d.parse::<u64>().unwrap())
        .map(|mut n| {
            for _ in 0..2000 {
                n = (n ^ (n << 6)) & MOD;
                n = (n ^ (n >> 5)) & MOD;
                n = (n ^ (n << 11)) & MOD;
            }
            n
        })
        .sum::<u64>();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut map: HashMap<(i64, i64, i64, i64), i64> =  HashMap::new();

    data_in
        .lines()
        .map(|d| d.parse::<u64>().unwrap())
        .for_each(|mut n| {
            let mut buyer = [0_u64; 2001];
            buyer[0] = n % 10;
            for i in 0..2000 {
                n = (n ^ (n << 6)) & MOD;
                n = (n ^ (n >> 5)) & MOD;
                n = (n ^ (n << 11)) & MOD;
                buyer[i+1] = n % 10;
            }
            let mut seen_suqence: HashSet<(i64, i64, i64, i64)> = HashSet::new();
            for slice in buyer.windows(5) {
                let a = slice[0] as i64;
                let b = slice[1] as i64;
                let c = slice[2] as i64;
                let d = slice[3] as i64;
                let e = slice[4] as i64;
                let (d1, d2, d3, d4) = (b-a, c-b, d-c, e-d);
                if seen_suqence.insert((d1, d2, d3, d4)) {
                    *map.entry((d1, d2, d3, d4)).or_insert(0) += e;
                }
            }
        });

    let solve = map.into_values().max().unwrap();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}