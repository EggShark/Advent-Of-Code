use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::ops::Range;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

type Map = [[Space; 1000]; 200];

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day14.txt").unwrap();
    let time = Instant::now();

    let mut grid: Map = [[Space::Air; 1000]; 200];

    let mut lines = text.lines();
    let _ = lines.next().unwrap();
    let line = lines.next().unwrap();

    let x = line.split(" -> ").map(|s|s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|point| Point::from_str(point))
        .collect::<Vec<Point>>();

    println!("{:?}", x);
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn build_cave(input: String) -> Map {
    let mut grid: Map = [[Space::Air; 1000]; 200];

    let lines = input.lines();
    for line in lines {
        let shape = line.split(" -> ").map(|s|s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|point| Point::from_str(point))
        .collect::<Vec<Point>>();
        let iter = shape.iter();
    }

    grid
}

#[derive(Copy, Clone ,Debug, PartialEq)]
enum Space {
    Air,
    Rock,
    Sand,
}

#[derive(Copy, Clone ,Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from_str(s: &str) -> Self {
        let (left, right) = s.split_once(",").unwrap();
        Self {
            x: left.parse().unwrap(),
            y: right.parse().unwrap(),
        }
    }

    pub fn get_ranges(&self, other: Point) -> (Range<i32>, Range<i32>) {
        (self.x..other.x, self.y..other.y)
    }
}