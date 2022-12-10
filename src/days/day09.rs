use crate::{Solution, SolutionType};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day09.txt").unwrap();
    let time = Instant::now();

    let mut head = Point{x:0,y:0};
    let mut tail: [Point; 9] = [Point{x:0,y:0}; 9];
    let directions = text.lines().map(|line| Insturction::from_str(line)).collect::<Vec<Insturction>>(); 

    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();
    p1.insert(tail[0]);
    p2.insert(tail[8]);

    for ins in directions {
        for _ in 0..ins.amount {
            // pull the tail
            match ins.direction {
                Direction::Down => head.y += 1,
                Direction::Up => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }

            let mut next_not = head;
            for not in tail.iter_mut() {
                let distance_x = next_not.x - not.x;
                let distance_y = next_not.y - not.y;
    
                if !(distance_y.abs() < 2 && distance_x.abs() < 2) {
                    not.x += distance_x.signum();
                    not.y += distance_y.signum();
                }
                next_not = *not
            }
            p1.insert(tail[0]);
            p2.insert(tail[8]);
        }

    }
    

    let sol1: usize = p1.len();
    let sol2: usize = p2.len();

    let solution = (SolutionType::USIZE(sol1), SolutionType::USIZE(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Insturction {
    direction: Direction,
    amount: u8,
}

impl Insturction {
    pub fn from_str(s: &str) -> Self {
        let (dir, num_steps) = s.split_once(' ').unwrap();
        Self {
            direction: Direction::from_str(dir),
            amount: num_steps.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!()
        }
    }
}