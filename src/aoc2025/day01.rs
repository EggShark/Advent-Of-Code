use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = data_in
        .lines()
        .map(|l| (&l[0..1], l[1..].parse::<i32>().unwrap()))
        .fold((0, 50), |(s, p), (d, i)| {
            if d == "L" {
                (s + ((p-i)%100==0) as i32 ,(p-i).rem_euclid(100))
            } else {
                (s + ((p+i)%100 == 0) as i32, (p+i).rem_euclid(100))
            }
        }).0;


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let solve = data_in
        .lines()
        .map(|l| (&l[0..1], l[1..].parse::<i32>().unwrap()))
        .fold((0, 50), move |(c, p), (d, i)| {
            if d == "L" {
                (c + (i/100) + (i % 100  >= p && p !=0) as i32, (p-i).rem_euclid(100))
            } else {
                (c + (p + i)/100, (p+i).rem_euclid(100))
            }
        }).0;


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
