use crate::Solve;
use std::{collections::{HashMap, HashSet}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    // assuming square input
    let size = (data_in.len() as f64).sqrt() as i32;

    let mut positions: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in data_in.lines().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char != b'.' {
                let to_edit = positions.entry(char).or_default();
                to_edit.push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, v) in positions {
        for i in 0..v.len() {
            let (x1, y1) = v[i];
            for j in i+1..v.len() {
                let (x2, y2) = v[j];
                let dx = x1 - x2;
                let dy = y1 - y2;

                if x1 + dx >= 0 && x1 + dx < size && y1 + dy >= 0 && y1 + dy < size {
                    antinodes.insert((x1 + dx, y1 + dy));
                }

                if x2 - dx >= 0 && x2 - dx < size && y2 - dy >= 0 && y2 - dy < size {
                    antinodes.insert((x2 - dx, y2 - dy));
                }
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(antinodes.len()),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    // assuming sqaure input
    let size = (data_in.len() as f64).sqrt() as i32;
    let mut positions: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for (y, line) in data_in.lines().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char != b'.' {
                let to_edit = positions.entry(char).or_default();
                to_edit.push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for (_, v) in positions {
        for i in 0..v.len() {
            let (x1, y1) = v[i];
            for j in i+1..v.len() {
                let (x2, y2) = v[j];
                let dx = x1 - x2;
                let dy = y1 - y2;

                let mut mult = 0;
                loop {
                    if x1 + (dx * mult) >= 0 && x1 + (dx * mult) < size && y1 + (dy * mult) >= 0 && y1 + (dy * mult) < size {
                        antinodes.insert((x1 + dx * mult, y1 + dy * mult));
                    } else {
                        break;
                    }
                    mult += 1;
                    
                }
                mult = 0;

                loop {
                    if x2 - (dx * mult) >= 0 && x2 - (dx * mult) < size && y2 - (dy * mult) >= 0 && y2 - (dy * mult) < size {
                        antinodes.insert((x2 - dx * mult, y2 - dy * mult));
                    } else {
                        break;
                    }
                    mult += 1;
                }
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(antinodes.len()),
        time_ms,
    }
}