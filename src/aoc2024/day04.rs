use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day4").unwrap();
    let time = Instant::now();

    let mut cards: Vec<Cards> = text.lines()
        .map(|line: &str| {
            let (_, right) = line.split_once(":").unwrap();
            let (left, right) = right.split_once("|").unwrap();
            let card: Vec<u64> = left
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|num| {
                    num.parse::<u64>().unwrap()
                })
                .collect();
            let winning_numbers: Vec<u64> = right
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|f| f.parse::<u64>().unwrap())
                .collect();
            Cards::new(&card, &winning_numbers)
        }).collect::<Vec<Cards>>();

        let p1: u64 = cards.iter().map(|card| {
            let mut score = 0;
            if card.wins > 0 {
                score += 1;
            }
            (1..card.wins).for_each(|_| {
                score *= 2;
            });
            score
        }).sum();

        for idx in 0..cards.len() {
            let wins = cards[idx].wins;
            for sub_idx in 0..wins as usize {
                let add_to = (idx + sub_idx) + 1;
                if add_to < cards.len() {
                    let amount_to_add = cards[idx].coppies;
                    cards[add_to].coppies += amount_to_add;
                }
            }
        }

        let p2: u32 = cards.iter().map(|card| card.coppies).sum();


    let sol1: u64 = p1;
    let sol2: u64 = p2 as u64;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

struct Cards {
    coppies: u32,
    wins: u32,
}

impl Cards {
    pub fn new(numbers: &[u64], winners: &[u64]) -> Self {
        let wins = numbers.iter().filter(|num| winners.contains(num)).count() as u32;
        Self {
            coppies: 1,
            wins,
        }
    }
}