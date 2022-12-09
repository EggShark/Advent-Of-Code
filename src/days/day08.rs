use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day08.txt").unwrap();
    let time = Instant::now();

    let mut trees = [[0_u8; 99]; 99];

    let mut x_counter = 0;
    let mut y_counter = 0;
    for line in text.lines() {
        let mut row = [0; 99]; // y contains x
        for char in line.chars() {
            let num = char.to_string().parse::<u8>().unwrap();
            row[x_counter] = num;
            x_counter += 1;
        }
        x_counter = 0;
        trees[y_counter] = row;
        y_counter += 1;
    }

    let mut counter = 0;
    for y_pos in 0..99 {
        for x_pos in 0..99 {
            if is_visble((x_pos, y_pos), &trees) {
                counter += 1;
            }
        }
    }
    
    let mut higest = 0;
    for y_pos in 0..99 {
        for x_pos in 0..99 {
            let score = calculate_senic_score((x_pos, y_pos), &trees);
            if score > higest{
                higest = score
            }
        }
    }

    let sol1: u32 = counter;
    let sol2: u32 = higest;

    let solution = (SolutionType::U32(sol1), SolutionType::U32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn calculate_senic_score(point: (usize, usize), list: &[[u8; 99]; 99]) -> u32 {
    get_senic_above(point, list) * get_senic_below(point, list) * get_senic_left(point, list) * get_senic_right(point, list)
}

fn get_senic_left((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> u32 {
    let mut counter = 0;
    for num in (0..x).rev() {
        counter += 1;
        if list[y][x] <= list[y][num] {
            return counter;
        }
    }
    counter
}

fn get_senic_right((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> u32 {
    let mut counter = 0;
    for num in x+1..99 {
        counter += 1;
        if list[y][x] <= list[y][num] {
            return counter;
        }
    }
    counter
}

fn get_senic_above((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> u32 {
    let mut counter = 0;
    for num in (0..y).rev() {
        counter += 1; 
        if list[y][x] <= list[num][x] {
            return counter;
        }        
    }
    counter
}

fn get_senic_below((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> u32 {
    let mut counter = 0;
    for num in y+1..99 {
        counter += 1;
        if list[y][x] <= list[num][x] {
            return counter;
        }
    }
    counter
}

// DO NOT TOUCH
fn is_visble(point: (usize, usize), list: &[[u8; 99]; 99]) -> bool {
    if check_above(point, list) {
        return true;
    }
    if check_to_left(point, list) {
        return true;
    }
    if check_below(point, list) {
        return true;
    }
    if check_to_right(point, list) {
        return true;
    }
    false
}

fn check_to_left((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> bool {
    for num in 0..x {
        if list[y][x] <= list[y][num] {
            return false;
        }
    }
    true
}

fn check_to_right((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> bool {
    for num in x+1..99 {
        if list[y][x] <= list[y][num] {
            return false;
        }
    }
    true
}

fn check_above((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> bool {
    for num in 0..y {
        if list[y][x] <= list[num][x] {
            return false;
        }
    }
    true
}

fn check_below((x,y): (usize, usize), list: &[[u8; 99]; 99]) -> bool {
    for num in y+1..99 {
        if list[y][x] <= list[num][x] {
            return false;
        }
    }
    true
}