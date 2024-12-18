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

    let mut grid = [[Tile::Empty; GRID_SIZE]; GRID_SIZE];
    let data_in = data_in.
        lines()
        .filter_map(|d| d
            .split_once(',')
            .and_then(|(l, r)| Some((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())))
        );

    data_in.clone().take(1024).for_each(|(x, y)| grid[y][x] = Tile::Courupted);
    let mut last = (0,0);

    'a: for (cx, cy) in data_in.skip(1024) {
        grid[cy][cx] = Tile::Courupted;

        let mut stack = vec![];
        let mut seen = HashSet::new();
        seen.insert((0, 0));
        stack.push((0,0));
        while let Some((x, y)) = stack.pop() {
            if (x, y) == (70, 70) {
                continue 'a;
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



        last = (cx, cy);
        break;
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(format!("{:?}", last)),
        time_ms,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Courupted,
}