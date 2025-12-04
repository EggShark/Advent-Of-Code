use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;
    
    let grid = data_in.lines().map(|l| l.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'.' {
                continue;
            }
            
            let mut score = 0;
            let y_range = y.saturating_sub(1)..std::cmp::min(y+2, grid.len());

            for ys in y_range {
                for xs in x.saturating_sub(1)..std::cmp::min(x+2, grid[y].len()) {
                    if grid[ys][xs] == b'@' {
                        score += 1;
                    }
                }
            }

            if score - 1 < 4 {
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
    let mut solve = 0;

    let mut grid = data_in.lines().map(|l| l.bytes().collect::<Vec<u8>>()).collect::<Vec<_>>();
    let mut removed_count = 1;

    while removed_count != 0 {
        removed_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == b'.' {
                    continue;
                }
            
                let mut score = 0;
                let y_range = y.saturating_sub(1)..std::cmp::min(y+2, grid.len());

                for ys in y_range {
                    for xs in x.saturating_sub(1)..std::cmp::min(x+2, grid[y].len()) {
                        if grid[ys][xs] == b'@' {
                            score += 1;
                        }
                    }
                }

                if score - 1 < 4 {
                    solve += 1;
                    grid[y][x] = b'.';
                    removed_count += 1;
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
