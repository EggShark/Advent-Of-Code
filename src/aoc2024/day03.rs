use crate::Solve;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();


    let solve = data_in
        .split("mul(")
        .map(|text| {
            let (left, rest) = text.split_once(',')?;
            let (right, _) = rest.split_once(')')?;
            let left = left.parse::<u32>().ok()?;
            let right = right.parse::<u32>().ok()?;


            Some(left * right)
        })
        .flatten()
        .sum::<u32>();

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let solve = data_in
        .split("do()")
        .map(|doers| {
            let (text, _) = doers.split_once("don't()").unwrap_or((doers, ""));
            text
                .split("mul(")
                .map(|inst| {
                    let (left, rest) = inst.split_once(',')?;
                    let (right, _) = rest.split_once(')')?;
                    let left = left.parse::<u32>().ok()?;
                    let right = right.parse::<u32>().ok()?;


                    Some(left * right)
                })
                .flatten()
                .sum::<u32>()
        }).sum::<u32>();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}