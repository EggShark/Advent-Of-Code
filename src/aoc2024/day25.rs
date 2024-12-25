use crate::{Solve, DOUBLE_NEW_LINE};
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = 0;

    let mut keys = vec![];
    let mut locks = vec![];
    let stuff = data_in.split(DOUBLE_NEW_LINE);

    for thing in stuff {
        let grid = thing.lines().map(|d| d.bytes().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut h = [0, 0, 0, 0, 0];
        if grid.last().unwrap().contains(&b'#') {
            for c in 0..grid[0].len() {
                for r in (0..grid.len()).rev() {
                    if grid[r][c] == b'.' {
                        h[c] = 5 - r;
                        break;
                    }
                }
            }
            keys.push(h);
        } else {
            for c in 0..grid[0].len() {
                for r in 0..grid.len() {
                    if grid[r][c] == b'.' {
                        h[c] = r - 1;
                        break;
                    }
                }
            }
            locks.push(h);
        }
    }

    let mut solve = 0;
    for &key in keys.iter() {
        for &lock in locks.iter() {
            if key.iter().zip(lock).all(|(&k, l)| l + k <= 5) {
                solve += 1;
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
    let solve = 0;


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}