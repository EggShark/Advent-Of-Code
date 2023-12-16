use crate::{Solution, SolutionType};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day16").unwrap();
    let time = Instant::now();

//     let text = r".|...\....
// |.-.\.....
// .....|-...
// ........|.
// ..........
// .........\
// ..../.\\..
// .-.-/..|..
// .|....-|.\
// ..//.|....";

    let grid = text
        .lines()
        .map(|line| {
            line
                .bytes()
                .map(|b| b.into())
                .collect::<Vec<Tile>>()
        }).collect::<Vec<Vec<Tile>>>();

    let p1 = bfs(
        Beam{
            x: -1,
            y: 0,
            direction: Direction::Right
        },
        &grid
    );

    // brute force it
    let mut p2 = 0;
    for num in 0..grid.len() {
        let bfs1 = bfs(
            Beam {
                x: -1,
                y: num as i32,
                direction: Direction::Right,
            },
            &grid
        );
        p2 = p2.max(bfs1);
        let bfs2 = bfs(
            Beam {
                x: grid[0].len() as i32,
                y: num as i32,
                direction: Direction::Left,
            },
            &grid
        );
        p2 = p2.max(bfs2);
    }

    for num in 0..grid[0].len() {
        let bfs1 = bfs(
            Beam {
                x: num as i32,
                y: -1,
                direction: Direction::Down,
            },
            &grid,
        );
        p2 = p2.max(bfs1);
        let bfs2 = bfs(
            Beam {
                x: num as i32,
                y: grid.len() as i32,
                direction: Direction::Up,
            },
            &grid
        );
        p2 = p2.max(bfs2);
    }

    let sol1: u64 = p1;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    RightUp,
    RightDown,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'|' => Self::VerticalSplitter,
            b'-' => Self::HorizontalSplitter,
            b'/' => Self::RightUp,
            b'\\' => Self::RightDown,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '/' => Self::RightUp,
            '\\' => Self::RightDown,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_horz(&self) -> bool {
        self == &Self::Left || self == &Self::Right
    }

    fn is_vert(&self) -> bool {
        self == &Self::Up || self == &Self::Down
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Beam {
    fn change_postion(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

fn bfs(start: Beam, grid: &[Vec<Tile>]) -> u64 {
    let mut beams = VecDeque::new();
    beams.push_front(start);

    let grid_height = grid.len() as i32;
    let grid_width = grid[0].len() as i32;

    let mut energized_tiles: HashSet<Beam> = HashSet::new();

    while beams.len() > 0 {
        let mut current_beam = beams.pop_front().unwrap();
        current_beam.change_postion();

        if current_beam.y < 0 || current_beam.y >= grid_height || current_beam.x < 0 || current_beam.x >= grid_width {
            continue;
        }

        let tile = grid[current_beam.y as usize][current_beam.x as usize];

        match tile {
            Tile::Empty => {
                if energized_tiles.insert(current_beam) {
                    beams.push_front(current_beam);
                    continue;
                }
            },
            Tile::HorizontalSplitter if current_beam.direction.is_horz() => {
                if energized_tiles.insert(current_beam) {
                    beams.push_front(current_beam);
                    continue;
                }
            },
            Tile::VerticalSplitter if current_beam.direction.is_vert() => {
                if energized_tiles.insert(current_beam) {
                    beams.push_front(current_beam);
                    continue;
                }
            },
            Tile::RightDown => {
                match current_beam.direction {
                    Direction::Right => current_beam.direction = Direction::Down,
                    Direction::Left => current_beam.direction = Direction::Up,
                    Direction::Up => current_beam.direction = Direction::Left,
                    Direction::Down => current_beam.direction = Direction::Right,
                }
                if energized_tiles.insert(current_beam) {
                    beams.push_front(current_beam);
                    continue;
                }
            },
            Tile::RightUp => {
                match current_beam.direction {
                    Direction::Right => current_beam.direction = Direction::Up,
                    Direction::Left => current_beam.direction = Direction::Down,
                    Direction::Up => current_beam.direction = Direction::Right,
                    Direction::Down => current_beam.direction = Direction::Left,
                }
                if energized_tiles.insert(current_beam) {
                    beams.push_front(current_beam);
                    continue;
                }
            },
            Tile::HorizontalSplitter => {
                let b1 = Beam {
                    x: current_beam.x,
                    y: current_beam.y,
                    direction: Direction::Left
                };
                let b2 = Beam {
                    x: current_beam.x,
                    y: current_beam.y,
                    direction: Direction::Right
                };
                if energized_tiles.insert(b1) {
                    beams.push_front(b1);
                }
                if energized_tiles.insert(b2) {
                    beams.push_front(b2);
                }
            },
            Tile::VerticalSplitter => {
                let b1 = Beam {
                    x: current_beam.x,
                    y: current_beam.y,
                    direction: Direction::Up
                };
                let b2 = Beam {
                    x: current_beam.x,
                    y: current_beam.y,
                    direction: Direction::Down
                };
                if energized_tiles.insert(b1) {
                    beams.push_front(b1);
                }
                if energized_tiles.insert(b2) {
                    beams.push_front(b2);
                }
            }
        }
    }

    energized_tiles
        .iter()
        .map(|b| (b.x, b.y))
        .collect::<HashSet<(i32, i32)>>()
        .len() as u64
}