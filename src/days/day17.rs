use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

type Board = [[bool; 7]; 8000];
type Block = Vec<(i32, i32)>;

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day17.txt").unwrap();
    let time = Instant::now();
    let directions = text.chars().map(|c| c).collect::<Vec<char>>();
    let d2 = text.chars().map(|c| c).collect::<Vec<char>>();
    let test_stuff = text.chars().map(|c| c).collect::<Vec<char>>();
    test_visualizer(test_stuff);

    let sol1: i32 = p1(directions);
    let sol2: i64 = p2(d2);

    let solution = (SolutionType::I32(sol1), SolutionType::I64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn p2(directions: Vec<char>) -> i64 {
    let pecies = [
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut board = [[false; 7]; 8000];
    let mut placed = 0;
    let mut height = 0;
    let mut dir_i = 0;

    0
}

fn p1(directions: Vec<char>) -> i32 {
    let pecies = [
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut board = [[false; 7]; 8000];
    let mut placed = 0;
    let mut height = 0;
    let mut dir_i = 0;
    while placed < 2022 {
        let piece = &pecies[(placed % 5) as usize];
        let mut piece_row = height + 3;
        let mut piece_col = 2;
        loop {
            if directions[dir_i] == '<' && try_pos(piece_row, piece_col - 1, piece, &board) {
                piece_col -= 1;
            }
            if directions[dir_i] == '>' && try_pos(piece_row, piece_col + 1, piece, &board) {
                piece_col += 1;
            }

            if try_pos(piece_row - 1, piece_col, piece, &board) {
                piece_row -= 1;
            } else {
                height = height.max(get_piece_height(piece, piece_row));
                set_pos(piece_row, piece_col, piece, &mut board);
                placed += 1;
                dir_i += 1;
                dir_i = dir_i % directions.len();
                break;
            }
            dir_i += 1;
            dir_i = dir_i % directions.len();
        }
    }
    height
}

fn test_visualizer(directions: Vec<char>) {
    let pecies = [
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    let mut board = [[false; 7]; 8000];
    let mut placed = 0;
    let mut height = 0;
    let mut last_cycle_heigt = 0;
    let mut dir_i = 0;
    while placed < 5000 {
        if placed % 5 == 0 {
            let z = height - last_cycle_heigt;
            last_cycle_heigt = height;
            println!("{}", z);
        }
        let piece = &pecies[(placed % 5) as usize];
        let mut piece_row = height + 3;
        let mut piece_col = 2;
        loop {
            if directions[dir_i] == '<' && try_pos(piece_row, piece_col - 1, piece, &board) {
                piece_col -= 1;
            }
            if directions[dir_i] == '>' && try_pos(piece_row, piece_col + 1, piece, &board) {
                piece_col += 1;
            }

            if try_pos(piece_row - 1, piece_col, piece, &board) {
                piece_row -= 1;
            } else {
                height = height.max(get_piece_height(piece, piece_row));
                set_pos(piece_row, piece_col, piece, &mut board);
                placed += 1;
                dir_i += 1;
                dir_i = dir_i % directions.len();
                break;
            }
            dir_i += 1;
            dir_i = dir_i % directions.len();
        }
    }
}

fn pretty_print(board: &Board) {
    let mut string = String::new();
    for y in (0..3000).rev() {
        for x in 0..7 {
            if board[y][x] {
                string.push('#');
            } else {
                string.push('.');
            }
        }
        string.push('\n');
    }
    println!("{}", string);
}

fn set_pos(row: i32, col: i32, piece: &Block, board: &mut Board) {
    for point in piece.iter() {
        board[(point.1 + row) as usize][(point.0 + col) as usize] = true;
    }
}

fn get_piece_height(piece: &Block, row: i32) -> i32 {
    let i = piece.iter().map(|p| p.1 + row).max().unwrap();
    i + 1
}

fn try_pos(row: i32, col: i32, piece: &Block, board: &Board) -> bool {
    for point in piece.iter() {
        if point.0 + col < 0 || point.0 + col > 6 || point.1 + row < 0 || board[(point.1 + row) as usize][(point.0 + col) as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTINPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn peice_height() {
        let pecies = [
            vec![(0,0), (1,0), (2,0), (3,0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        ];
        assert_eq!(get_piece_height(&pecies[1], 0), 3);
    }

    #[test]
    fn part_1() {
        let directions = TESTINPUT.chars().map(|c| c).collect::<Vec<char>>();
        assert_eq!(p1(directions), 3068);
    }
}