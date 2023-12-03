use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::iter::successors;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day3").unwrap();
    let time = Instant::now();

    let mut sum = 0;

    let char_array: Vec<Vec<char>> = text
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let grid_hieght = char_array.len();
    let grid_width = char_array[0].len();

    let mut nums_adj_to_symbols: Vec<Vec<Vec<u32>>> = vec![vec![vec![]; grid_width]; grid_hieght];

    for row in 0..grid_hieght {
        let mut col = 0;
        while col < grid_width {
            if char_array[row][col].is_digit(10) {
                let mut number = 0;
                let start_col = col;

                while col < grid_width && char_array[row][col].is_digit(10) {
                    number *= 10;
                    number += char_array[row][col].to_digit(10).unwrap();
                    col += 1;
                }

                let end_col = col-1;

                let mut is_part_id = false;
                'inner: for sub_col in (start_col as isize - 1)..=(end_col as isize + 1) {
                    for sub_row in (row as isize-1)..=(row as isize + 1) {
                        if (sub_col >= 0 && sub_col < grid_width as isize && sub_row >= 0 && sub_row < grid_hieght as isize)
                            && symbols(char_array[sub_row as usize][sub_col as usize]) {
                            is_part_id = true;
                            nums_adj_to_symbols[sub_row as usize][sub_col as usize].push(number);
                            break 'inner;
                        }
                    }
                }

                if is_part_id {
                    sum += number as u64;
                }
            } else {
                col += 1;
            }
        }
    }

    let mut sum2 = 0;
    (0..grid_hieght).for_each(|row| {
        (0..grid_width).for_each(|col| {
            if char_array[row][col] == '*' && nums_adj_to_symbols[row][col].len() == 2 {
                sum2 += (nums_adj_to_symbols[row][col][0] * nums_adj_to_symbols[row][col][1]) as u64
            }
        })
    });

    let sol1: u64 = sum;
    let sol2: u64 = sum2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}


fn symbols(string: char) -> bool {
    string != '.' && !string.is_numeric()
}

fn numbers_of_digits(number: u64) -> usize {
    successors(Some(number), |&number| (number >= 10).then(|| number/10)).count()
}