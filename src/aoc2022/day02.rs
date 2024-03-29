use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day02.txt").unwrap();
    let time = Instant::now();

    let moves: Vec<String> = text.lines().map(|l| l.to_string()).collect();

    let mut score = 0;

    for line in &moves {
        let my_move = &line[2..3];
        let opponent_move = &line[0..1];
        if my_move == "Y" {
            score += 2; // paper
        } 
        if my_move == "X" {
            score += 1; // rock
        }
        if my_move == "Z" {
            score += 3; // siccors
        }
    
        if is_win(my_move, opponent_move) {
            score += 6;
        } else if is_draw(my_move, opponent_move) {
            //draw
            score += 3;
        }
    }

    let mut score2 = 0;

    for line in moves {
        let opponent_move = &line[0..1];
        let my_choice = &line[2..3];
        let outcome: Outcome = {
            if my_choice == "Y" {
                Outcome::Draw
            } else if my_choice == "Z" {
                Outcome::Win
            } else {
                Outcome::Loose
            }
        };

        score2 += find_shape(&outcome, opponent_move);

        match outcome {
            Outcome::Win => {score2 += 6},
            Outcome::Draw => {score2 += 3},
            _ => {},
        }
    }

    let sol1: u64 = score;
    let sol2: u64 = score2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;

    Solution {
        solution,
        time_ms,
    }
}

fn is_win(my_move: &str, opponent_move: &str) -> bool {
    if opponent_move == "A" && my_move == "Y" {
        return true;
    }
    if opponent_move == "B" && my_move == "Z" {
        return true;
    }
    if opponent_move == "C" && my_move == "X" {
        return true;
    }

    false
}

fn is_draw(my_move: &str, opponent_move: &str) -> bool {
    if opponent_move == "A" && my_move == "X" {
        return true;
    }
    if opponent_move == "B" && my_move == "Y" {
        return true;
    }
    if opponent_move == "C" && my_move == "Z" {
        return true;
    }

    false
}

fn find_shape(option: &Outcome, opponent_move: &str) -> u64 {
    match option {
        Outcome::Win => {
            if opponent_move == "A" {
                2
            } else if opponent_move == "B" {
                3
            } else {
                1
            }
        },
        Outcome::Draw => {
            if opponent_move == "A" {
                1
            } else if opponent_move == "B" {
                2
            } else {
                3
            }
        },
        Outcome::Loose => {
            if opponent_move == "A" {
                3
            } else if opponent_move == "B" {
                1
            } else {
                2
            }
        }
    }
}
enum Outcome {
    Win,
    Draw,
    Loose,
}
