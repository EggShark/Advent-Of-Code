use std::{collections::HashSet, time::Instant};
///////////////////////////////////////////////////////////////////////////////
use crate::Solve;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let grid = data_in
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|b| Tile::from(*b)).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let mut pos: (i32, i32) = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == Tile::Start {
                pos = (y as i32, x as i32);
            }
        }
    }

    let solve = get_path(&grid, pos).len();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let grid = data_in
        .lines()
        .map(|line| line.as_bytes().into_iter().map(|b| Tile::from(*b)).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    let mut start_pos: (i32, i32) = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] == Tile::Start {
                start_pos = (y as i32, x as i32);
            }
        }
    }

    let orignal_path = get_path(&grid, start_pos);

    for (y, x) in orignal_path {
        if grid[y as usize][x as usize] == Tile::Start {
            continue;
        }

        let mut pos = start_pos;
        let mut dir = Direction::Up;
        let obstacle = (y as i32, x as i32);
        let mut visted_with_dir: HashSet<(i32, i32, Direction)> = HashSet::new();

        loop {
            if !visted_with_dir.insert((pos.0, pos.1, dir)) {
                solve += 1;
                break;
            }

            let (dy, dx) = dir.into_value();
            if pos.0 + dy < 0 || pos.0 + dy >= grid.len() as i32 || pos.1 + dx < 0 || pos.1 + dx >= grid.len() as i32 {
                break;
            }
    
            if grid[(pos.0 + dy) as usize][(pos.1 + dx) as usize] == Tile::Occupied || (pos.0 + dy, pos.1 + dx) == obstacle {
                dir = dir.rotate_right();
            } else {
                pos = (pos.0 + dy, pos.1 + dx);
            }
        }
    
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn get_path(grid: &[Vec<Tile>], mut pos: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut visted: HashSet<(i32, i32)> = HashSet::new();
    let mut dir = Direction::Up;
    loop {
        visted.insert(pos);
        let (y, x) = dir.into_value();
        if pos.0 + y < 0 || pos.0 + y >= grid.len() as i32 || pos.1 + x < 0 || pos.1 + x >= grid.len() as i32 {
            break;
        }

        if grid[(pos.0 + y) as usize][(pos.1 + x) as usize] == Tile::Occupied {
            dir = dir.rotate_right();
        } else {
            pos = (pos.0 + y, pos.1 + x);
        }
    }

    visted
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Occupied,
    Start,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::Occupied,
            b'^' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    fn into_value(&self) -> (i32, i32) {
        match self {
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
        }
    }
}