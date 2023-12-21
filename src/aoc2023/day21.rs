use crate::{Solution, SolutionType};
use std::collections::{HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day21").unwrap();
    let time = Instant::now();

    let grid = text
        .lines()
        .map(|line| {
            line
                .bytes()
                .map(|b| b.into())
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let start_y = grid.len() / 2;
    let start_x = grid[0].len() / 2;

    let p2_steps = 26501365;
    let size = grid.len();


    // basically means that we can only reach half way into some of the 
    // repeating grids
    assert_eq!(p2_steps % size, size / 2);
    let repeated_grid_width = (p2_steps / size) - 1;

    // the number of odd repitions
    let odd = ((repeated_grid_width / 2) * 2 + 1).pow(2);
    // the number of even repitions
    let even = (((repeated_grid_width + 1) / 2) * 2).pow(2);

    // how many valid spots there are in the even and odd grids
    let odd_steps = bfs_fill(start_x as i32, start_y as i32, size * 2 + 1, &grid);
    let even_steps = bfs_fill(start_x as i32, start_y as i32, size * 2, &grid);

    // this is all the corners that will look like
    // |----|
    // | /\ |
    // |/  \|
    // |    |
    // ------
    let top = bfs_fill(start_x as i32, (size - 1) as i32, size - 1, &grid);
    let right = bfs_fill(0, start_y as i32, size - 1, &grid);
    let bottom = bfs_fill(start_x as i32, 0, size - 1, &grid);
    let left = bfs_fill((size - 1) as i32, start_y as i32, size - 1, &grid);

    // these are the smaller segments along the diaganals
    let small_segment_steps = size / 2 - 1;
    let small_tr = bfs_fill(0, (size - 1) as i32, small_segment_steps, &grid);
    let small_tl = bfs_fill((size - 1) as i32, (size - 1) as i32, small_segment_steps, &grid);
    let small_br = bfs_fill(0, 0, small_segment_steps, &grid);
    let small_bl = bfs_fill((size - 1) as i32, 0, small_segment_steps, &grid);

    // the larger segment along the diaganal
    let large_segement_steps = (3 * size) / 2 - 1;
    let large_tr = bfs_fill(0, (size - 1) as i32, large_segement_steps, &grid);
    let large_tl = bfs_fill((size - 1) as i32, (size - 1) as i32, large_segement_steps, &grid);
    let large_br = bfs_fill(0, 0, large_segement_steps, &grid);
    let large_bl = bfs_fill((size - 1) as i32, 0, large_segement_steps, &grid);

    let combined_points = odd_steps * odd + 
        even_steps * even +
        top + left + bottom + right +
        ((repeated_grid_width + 1) * (small_tr + small_tl + small_br + small_bl)) +
        (repeated_grid_width * (large_tr + large_tl + large_br + large_bl));


    let sol1: u64 = bfs_fill(start_x as i32, start_y as i32, 64, &grid) as u64;
    let sol2: u64 = combined_points as u64;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn bfs_fill(start_x: i32, start_y: i32, step_count: usize, grid: &[Vec<Tile>]) -> usize {
    let mut queue: VecDeque<(i32, i32, usize)> = VecDeque::new();
    queue.push_back((start_x as i32, start_y as i32, step_count));


    let mut final_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert((start_x as i32, start_y as i32));

    while let Some((x, y, step_count)) = queue.pop_front() {
        // we are allowed to move back and forth so if its even we can just
        // come back here later and this is a valid final pos
        if step_count % 2 == 0 {
            final_positions.insert((x, y));
        }

        if step_count == 0 {
            continue;
        }


        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if x + dx < 0
                || x + dx >= grid[0].len() as i32
                || y + dy < 0
                || y + dy >= grid.len() as i32
                || !seen.insert((x + dx, y + dy))
                || grid[(y+dy) as usize][(x+dx) as usize] == Tile::Rock {
                continue;
            }

            queue.push_back((x+dx, y+dy, step_count - 1));
        }
    }

    final_positions.len()
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Garden,
            b'#' => Self::Rock,
            _ => Self::Garden,
        }
    }
}