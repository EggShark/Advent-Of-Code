use std::time::Instant;
use crate::Solve;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = data_in
        .lines()
        .map(|line| line.split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
            .windows(3)
            .map(|slice| (slice[0] - slice[1], slice[1] - slice[2]))
            .all(|(val1, val2)| (val1.is_negative() == val2.is_negative() && i32::abs(val1) >= 1 && i32::abs(val1) <= 3 && i32::abs(val2) >= 1 && i32::abs(val2) <= 3))
        ).filter(|all| *all)
        .count();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let input = data_in.
    lines()
    .map(|line| line.split(' ')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>();

    let mut solve = 0;
    for line in input {
        let prev_decrease = line[0] > line[1];
        let mut bad_level = 0;
        for i in 0..line.len() - 1 {
            let diff = line[i] - line[i + 1];
            if (line[i] < line[i + 1] && prev_decrease)
                || (line[i] > line[i + 1] && !prev_decrease)
                || i32::abs(diff) > 3 || i32::abs(diff) < 1 {
                bad_level += 1;
            }
        }

        if bad_level <= 1{
            solve += 1;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}