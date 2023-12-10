use crate::{Solution, SolutionType};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day10").unwrap();
    let time = Instant::now();

    let map = text.lines().map(|f| {
        f
            .chars()
            .map(|c| Tile::from(c))
            .collect::<Vec<Tile>>()
    }).collect::<Vec<Vec<Tile>>>();

    let mut starting_positon = (0_i32, 0_i32);

    // finds starting
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Tile::StartingPostion {
                starting_positon = (x as i32, y as i32);
                break;
            }
        }
    }

    // to lazy to make algo for this just looked at input
    let mut direction = Direction::North;

    let mut steps = 0;
    let mut current_position = starting_positon;

    // double heap ik but saves searching through an array
    let mut poly = Vec::new();
    let mut visted_points = HashSet::new();

    
    loop {
        steps += 1;
        
        poly.push((current_position.0 as f64, current_position.1 as f64));
        visted_points.insert(current_position);
        
        let (x, y) = direction.to_numbers();
        current_position = (current_position.0 + x, current_position.1 + y);
        let pipe_at_current = map[current_position.1 as usize][current_position.0 as usize];
        
        direction = match pipe_at_current {
            Tile::VerticalPipe | Tile::HorizontalPipe => direction,
            Tile::NorthEastBend => {
                if direction == Direction::North || direction == Direction::South {
                    Direction::East
                } else {
                    Direction::North
                }
            },
            Tile::NorthWestBend => {
                if direction == Direction::North || direction == Direction::South {
                    Direction::West
                } else {
                    Direction::North
                }
            },
            Tile::SouthEastBend => {
                if direction == Direction::South || direction == Direction::North {
                    Direction::East
                } else {
                    Direction::South
                }
            },
            Tile::SouthWestBend => {
                if direction == Direction::South || direction == Direction::North {
                    Direction::West
                } else {
                    Direction::South
                }
            },
            Tile::StartingPostion => break,
            Tile::Ground => panic!(),
        }
    }

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visted_points.contains(&(x as i32, y as i32)) && is_point_in_polygon((x as f64, y as f64), &poly) {
                count += 1;
            }
        }
    }


    let sol1: u64 = steps / 2;
    let sol2: u64 = count;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

// just point in poly collision
fn is_point_in_polygon(point: (f64, f64), polygon: &[(f64, f64)]) -> bool {
    let mut is_inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        let intersect = ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj-xi) * (point.1 - yi) / (yj - yi) + xi);

        if intersect {
            is_inside = !is_inside
        }
        j = i;
    }

    is_inside
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    StartingPostion,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NorthEastBend,
            'J' => Self::NorthWestBend,
            '7' => Self::SouthWestBend,
            'F' => Self::SouthEastBend,
            '.' => Self::Ground,
            'S' => Self::StartingPostion,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_numbers(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::South => (0, 1),
        }
    }
}