use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

type Map = [[Space; 1000]; 200];
const SANDDROPOFF: Point = Point{x: 500, y: 0};

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day14.txt").unwrap();
    let time = Instant::now();

    let mut grid = build_cave(&text);
    let mut grid2 = grid;
    add_floor(&mut grid2);

    let mut counter = 0;

    loop {
        if drop_sand(&mut grid) {
            counter += 1;
        } else {
            break;
        }
    }

    let mut c2 = 0;
    loop {
        if drop_sand(&mut grid2) {
            c2 += 1;
        } else {
            break;
        }
    }

    let sol1: i32 = counter;
    let sol2: i32 = c2 + 1; // the code above doesnt count the last bit of sand

    let solution = (SolutionType::I32(sol1), SolutionType::I32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn drop_sand(grid: &mut Map) -> bool {
    let mut pos = SANDDROPOFF;
    loop {
        if pos.y == 199 {
            return false; // into the void
        }
        let bellow = grid[(pos.y + 1) as usize][(pos.x) as usize];
        let left = grid[(pos.y + 1) as usize][(pos.x - 1) as usize];
        let right = grid[(pos.y + 1) as usize][(pos.x + 1) as usize];
        if bellow == Space::Air {
            pos.y += 1;
        } else {
            //try left then right
            if left == Space::Air {
                pos.y += 1;
                pos.x -= 1;
            } else if right == Space::Air {
                pos.y += 1;
                pos.x += 1;
            } else {
                // hes resting
                grid[pos.y as usize][pos.x as usize] = Space::Sand;
                if pos == SANDDROPOFF {
                    return false;
                }
                break;
            }
        }
    }
    true
}

fn build_cave(input: &String) -> Map {
    let mut grid: Map = [[Space::Air; 1000]; 200];

    let lines = input.lines();
    for line in lines {
        let shape = line.split(" -> ").map(|s|s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|point| Point::from_str(point))
        .collect::<Vec<Point>>();

        let mut iter = shape.iter();
        let mut prev = iter.next().unwrap();
        grid[prev.y as usize][prev.x as usize] = Space::Rock;
        while let Some(new_point) = iter.next() {
            if prev.x > new_point.x {
                // 30..10
                let range = new_point.get_x_range(*prev);
                for new_x in range {
                    grid[prev.y as usize][new_x as usize] = Space::Rock;
                }
            } else if prev.x < new_point.x {
                //10..30
                let range = prev.get_x_range(*new_point);
                for new_x in range {
                    grid[prev.y as usize][new_x as usize] = Space::Rock;
                }
            } else if prev.y > new_point.y {
                let range = new_point.get_y_range(*prev);
                for new_y in range {
                    grid[new_y as usize][prev.x as usize] = Space::Rock;
                }
            } else {
                // prev_y < y
                let range = prev.get_y_range(*new_point);
                for new_y in range {
                    grid[new_y as usize][prev.x as usize] = Space::Rock;
                }
            }
            prev = new_point;
        }
        grid[prev.y as usize][prev.x as usize] = Space::Rock;
    }

    grid
}

fn add_floor(grid: &mut Map) {
    let mut largest_y = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Space::Rock && y > largest_y {
                largest_y = y;
            }
        }
    }
    let floor_height = largest_y + 2;
    for x in 0..1000 {
        grid[floor_height][x] = Space::Rock;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn get_x_range(&self, other: Point) -> RangeInclusive<i32> {
        self.x..=other.x
    }

    pub fn get_y_range(&self, other: Point) -> RangeInclusive<i32> {
        self.y..=other.y
    }
}