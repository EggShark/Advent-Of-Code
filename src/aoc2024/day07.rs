use crate::Solve;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve: u64 = 0;
    for line in data_in.lines() {
        let (want, values) = line.split_once(": ").unwrap();
        let want = want.parse::<u64>().unwrap();
        let values = values.split(' ').map(|i| i.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        if try_operand(want, &values[1..], values[0]) {
            solve += want;
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut solve: u64 = 0;
    for line in data_in.lines() {
        let (want, values) = line.split_once(": ").unwrap();
        let want = want.parse::<u64>().unwrap();
        let values = values.split(' ').map(|i| i.parse::<u64>().unwrap()).collect::<Vec<u64>>();

        if try_p2(want, &values[1..], values[0]) {
            solve += want;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn try_operand(wanted: u64, numbers: &[u64], current_result: u64) -> bool {
    if numbers.is_empty() {
        return current_result == wanted
    }

    try_operand(wanted, &numbers[1..], current_result + numbers[0])
        || try_operand(wanted, &numbers[1..], current_result * numbers[0])
}

fn try_p2(wanted: u64, numbers: &[u64], current_result: u64) -> bool {
    if numbers.is_empty() {
        return current_result == wanted
    }

    try_p2(wanted, &numbers[1..], current_result + numbers[0])
        || try_p2(wanted, &numbers[1..], current_result * numbers[0])
        || try_p2(wanted, &numbers[1..], (current_result * (10_u64.pow(numbers[0].ilog10() + 1))) + numbers[0])
}