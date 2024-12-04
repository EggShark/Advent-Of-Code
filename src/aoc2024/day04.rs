use std::time::Instant;
use crate::Solve;
///////////////////////////////////////////////////////////////////////////////

const FORWARD: &[u8] = &[b'X', b'M', b'A', b'S'];
const BACKWARD: &[u8] = &[b'S', b'A', b'M', b'X'];

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let grid = data_in.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if x < grid[y].len() - 3 {
                if &grid[y][x..x+4] == FORWARD || &grid[y][x..x+4] == BACKWARD {
                    solve += 1;
                }          
            }

            if y < grid.len() - 3 {
                let h = [grid[y][x], grid[y+1][x], grid[y+2][x], grid[y+3][x]];
                if h == BACKWARD || h == FORWARD {
                    solve += 1;
                }
            }


            if x < grid[y].len() - 3 && y < grid.len() - 3 {
                let dir_1 = [grid[y][x], grid[y+1][x+1], grid[y+2][x+2], grid[y+3][x+3]];
                let dir_2 = [grid[y][x+3], grid[y+1][x+2], grid[y+2][x+1], grid[y+3][x]];
                
                for dir in [dir_1, dir_2] {
                    if dir == BACKWARD || dir == FORWARD {
                        solve += 1;
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
    let grid = data_in.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    for y in 1..grid.len() - 1 {
        for x in 1..grid.len() - 1 {
            if grid[y][x] == b'A' {
                let left = [grid[y-1][x-1], grid[y+1][x+1]];
                let right = [grid[y-1][x+1], grid[y+1][x-1]];
                if (left == [b'M', b'S'] || left == [b'S', b'M'])
                    && (right == [b'M', b'S'] || right == [b'S', b'M'])
                {
                    solve += 1;
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