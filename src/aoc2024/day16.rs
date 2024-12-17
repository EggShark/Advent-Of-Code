use crate::Solve;
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let grid = data_in
        .lines()
        .map(|l| l.bytes().map(|b| Tile::from(b)).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    let mut prio_queue = BinaryHeap::new();
    prio_queue.push(Reverse(State {score: 0, pos: (1, grid.len() as i32 - 2), dir: (1, 0)}));
    while let Some(Reverse(state)) = prio_queue.pop() {
        if (state.pos.0, state.pos.1) == (grid.len() as i32 - 2, 1) {
            solve = state.score;
            break;
        }


        if !seen.insert((state.pos.0, state.pos.1, state.dir.0, state.dir.1)) {
            continue;
        }

        prio_queue.push(Reverse(State {
            score: state.score + 1000,
            pos: state.pos,
            dir: (state.dir.1, -state.dir.0)
        }));
        prio_queue.push(Reverse(State {
            score: state.score + 1000,
            pos: state.pos,
            dir: (-state.dir.1, state.dir.0),
        }));

        let mut new_state = State { score: state.score + 1, pos: (state.pos.0 + state.dir.0, state.pos.1 + state.dir.1), dir: state.dir};
        while grid[new_state.pos.1 as usize][new_state.pos.0 as usize] != Tile::Wall {
            prio_queue.push(Reverse(new_state));
            new_state = State {
                score: new_state.score + 1,
                pos: (new_state.pos.0 + new_state.dir.0, new_state.pos.1 + new_state.dir.1),
                dir: new_state.dir,
            };
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

    let mut best_score = None;

    let grid = data_in
        .lines()
        .map(|l| l.bytes().map(|b| Tile::from(b)).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();

    let mut paths = HashMap::new();
   
    let mut seen = HashSet::new();
    let mut prio_queue = BinaryHeap::new();
    prio_queue.push((Reverse(0), (1, grid.len() as i32 - 2), (1, 0)));
    while let Some((Reverse(score), (x, y), (dx, dy))) = prio_queue.pop() {
        if (x, y) == (grid.len() as i32 - 2, 1) {
            best_score = Some(score);
        }

        if best_score.is_some_and(|c| c < score) {
            break;
        }


        if !seen.insert((x, y, dx, dy)) {
            continue;
        }

        prio_queue.push((Reverse(score + 1000), (x, y), (dy, -dx)));
        paths.entry((x, y, dy, -dx, score + 1000)).or_insert_with(Vec::new).push((x, y, dx, dy, score));
        prio_queue.push((Reverse(score + 1000), (x, y), (-dy, dx)));
        paths.entry((x, y, -dy, dx, score + 1000)).or_insert_with(Vec::new).push((x, y, dx, dy, score));

        let (mut sx, mut sy, mut new_score) = (x + dx, y + dy, score + 1);
        while grid[sy as usize][sx as usize] != Tile::Wall {
            paths.entry((sx, sy, dx, dy, new_score)).or_insert_with(Vec::new).push((sx - dx, sy - dy, dx, dy, new_score - 1));
            prio_queue.push((Reverse(new_score), (sx, sy), (dx, dy)));
            (sx, sy, new_score) = (sx + dx, sy + dy, new_score + 1);
        }
    } 

    let mut seen = HashSet::new();
    let mut best_seen = HashSet::new();
    let mut best_queue = vec![];
    
    for (dx, dy) in [(1,0), (-1,0), (0,1), (0,-1)] {
        let k = (grid.len() as i32 - 2, 1, dx, dy, best_score.unwrap());

        if paths.contains_key(&k) {
            best_queue.push(k);
        }
    }

    // walk all the paths white boy
    while let Some((x, y, dx, dy, score)) = best_queue.pop() {
        if !best_seen.insert((x, y, dx, dy)) {
            continue;
        }
        seen.insert((x, y));
        if let Some(path) = paths.get(&(x, y, dx, dy, score)) {
            for &(sx, sy, n_dx, n_dy, n_score) in path {
                best_queue.push((sx, sy, n_dx, n_dy, n_score));
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(seen.len()),
        time_ms,
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    End,
    Empty,
    Wall,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'S' => Tile::Start,
            b'E' => Tile::End,
            b'.' => Tile::Empty,
            b'#' => Tile::Wall,
            _ => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq, Ord, Clone, Copy)]
struct State {
    score: u32,
    pos: (i32, i32),
    dir: (i32, i32)
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}