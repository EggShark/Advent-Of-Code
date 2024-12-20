use crate::Solve;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut grid = data_in
        .lines()
        .map(|d| d.bytes().map(|b| Tile {kind:b, distance: u32::MAX}).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    'o: for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x].kind == b'S' {
                start = (x, y);
                break 'o;
            }   
        }
    }

    populate_distances(start, &mut grid);

    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x].kind == b'#' {
                continue;
            }

            for (nx, ny) in [(x, y + 2), (x + 1, y + 1), (x + 2, y), (x + 1, y - 1)] {
                if nx < grid.len() && ny < grid.len()
                    && grid[ny][nx].kind != b'#'
                    && grid[y][x].distance.abs_diff(grid[ny][nx].distance) >= 102
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

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut grid = data_in
        .lines()
        .map(|d| d.bytes().map(|b| Tile {kind:b, distance: u32::MAX}).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    'o: for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x].kind == b'S' {
                start = (x, y);
                break 'o;
            }   
        }
    }

    populate_distances(start, &mut grid);


    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x].distance == u32::MAX {
                continue;
            }

            for dx in -20..21_i32 {
                let dy_max = 20 - dx.abs();
                for dy in -dy_max..dy_max+1 {
                    let total_distance = dx.abs() + dy.abs();
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx < 0 || ny < 0 || ny >= grid.len() as i32 || nx >= grid.len() as i32 {
                        continue;
                    }

                    if grid[ny as usize][nx as usize].distance == u32::MAX {
                        continue;
                    }

                    if grid[ny as usize][nx as usize].distance >= 100 + grid[y][x].distance + total_distance as u32 {
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

fn populate_distances((mut x, mut y): (usize, usize), grid: &mut[Vec<Tile>]) {
    grid[y][x].distance = 0;

    while grid[y][x].kind != b'E' {
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if grid[ny][nx].kind != b'#'
                && grid[ny][nx].distance == u32::MAX
            {
                grid[ny][nx].distance = grid[y][x].distance + 1;
                (x, y) = (nx, ny);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    kind: u8,
    distance: u32,
}