use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = read_to_string("./texts/day03.txt").unwrap();

    let x: u32 = input.split("\n").collect::<Vec<&str>>()
        .iter()
        .map(|g| {
            let g = g.to_string();
            let (c1, c2) = g.split_at(g.len()/2);
            let mut priority = 0;
            for c in c1.chars() {
                if c2.contains(c) {
                    priority += get_prority(c);
                    break;
                }
            }
            priority
        }).sum();

        let mut group_vec: Vec<&str> = Vec::new();
        let mut counter = 0;
        for line in input.lines() {
            if group_vec.len() != 3 {
                group_vec.push(line);
            } else {
                for c in group_vec[0].chars() {
                    if group_vec[1].contains(c) && group_vec[2].contains(c) {
                        counter += get_prority(c);
                        break;
                    }
                }
                group_vec = Vec::new();
                group_vec.push(line);
            }
        }
        for c in group_vec[0].chars() {
            if group_vec[1].contains(c) && group_vec[2].contains(c) {
                counter += get_prority(c);
                break;
            }
        }
        

    let sol1: u32 = x;
    let sol2: u32 = counter;

    (Solution::U32(sol1), Solution::U32(sol2))
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_prority(c: char) -> u32 {
    let mut pos = 0;
    for a in ALPHABET.chars() {
        pos += 1;
        if a == c {
            break
        }
    }
    pos
}