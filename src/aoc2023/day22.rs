use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day22").unwrap();
    let time = Instant::now();

    let mut bricks = text
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let start = start
                .split(',')
                .map(|s| s.parse().unwrap());
            let end = end
                .split(',')
                .map(|s| s.parse().unwrap());
            let mut combinded = start.zip(end);
            let (x1, x2): (u32, u32) = combinded.next().unwrap();
            let (y1, y2) = combinded.next().unwrap();
            let (z1, z2) = combinded.next().unwrap();
            let start = Point3D {
                x: x1.min(x2),
                y: y1.min(y2),
                z: z1.min(z1),
            };
            let end = Point3D {
                x: x1.max(x2),
                y: y1.max(y2),
                z: z1.max(z2),
            };
            Brick{
                start,
                end,
                supporting: Vec::new(),
                supporter_count: 0,
            }
        })
        .collect::<Vec<Brick>>();

    // test input is in order however reall input is not so sort by Z to have lowest bricks at the start?
    bricks.sort_by_key(|a| a.start.z);
    drop_bricks(&mut bricks, 0);
    bricks.sort_by_key(|a| a.start.z);

    let mut disintegrate_count = 0;
    for brick in bricks.iter() {
        if brick.supporting.is_empty() {
            disintegrate_count += 1;
        } else if brick.supporting.iter().all(|idx| bricks[*idx].supporter_count > 1) {
            disintegrate_count += 1;
        }
    }

    let mut chains = 0;
    for idx in 0..bricks.len() {
        let mut copy = bricks.clone();
        copy.remove(idx);
        let count_dropped = drop_bricks(&mut copy, idx);
        chains += count_dropped as u64;
    }


    let sol1: u64 = disintegrate_count;
    let sol2: u64 = chains;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn drop_bricks(bricks: &mut [Brick], start: usize) -> u32 {
    let mut count = 0;
    for idx in start..bricks.len() {
        let current_start = bricks[idx].start;
        let current_end = bricks[idx].end;

        let mut tallest_supporter = 0;
        for below_idx in (0..idx).rev() {
            let bellow_start = bricks[below_idx].start;
            let bellow_end = bricks[below_idx].end;
            if range_overlap(current_start.x, current_end.x, bellow_start.x, bellow_end.x)
                && range_overlap(current_start.y, current_end.y, bellow_start.y, bellow_end.y)
                && bellow_end.z > tallest_supporter
            {
                tallest_supporter = bellow_end.z;
            }
        }

        let z_diff = current_end.z - current_start.z;
        bricks[idx].start.z = tallest_supporter + 1;
        bricks[idx].end.z = tallest_supporter + 1 + z_diff;
        // scan for supportors
        for below_idx in (0..idx).rev() {
            let bellow_start = bricks[below_idx].start;
            let bellow_end = bricks[below_idx].end;
            if bellow_end.z + 1 == bricks[idx].start.z
                && range_overlap(current_start.x, current_end.x, bellow_start.x, bellow_end.x)
                && range_overlap(current_start.y, current_end.y, bellow_start.y, bellow_end.y)
            {
                bricks[idx].supporter_count += 1;
                bricks[below_idx].supporting.push(idx);
            }
        }

        // counts the dropped bricks
        if current_start.z > bricks[idx].start.z {
            count += 1;
        }
    }
    count
}

fn range_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    start1 <= end2 && start2 <= end1
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: Point3D,
    end: Point3D,
    // vector of whats supporting it?
    supporting: Vec<usize>,
    supporter_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}