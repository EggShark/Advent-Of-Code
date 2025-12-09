use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let points = data_in.lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        (l.parse().unwrap(),r.parse().unwrap())
    }).collect::<Vec<(i64, i64)>>();

    let mut max_area = 0;
    for &(x1, y1) in points.iter() {
        for &(x2, y2) in points.iter() {
            let area = ((x1-x2+1)*(y1-y2+1)).abs();
            max_area = std::cmp::max(area, max_area);
        }
    }

    let solve = max_area;

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = 0;


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
