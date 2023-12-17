use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
///////////////////////////////////////////////////////////////////////////////

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day17").unwrap();
    let time = Instant::now();

    let grid = text
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();


    let sol1: u64 = dijkstra(&grid, 3, 0) as u64;
    let sol2: u64 = dijkstra(&grid, 10, 4) as u64;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn dijkstra(grid: &[Vec<u32>], max_forward: u32, min_forward: u32) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    heap.push(Reverse(State::default()));

    while heap.len() > 0 {
        let current_position = heap.pop().unwrap().0;

        if current_position.y == grid.len() as i32 - 1
            && current_position.x == grid[0].len() as i32 - 1
            && current_position.times_in_direction >= min_forward {
            return current_position.heat_loss;
        }

        let hs: HashState = current_position.into();
        if !seen.insert(hs) {
            continue;
        }

        // checks if we are allowed to go forawrd and that we arent standing still
        // as if we are still this code does nothing
        if current_position.times_in_direction < max_forward && (current_position.delta_x, current_position.delta_y) != (0, 0) {
            let new_x = current_position.x + current_position.delta_x;
            let new_y = current_position.y + current_position.delta_y;
            if new_x >= 0 && new_x < grid[0].len() as i32 && new_y >= 0 && new_y < grid.len() as i32 {
                let new_state = State {
                    heat_loss: current_position.heat_loss + grid[new_y as usize][new_x as usize],
                    x: new_x,
                    y: new_y,
                    delta_x: current_position.delta_x,
                    delta_y: current_position.delta_y,
                    times_in_direction: current_position.times_in_direction + 1, 
                };
                heap.push(Reverse(new_state));
            }
        }

        // checks if we are allowed to turn the attempts to turn and we need to pick a direction/turn on the first turn so
        // thats why we have the or clause
        if current_position.times_in_direction >= min_forward || (0, 0) == (current_position.delta_x, current_position.delta_y) {
            for d in DIRECTIONS {
                if d != (current_position.delta_x, current_position.delta_y) 
                    && d != (-current_position.delta_x, -current_position.delta_y) {
                    let new_x = d.0 + current_position.x;
                    let new_y = d.1 + current_position.y;
                    if new_x >= 0 && new_x < grid[0].len() as i32 && new_y >= 0 && new_y < grid.len() as i32 {
                        let new_state = State {
                            heat_loss: current_position.heat_loss + grid[new_y as usize][new_x as usize],
                            x: new_x,
                            y: new_y,
                            times_in_direction: 1,
                            delta_x: d.0,
                            delta_y: d.1,
                        };
                        heap.push(Reverse(new_state));
                    }
                }
            }
        }
    }

    0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord)]
struct State {
    heat_loss: u32,
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
    times_in_direction: u32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss))
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            heat_loss: 0,
            x: 0,
            y: 0,
            delta_x: 0,
            delta_y: 0,
            times_in_direction: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct HashState {
    x: i32,
    y: i32,
    delta_x: i32,
    delta_y: i32,
    times_in_direction: u32,
}

impl From<State> for HashState {
    fn from(value: State) -> Self {
        Self {
            x: value.x,
            y: value.y,
            delta_x: value.delta_x,
            delta_y: value.delta_y,
            times_in_direction: value.times_in_direction,
        }
    }
}