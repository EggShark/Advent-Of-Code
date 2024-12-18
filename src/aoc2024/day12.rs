use crate::{shared::DIRS, Solve};
use std::{collections::{HashSet, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let grid = data_in
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut global_seen = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if !global_seen.insert((x as i32, y as i32)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((x as i32, y as i32));
            let mut area = 0;
            let mut perimiter = 0;
            let c = grid[y][x];

            while let Some((sx, sy)) = queue.pop_back() {
                area += 1;

                let mut neigboor_count = 0;
                for (dy, dx) in DIRS {
                    if sx + dx >= 0 && sy + dy >= 0
                    && sx + dx < grid.len() as i32 && sy + dy < grid.len() as i32
                    && grid[(sy + dy) as usize][(sx + dx) as usize] == c 
                    {
                        neigboor_count += 1;
                        if global_seen.insert((sx + dx, sy + dy)) {
                            queue.push_back((sx + dx, sy+dy));
                        }
                    }
                }
                perimiter += 4 - neigboor_count;
            }
            solve += area * perimiter;
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

    let grid = data_in
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut global_seen = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if !global_seen.insert((x as i32, y as i32)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((x as i32, y as i32));
            let mut area = 0;
            let c = grid[y][x];
            let mut perimeter_t = Vec::new();
            let mut perimeter_b = Vec::new();
            let mut perimeter_l = Vec::new();
            let mut perimeter_r = Vec::new();

            while let Some((sx, sy)) = queue.pop_back() {
                area += 1;

                for (dy, dx) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                    if sx + dx >= 0 && sy + dy >= 0
                    && sx + dx < grid.len() as i32 && sy + dy < grid.len() as i32
                    && grid[(sy + dy) as usize][(sx + dx) as usize] == c 
                    {
                        if global_seen.insert((sx + dx, sy + dy)) {
                            queue.push_back((sx + dx, sy+dy));
                        }
                        continue;
                    }

                    if dx == 1 {
                        perimeter_r.push((sx + dx, sy + dy));
                    } else if dx == -1 {
                        perimeter_l.push((sx + dx, sy+dy));
                    } else if dy == 1 {
                        perimeter_b.push((sx + dx, sy+dy));
                    } else {
                        perimeter_t.push((sx + dx, sy + dy));
                    }
                }
            }

            perimeter_l.sort_by_key(|&(x, y)| (x, y));
            perimeter_r.sort_by_key(|&(x, y)| (x, y));
            perimeter_t.sort_by_key(|&(x, y)| (y, x));
            perimeter_b.sort_by_key(|&(x, y)| (y, x));

            let sides = perimeter_l
                .chunk_by(|&(x1, y1), &(x2, y2)| x1==x2 && y1 + 1 >= y2)
                .count() +
                perimeter_r
                    .chunk_by(|&(x1, y1), &(x2, y2)| x1==x2 && y1 + 1 >= y2)
                    .count() +
                perimeter_t
                    .chunk_by(|&(x1, y1), &(x2, y2)| y1 == y2 && x1 + 1 >= x2)
                    .count() +
                perimeter_b
                    .chunk_by(|&(x1, y1), &(x2, y2)| y1 == y2 && x1 + 1 >= x2)
                    .count();
            solve += area * sides as u32;
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}