use crate::Solve;
use std::{collections::{HashSet, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////
const GRID_SIZE: usize = 71;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut grid = [[Tile::Empty; GRID_SIZE]; GRID_SIZE];
    data_in.
        lines()
        .take(1024)
        .filter_map(|d| d
            .split_once(',')
            .and_then(|(l, r)| Some((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())))
        ).for_each(|(x, y)| grid[y][x] = Tile::Courupted);

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    // x, y, score
    queue.push_back((0,0,0));
    seen.insert((0, 0));
    while let Some((x, y, length)) = queue.pop_front() {
        if (x, y) == (70, 70) {
            solve = length;
            break;
        }

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if x + dx >= 0 && y + dy >= 0
                && x + dx < GRID_SIZE as i32 && y + dy < GRID_SIZE as i32
                && grid[(y + dy) as usize][(x + dx) as usize] == Tile::Empty
                && seen.insert((x + dx, y + dy))
            {
                queue.push_back((x + dx, y + dy, length + 1));
            }
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

    let data_in = data_in.
        lines()
        .filter_map(|d| d
            .split_once(',')
            .and_then(|(l, r)| Some((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())))
        )
        .collect::<Vec<(usize, usize)>>();

    let mut original_grid = [[Tile::Empty; GRID_SIZE]; GRID_SIZE];
    for &(x, y) in data_in.iter().take(1024) {
        original_grid[y][x] = Tile::Courupted;
    }

    let mut min = 1024;
    let mut max = data_in.len() - 1;
    while min <= max {
        let idx = ((max-min) / 2) + min;
        let mut c = original_grid;
        data_in.iter().skip(1024).take(idx-1024).for_each(|&(x,y)| c[y][x] = Tile::Courupted);
        
        if way_out(&c) {
            min = idx + 1;
        } else {
            max = idx - 1;
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(format!("{},{}",data_in[max].0, data_in[max].1)),
        time_ms,
    }
}

fn way_out(grid: &[[Tile; GRID_SIZE]; GRID_SIZE]) -> bool {
    let mut seen = HashSet::new();
    let mut stack = vec![];
    stack.push((0,0));
    seen.insert((0,0));
    while let Some((x, y)) = stack.pop() {
        if (x, y) == (70, 70) {
            return true;
        }

        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if x + dx >= 0 && y + dy >= 0
                && x + dx < GRID_SIZE as i32 && y + dy < GRID_SIZE as i32
                && grid[(y + dy) as usize][(x + dx) as usize] == Tile::Empty
                && seen.insert((x + dx, y + dy))
            {
                stack.push((x + dx, y + dy));
            }
        }
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Courupted,
}