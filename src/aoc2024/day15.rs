use crate::{Solve, DOUBLE_NEW_LINE};
use std::{collections::{HashSet, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let (grid, moves) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut grid = grid
        .lines()
        .map(|l| l.bytes().map(|b| b.into()).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();
    let directions = moves.bytes().filter(|b| *b != b'\n').map(|b| to_dir(b));

    let mut pos = ((grid.len() / 2 - 1) as i16, (grid.len() / 2 - 1) as i16);

    for (dx, dy) in directions {
        if grid[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Wall {
            continue;
        }

        if grid[(pos.1 + dy) as usize][(pos.0 + dx) as usize] == Tile::Empty {
            pos.0 += dx;
            pos.1 += dy;
        } else {

            let mut s_pos = (pos.0 + dx, pos.1 + dy);
            while grid[(s_pos.1) as usize][(s_pos.0) as usize] == Tile::Box {
                s_pos.0 += dx;
                s_pos.1 += dy;
            }

            if grid[s_pos.1 as usize][s_pos.0 as usize] != Tile::Wall {
                // update player pos and grid
                pos.0 += dx;
                pos.1 += dy;
                grid[pos.1 as usize][pos.0 as usize] = Tile::Empty;
                grid[s_pos.1 as usize][s_pos.0 as usize] = Tile::Box;
            }
        }
    }

    let mut score = 0;
    for y in 1..grid.len()-1 {
        for x in 1..grid.len()-1 {
            if grid[y][x] != Tile::Box {
                continue;
            }

            score += y * 100 + x;
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(score),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let (grid, moves) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut grid = grid
        .lines()
        .map(|l| l.bytes().map(|b| {
            match b {
                b'#' => [Tile::Wall, Tile::Wall],
                b'O' => [Tile::BoxLeft, Tile::BoxRight],
                b'.' => [Tile::Empty, Tile::Empty],
                b'@' => [Tile::Start, Tile::Empty],
                _ => unreachable!(),
            }
            }).flatten()
            .collect::<Vec<Tile>>()
        ).collect::<Vec<_>>();

    let directions = moves.bytes().filter(|b| *b != b'\n').map(|b| to_dir(b));
    let mut pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            if *t == Tile::Start {
                pos = (x as i16, y as i16);
                break;
            }
        }
    }
    grid[pos.1 as usize][pos.0 as usize] = Tile::Empty;

    println!("{:?}", pos);

    for (dx, dy) in directions {
        let mut moving_objects: HashSet<(i16, i16, Tile)> = HashSet::new();
        let mut qeue: VecDeque<(i16, i16)> = VecDeque::new();
        qeue.push_back((pos.0 + dx, pos.1 + dy));
        let mut can_move = true;
        while let Some((sx, sy)) = qeue.pop_back() {
            match grid[sy as usize][sx as usize] {
                Tile::Wall => {
                    can_move = false;
                    break;
                }
                Tile::Empty => {continue;}
                Tile::BoxLeft => {
                    if moving_objects.insert((sx, sy, grid[sy as usize][sx as usize])) {
                        moving_objects.insert((sx + 1, sy, grid[sy as usize][sx as usize + 1]));
                        qeue.push_back((sx + dx, sy + dy));
                        qeue.push_back((sx + dx + 1, sy + dy));
                    }
                }
                Tile::BoxRight => {
                    if moving_objects.insert((sx, sy, grid[sy as usize][sx as usize])) {
                        moving_objects.insert((sx - 1, sy, grid[sy as usize][sx as usize - 1]));
                        qeue.push_back((sx + dx, sy + dy));
                        qeue.push_back((sx + dx -1, sy + dy));
                    }
                }
                e => panic!("WHA {:?}", e)
            }
        }

        if !can_move {
            continue;
        }

        for (x, y, _) in moving_objects.iter() {
            grid[*y as usize][*x as usize] = Tile::Empty;
        }

        for (x, y, t) in moving_objects {
            grid[(y + dy) as usize][(x + dx) as usize] = t;
        }
        pos.0 += dx;
        pos.1 += dy;
    }

    let mut score = 0;
    for y in 1..grid.len() - 1 {
        for x in 2..grid.len()*2 - 2 {
            if grid[y][x] == Tile::BoxLeft {
                score += y * 100 + x;
            }
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(score),
        time_ms,
    }
}

fn to_dir(v: u8) -> (i16, i16) {
    match v {
        b'^' => (0, -1),
        b'v' => (0, 1),
        b'<' => (-1 , 0),
        b'>' => (1, 0),
        e => unimplemented!("{}", e),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Start,
    Wall,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'O' => Tile::Box,
            b'#' => Tile::Wall,
            b'@' => Tile::Empty,
            _ => unimplemented!(),
        }
    }
}