use crate::{Solution, SolutionType};
use std::collections::{VecDeque, HashSet};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day12.txt").unwrap();
    let time = Instant::now();

    let mut start = Point{x:0,y:0};
    let mut end  = Point{x: 0,y:0};

    let mut x: usize = 0;
    let mut y: usize = 0;
    //rember is [y][x];
    let mut map: [[i16; 144]; 41] = [[0; 144]; 41];
    for line in text.lines() {
        for char in line.chars() {
            if char == 'S' {
                map[y][x] = 0;
                start = Point{x: x as i16, y: y as i16};
            } else if char == 'E' {
                end = Point{x: x as i16, y: y as i16};
                map[y][x] = 25;
            } else {
                map[y][x] = ALPHABET.find(char).unwrap() as i16;
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    let sol1: i32 = bfs(&map, start, end).unwrap();

    //let mut sol2 = 0;
    // for (y, row) in map.iter().enumerate() {
    //     for (x, height) in row.iter().enumerate() {
    //         if *height == 0 {
    //             if let Some(x) = bfs(&map, Point{x: x as i16,y: y as i16}, end) {
    //                 if sol2 > x {
    //                     sol2 = x;
    //                 }
    //             }
    //         }
    //     }
    // }

    let sol2: i32 = bfs_p2(&map, end).unwrap();

    let solution = (SolutionType::I32(sol1), SolutionType::I32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn bfs(grid: &[[i16; 144]; 41], start: Point, end: Point) -> Option<i32> {
    let mut que = VecDeque::new();
    let mut visted = HashSet::new();
    que.push_back((0, start));
    visted.insert(start);
    while let Some((distance, point)) = que.pop_front() {
        // gets all the neighborhs
        for new_point in [Point{x: point.x + 1, y: point.y}, Point{x: point.x - 1, y: point.y}, Point{x: point.x, y: point.y + 1}, Point{x:point.x, y: point.y - 1}] {
            let index_x = new_point.x as usize;
            let index_y = new_point.y as usize;
            if new_point.x >= 0 // makes sure the point is in range
            && new_point.y >= 0 // same here
            && index_y < grid.len() // more in range checks
            && index_x < grid[index_y].len() // yairc
            && grid[index_y][index_x] <= grid[point.y as usize][point.x as usize] + 1 // checks if the we are allowed to walk to the neighboor
            && visted.insert(new_point) { // checks to see if we have already been here
                if new_point == end {
                    return Some(distance + 1);
                }
                que.push_back((distance + 1, new_point));
            }
        }
    }

    None
}

fn bfs_p2(&grid: &[[i16; 144]; 41], start: Point) -> Option<i32> {
    let mut que = VecDeque::new();
    let mut visted = HashSet::new();
    que.push_back((0, start));
    visted.insert(start);

    while let Some((distance, point)) = que.pop_front() {
        // gets all the neighborhs
        for new_point in [Point{x: point.x + 1, y: point.y}, Point{x: point.x - 1, y: point.y}, Point{x: point.x, y: point.y + 1}, Point{x:point.x, y: point.y - 1}] {
            let index_x = new_point.x as usize;
            let index_y = new_point.y as usize;
            if new_point.x >= 0 // makes sure the point is in range
            && new_point.y >= 0 // same here
            && index_y < grid.len() // more in range checks
            && index_x < grid[index_y].len() // yairc
            && grid[index_y][index_x] >= grid[point.y as usize][point.x as usize] - 1  // checks if the we are allowed to walk to the neighboor
            && visted.insert(new_point) { // checks to see if we have already been here
                if grid[index_y][index_x] == 0 {
                    return Some(distance + 1);
                }
                que.push_back((distance + 1, new_point));
            }
        }
    }

    None
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i16,
    y: i16,
}