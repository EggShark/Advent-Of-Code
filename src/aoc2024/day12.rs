use crate::Solve;
use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant};
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
                for (dy, dx) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
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
    let data_in = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
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
            let mut seen = HashSet::new();
            queue.push_back((x as i32, y as i32));
            seen.insert((x as i32, y as i32));
            let mut area = 0;
            let mut perimiter = HashSet::new();
            let mut sides = 0;
            let c = grid[y][x];

            while !queue.is_empty() {
                let (sx, sy) = queue.pop_back().unwrap();
                area += 1;

                if sx == 0  {
                    if perimiter.insert((sx - 1, sy)) && !perimiter.contains(&(sx-1, sy +1)) && !perimiter.contains(&(sx-1, sy+1)) {
                        sides += 1;
                    }
                    
                } else if sx as usize == grid.len() - 1 {
                    if perimiter.insert((sx + 1, sy)) && !perimiter.contains(&(sx+1, sy +1)) && !perimiter.contains(&(sx+1, sy+1)) {
                        sides += 1;
                    }
                }

                if sy == 0  {
                    if perimiter.insert((sx, sy - 1)) && !perimiter.contains(&(sx+1, sy-1)) && !perimiter.contains(&(sx-1, sy-1)) {
                        sides += 1;
                    }
                } else if sy as usize == grid.len() - 1 {
                    if perimiter.insert((sx, sy + 1)) && !perimiter.contains(&(sx+1, sy+1)) && !perimiter.contains(&(sx-1, sy+1)) {
                        sides += 1;
                    }
                }

                for (dy, dx) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                    if sx + dx >= 0 && sy + dy >= 0
                    && sx + dx < grid.len() as i32 && sy + dy < grid.len() as i32
                    {
                        if grid[(sy + dy) as usize][(sx + dx) as usize] != c &&  perimiter.insert((sx + dx, sy + dy)) {
                            if dy != 0 && (!perimiter.contains(&(sx+1, sy+dy)) && !perimiter.contains(&(sx-1, sy+dy))) {
                                sides += 1;
                            } else if !perimiter.contains(&(sx+dx, sy+1)) && !perimiter.contains(&(sx+dx, sy-1)) {
                                sides += 1;
                            }
                        }
                        
                        if grid[(sy + dy) as usize][(sx + dx) as usize] == c && seen.insert((sx + dx, sy + dy)) {
                            queue.push_back((sx + dx, sy+dy));
                            global_seen.insert((sx + dx, sy + dy));
                        }
                    }
                }
            }
            println!("region {}, area: {}, sides: {}", c, area, sides);
            solve += area * sides;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}