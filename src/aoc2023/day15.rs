use crate::{Solution, SolutionType};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day15").unwrap();
    let time = Instant::now();

    let p1 = text
        .split(',')
        .map(|s| {
            let mut val: u32 = 0;
            for b in s.bytes() {
                val += b as u32;
                val *= 17;
                val %= 256;
            }

            val
        }).sum::<u32>();

    let mut map: HashMap<u32, Vec<(Vec<u8>, u8)>> = HashMap::new();

    text
        .split(',')
        .for_each(|s| {
            let mut key: u32 = 0;
            let mut bytes = s.bytes();

            let mut name = Vec::new();
            let mut b = bytes.next().unwrap();
            while b.is_ascii_alphabetic() {
                name.push(b);
                key += b as u32;
                key *= 17;
                key %= 256;
                b = bytes.next().unwrap();
            }

            match b {
                b'=' => {
                    let lens_box = map.get_mut(&key);
                    match lens_box {
                        Some(l_box) => {
                            let value = bytes.next().unwrap() as char;
                            let x = l_box
                                .iter()
                                .enumerate()
                                .find(|(_, (n, _))| n == &name);
                            match x {
                                Some((idx, _)) => l_box[idx] = (name, value.to_digit(10).unwrap() as u8),
                                None => l_box.push((name, value.to_digit(10).unwrap() as u8))
                            }
                        },
                        None => {
                            let mut l_box = Vec::new();
                            let value = bytes.next().unwrap() as char;
                            l_box.push((name, value.to_digit(10).unwrap() as u8));
                            map.insert(key, l_box);
                        }
                    }
                },
                b'-' => {
                    let l_box = map.get_mut(&key);
                    match l_box {
                        None => {},
                        Some(l_box) => {
                            let x = l_box
                                .iter()
                                .enumerate()
                                .find(|(_, (n, _))| n == &name);
                            match x {
                                Some((idx, _)) => {l_box.remove(idx);},
                                None => {},
                            };
                        }
                    };

                },
                _ => unreachable!(),
            }
        });

    let mut p2 = 0;
    for list in map {
        for idx in 0..list.1.len() {
           p2 += (list.0 + 1) * (idx + 1) as u32 * list.1[idx].1 as u32;
        }
    }



    let sol1: u64 = p1 as u64;
    let sol2: u64 = p2 as u64;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}