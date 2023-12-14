use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

#[cfg(target_os="windows")]
const DOUBLE_NEW_LINE: &str = "\n\r\n\r";

#[cfg(not(target_os="windows"))]
const DOUBLE_NEW_LINE: &str = "\n\n";

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day13").unwrap();
    let time = Instant::now();

    let patterns = text
        .split(DOUBLE_NEW_LINE)
        .map(|section| {
            let pat = section
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            Pattern {pat}
        })
        .map(|p| p.find_reflection())
        .sum::<u64>();

    // 37692
    let p2 = text
        .split(DOUBLE_NEW_LINE)
        .map(|section| {
            let pat = section
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            Pattern {pat}
        })
        .map(|p| p.find_smudges())
        .sum::<u64>();
    


    let sol1: u64 = patterns;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug)]
struct Pattern {
    pat: Vec<Vec<char>>,
}

impl Pattern {
    fn find_reflection(&self) -> u64 {
        let horz = self.find_horizontal();

        if let Some(midpoint) = horz {
            100 * (midpoint + 1) as u64
        } else {
            let mid_point = self.find_vertical().unwrap();
            mid_point as u64 + 1
        }
    }

    fn find_smudges(&self) -> u64 {
        let h_smudge = self.find_horizontal_smudge();
        if let Some(midpoint) = h_smudge {
            midpoint as u64
        } else {
            let mid_point = self.find_vetical_smudge().unwrap();
            mid_point as u64
        }
    }

    fn find_horizontal_smudge(&self) -> Option<usize> {
        'rows: for idx in 0..self.pat.len() - 1 {
            let mut diff = self.row_diff(idx, idx + 1);
            if diff <= 1 {
                let min_distance_to_edge = idx.min(self.pat.len() - idx - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.row_diff(idx - d, idx + d + 1);
                    if diff > 1 {
                        continue 'rows;
                    }
                }

                if diff == 0 {
                    continue 'rows;
                }
                return Some((idx + 1) * 100);
            }
        }

        None
    }

    fn row_diff(&self, y1: usize, y2: usize) -> u64 {
        let mut diff = 0;
        for x in 0..self.pat[0].len() {
            if self.pat[y1][x] != self.pat[y2][x] {
                diff += 1;
            }
        }
        diff
    }

    fn col_diff(&self, x1: usize, x2: usize) -> u64 {
        let mut diff = 0;
        for y in 0..self.pat.len() {
            if self.pat[y][x1] != self.pat[y][x2] {
                diff += 1;
            }
        }
        diff
    }

    fn find_vetical_smudge(&self) -> Option<usize> {
        'columns: for idx in 0..self.pat[0].len() - 1 {
            let mut diff = self.col_diff(idx, idx + 1);
            if diff <= 1 {
                let min_distance_to_edge = idx.min(self.pat[0].len() - idx - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.col_diff(idx - d, idx + d + 1);
                    if diff > 1 {
                        continue 'columns;
                    }
                }
                if diff == 0 {
                    continue 'columns;
                }
                return Some(idx + 1);
            }
        }

        None
    }

    fn find_horizontal(&self) -> Option<usize> {
        for idx in 0..self.pat.len()-1 {
            if self.pat[idx] == self.pat[idx+1] {
                // actually check
                let mut left = idx;
                let mut right = idx + 1;
                let mut real = true;
                while left > 0 && right < self.pat.len()-1 {
                    left -= 1;
                    right += 1;
                    if self.pat[left] != self.pat[right] {
                        real = false;
                        break;
                    }
                }

                if real {
                    return Some(idx);
                }
            }
        }

        None
    }

    fn find_vertical(&self) -> Option<usize> {
        for x in 0..self.pat[0].len() - 1 {
            let mut whole_col = true;
            let x2 = x + 1;
            for y in 0..self.pat.len() {
                if self.pat[y][x] != self.pat[y][x2] {
                    whole_col = false;
                    break;
                }
            }

            if whole_col {
                let mut left = x;
                let mut right = x + 1;
                let mut real = true;
                'a: while left > 0 && right < self.pat[0].len()-1 {
                    left -= 1;
                    right += 1;
                    for y in 0..self.pat.len() {
                        if self.pat[y][left] != self.pat[y][right] {
                            real = false;
                            break 'a;
                        }
                    }
                }
                if real {
                    return Some(x);
                }
            }
        }
        None
    }
}