use crate::Solve;

use std::{collections::HashSet, time::Instant};

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

    let mut pos = Vec::new();
    let mut seen_shit = HashSet::new();
    pos.push((0, start_x));

    while let Some((y, x)) = pos.pop() {
        if y >= grid.len() - 2 {
            solve += 1;
        }
        for ys in y+1..grid.len() {
            if grid[ys][x] == Tile::Spliter {
                // hash tile and just auto solve += 2;
                // or also count how many times we get to the bottom splitters * 2 idk
                // keep track of # of splits and * 2 at end for score per beam ig?
                if seen_shit.insert((ys, x+1)) {
                    pos.push((ys, x+1));
                    solve += 1;
                }
                if seen_shit.insert((ys, x-1)) {
                    pos.push((ys, x-1));
                    solve += 1;
                }
                break;
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
