use crate::{Solution, SolutionType};
use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day14").unwrap();
    let time = Instant::now();

    let mut grid = text
        .lines()
        .map(|f| {
            f
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Tile>>()
        }).collect::<Vec<Vec<Tile>>>();

    tilt_north(&mut grid);

    let p1 = calculate_load(&grid);

    // hash set the load value and see what happens
    // should let us know when weve hit a repition?
    let mut grid = text
        .lines()
        .map(|f| {
            f
                .chars()
                .map(|c| c.into())
                .collect::<Vec<Tile>>()
        }).collect::<Vec<Vec<Tile>>>();

    let mut seen: HashSet<String> = HashSet::new();
    let mut cycle_found_at = 0;
    let mut count = 0;
    loop {
        spin_cylce(&mut grid);
        let hash_str = format!("{:?}", grid);
        if !seen.insert(hash_str.clone()) {
            if cycle_found_at == 0 {
                cycle_found_at = count;
                seen.clear();
                seen.insert(hash_str);
                count += 1;
                continue;
            }

            let mut remaining = 1000000000 - count - 1;
            remaining %= count - cycle_found_at;
            for _ in 0..remaining {
                spin_cylce(&mut grid);
            }
            break;
        }

        count += 1;
    }


    let sol1: u64 = p1;
    let sol2: u64 = calculate_load(&grid);

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Blocked,
    Empty,
    Roller,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Blocked,
            'O' => Self::Roller,
            _ => unreachable!(),
        }
    }
}

fn is_occupied(tile: Tile) -> bool {
    matches!(tile, Tile::Blocked | Tile::Roller)
}

fn spin_cylce(grid: &mut Vec<Vec<Tile>>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn tilt_north(grid: &mut Vec<Vec<Tile>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Tile::Roller {
                let mut new_y = y;
                while new_y > 0 {
                    if grid[new_y-1][x] == Tile::Empty {
                        new_y -= 1;
                    } else {
                        break;
                    }
                }
                grid[y][x] = Tile::Empty;
                grid[new_y][x] = Tile::Roller;
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<Tile>>) {
    for x in (0..grid[0].len()).rev() {
        for y in 0..grid.len() {
            if grid[y][x] == Tile::Roller {
                let mut new_x = x;
                while new_x < grid[0].len() - 1 {
                    if grid[y][new_x+1] == Tile::Empty {
                        new_x += 1;
                    } else {
                        break;
                    }
                }
                grid[y][x] = Tile::Empty;
                grid[y][new_x] = Tile::Roller;
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<Tile>>) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Tile::Roller {
                let mut new_y = y;
                while new_y < grid.len() - 1 {
                    if grid[new_y+1][x] == Tile::Empty {
                        new_y += 1;
                    } else {
                        break;
                    }
                }
                grid[y][x] = Tile::Empty;
                grid[new_y][x] = Tile::Roller;
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<Tile>>) {
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == Tile::Roller {
                let mut new_x = x;
                while new_x > 0 {
                    if grid[y][new_x-1] == Tile::Empty {
                        new_x -= 1;
                    } else {
                        break;
                    }
                }
                grid[y][x] = Tile::Empty;
                grid[y][new_x] = Tile::Roller;
            }
        }
    }
}

fn calculate_load(grid: &Vec<Vec<Tile>>) -> u64 {
    let mut sum = 0;
    for y in 0..grid.len() {
        let rollers = grid[y]
            .iter()
            .filter(|t| **t == Tile::Roller)
            .count();
        let distance_to_south = grid.len() - y;
        sum += (rollers * distance_to_south) as u64;
    }
    sum
}