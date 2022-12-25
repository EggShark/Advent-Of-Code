use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day25.txt").unwrap();
    let time = Instant::now();

    let mut sum = 0;
    for line in text.lines() {
        let mut n: i64 = 0;
        for c in line.chars() {
            n *= 5;
            match c {
                '2' => n+= 2,
                '1' => n += 1,
                '0' => n += 0,
                '-' => n -= 1,
                '=' => n -= 2,
                _ => panic!(),
            }
        }
        sum += n;
    }
    let mut string = String::new();
    while sum != 0 {
        let v = (sum + 2) % 5 - 2;
        match v {
            -2 => string.insert(0, '='),
            -1 => string.insert(0, '-'),
            0 => string.insert(0, '0'),
            1 => string.insert(0, '1'),
            2 => string.insert(0, '2'),
            _ => panic!()
        }
        sum -= v;
        sum /= 5;
    }

    let sol1: String = string;
    let sol2: u64 = 0;

    let solution = (SolutionType::Str(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}
