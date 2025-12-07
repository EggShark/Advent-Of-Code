use crate::Solve;

use std::{collections::{HashMap, HashSet}, time::Instant};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Spliter,
    Beam,
    Start,
}

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut grid = data_in.lines().map(|l| l.bytes().map(|b| match b {
        b'S' => Tile::Start,
        b'.' => Tile::Empty,
        b'|' => Tile::Beam,
        b'^' => Tile::Spliter,
        _ => unreachable!()
    }).collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let start_x = grid[0].len() / 2;
    grid[0][start_x] = Tile::Beam;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Tile::Beam {
                if let Some(t) = grid.get(y+1).map(|r| r[x]) {
                    match t {
                        Tile::Empty => grid[y+1][x] = Tile::Beam,
                        Tile::Spliter => {
                            solve += 1;
                            grid[y+1][x-1] = Tile::Beam;
                            grid[y+1][x+1] = Tile::Beam;
                        },
                        Tile::Beam => {},
                        _ => unreachable!()
                    }
                }
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

    let grid = data_in.lines().map(|l| l.bytes().map(|b| match b {
        b'S' => Tile::Start,
        b'.' => Tile::Empty,
        b'|' => Tile::Beam,
        b'^' => Tile::Spliter,
        _ => unreachable!()
    }).collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let start_x = grid[0].len() / 2;

    let mut map = HashMap::new();

    let solve = dfs_memoized((0, start_x), &grid, &mut map);


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn dfs_memoized((y, x): (usize, usize), grid: &[Vec<Tile>], map: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(&v) = map.get(&(y, x)) {
        return v;
    }

    let time_lines = if let Some ((ys, _)) = grid.iter().enumerate().skip(y+1).find(|(_, r)| r[x] == Tile::Spliter) {
        dfs_memoized((ys, x+1), grid, map) + dfs_memoized((ys, x-1), grid, map)
    } else {
        1
    };


    map.insert((y,x), time_lines);

    time_lines
}
