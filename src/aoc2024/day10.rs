use crate::Solve;
use std::{collections::{HashSet, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
//     let data_in = "89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732";

    let time = Instant::now();
    
    // assumes square input
    let map_size = (data_in.len() as f64).sqrt() as usize;
    
    let map = data_in.lines().map(|l| {
        l.bytes().map(|b| byte_char_to_num(b)).collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();
    
    let mut ends = HashSet::new();
    for y in 0..map_size {
        for x in 0..map_size {
            if map[y][x] == 0 {
                let mut queue = VecDeque::new();
                let mut nines_seen = 0;

                queue.push_back((x as i32, y as i32));

                while !queue.is_empty() {
                    let (search_x, search_y) = queue.pop_front().unwrap();
                    for (dx, dy) in [(1,0), (0,1), (-1, 0), (0, -1)] {
                        if search_x + dx >= 0 && search_x + dx < map_size as i32
                            && search_y + dy >= 0 && search_y + dy < map_size as i32
                            && map[(search_y + dy) as usize][(search_x + dx) as usize] as i16 - map[search_y as usize][search_x as usize] as i16 == 1 
                        {
                            if map[(search_y + dy) as usize][(search_x + dx) as usize] == 9 {
                                ends.insert((search_x + dx, search_y + dy));
                            }

                            queue.push_back((search_x + dx, search_y + dy));
                        }
                    }
                }
            }
        }
    }
    

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(ends.len()),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let map_size = (data_in.len() as f64).sqrt() as usize;
    
    let map = data_in.lines().map(|l| {
        l.bytes().map(|b| byte_char_to_num(b)).collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();
    
    
    let mut solve = 0; 
    for y in 0..map_size {
        for x in 0..map_size {
            if map[y][x] == 0 {
                let mut stack = Vec::new();
                stack.push((x as i32, y as i32));

                while !stack.is_empty() {
                    let (search_x, search_y) = stack.pop().unwrap();
                    for (dx, dy) in [(1,0), (0,1), (-1, 0), (0, -1)] {
                        if search_x + dx >= 0 && search_x + dx < map_size as i32
                            && search_y + dy >= 0 && search_y + dy < map_size as i32
                            && map[(search_y + dy) as usize][(search_x + dx) as usize] as i16 - map[search_y as usize][search_x as usize] as i16 == 1 
                        {
                            if map[(search_y + dy) as usize][(search_x + dx) as usize] == 9 {
                                solve += 1;
                            }

                            stack.push((search_x + dx, search_y + dy));
                        }
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

fn byte_char_to_num(byte: u8) -> u8 {
    match byte {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'.' => 200,
        _ => unreachable!(),
    }
}