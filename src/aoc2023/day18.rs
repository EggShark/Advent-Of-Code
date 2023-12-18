use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::str::SplitWhitespace;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day18").unwrap();
    let time = Instant::now();

    let mut current_pos: (i64, i64) = (0, 0);
    let mut circ = 0;
    let points = text
        .lines()
        .map(|line| {
            let items = line.split_whitespace();
            let p = Point::new(current_pos, items, &mut circ);
            current_pos = (p.x, p.y);
            p
        })
        .collect::<Vec<Point>>();

    let mut current_pos = (0, 0);
    let mut circ = 0;
    let p2_points = text
        .lines()
        .map(|line| {
            let items = line.split_whitespace();
            let p = Point::new_p2(current_pos, items, &mut circ);
            current_pos = (p.x, p.y);
            p
        })
        .collect::<Vec<Point>>();
    

    let sol1: u64 = shoelace(&points) as u64 + (circ / 2) + 1;
    let sol2: u64 = shoelace(&p2_points) as u64 + (circ / 2) + 1;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(current_pos: (i64, i64), mut text: SplitWhitespace, circ: &mut u64) -> Self {
        let direction = text.next().unwrap();
        let distance: i64 = text.next().unwrap().parse().unwrap();
        *circ += distance as u64;

        let (x, y) = match direction {
            "R" => (current_pos.0 + distance, current_pos.1),
            "L" => (current_pos.0 - distance, current_pos.1),
            "U" => (current_pos.0, current_pos.1 + distance),
            "D" => (current_pos.0, current_pos.1 - distance),
            _ => unreachable!()
        };

        Self {
            x,
            y,
        }
    }

    fn new_p2(current_pos: (i64, i64), mut text: SplitWhitespace, circ: &mut u64) -> Self {
        let hex_str = text.nth(2).unwrap();
        let distance = i64::from_str_radix(&hex_str[2..7], 16).unwrap();
        *circ += distance as u64;
        let (x, y) = match &hex_str[7..8] {
            "0" => (current_pos.0 + distance, current_pos.1),
            "2" => (current_pos.0 - distance, current_pos.1),
            "3" => (current_pos.0, current_pos.1 + distance),
            "1" => (current_pos.0, current_pos.1 - distance),
            _ => unreachable!(),
        }; 

        Self {
            x,
            y,
        }
    }
}

// could probably windows iter this but oh well
fn shoelace(points: &[Point]) -> i64 {
    let mut s1 = 0;
    for idx in 0..points.len() - 1 {
        s1 += points[idx].x * points[idx + 1].y;
    }
    s1 += points[0].y * points.last().unwrap().x;

    let mut s2 = 0;
    for idx in 0..points.len() - 1 {
        s2 += points[idx].y * points[idx + 1].x;
    }
    s2 += points[0].x * points.last().unwrap().y;

    ((s1 - s2) / 2).abs()
}