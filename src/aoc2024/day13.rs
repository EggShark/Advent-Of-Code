use crate::{Solve, DOUBLE_NEW_LINE};
use std::{str::FromStr, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let buttons: i64 = data_in.split(DOUBLE_NEW_LINE).map(|s| Machine::from_str(s).unwrap())
    .map(|b| {
        // determinate of original matrix
        // [x1, x2]
        // [y1, y2]
        let det = b.button_one.0 * b.button_two.1 - b.button_two.0 * b.button_one.1;
        // determinate to solve for c1
        // [px, x2]
        // [py, y2]
        let d1 = b.prize.0 * b.button_two.1 - b.button_two.0 * b.prize.1;
        // determinate to solve for c2
        // [x1, px]
        // [y1, py]
        let d2 = b.button_one.0 * b.prize.1 - b.prize.0 * b.button_one.1;

        // checks for positive integer answer
        if d1 % det == 0 && d2 % det == 0 && d1/det >= 0 && d2/det >= 0 {
            // cramers rule says to get nth unknown we can divide the determinate
            // where the nth colloum is replaced by prize matrix       
            3 * (d1 / det) + (d2 / det)
        } else {
            0
        }
    }).sum();



    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(buttons),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let buttons: i64 = data_in.split(DOUBLE_NEW_LINE).map(|s| Machine::from_str(s).unwrap())
    .map(|b| {
        let prize = (b.prize.0 + 10000000000000, b.prize.1 + 10000000000000);
        let det = b.button_one.0 * b.button_two.1 - b.button_two.0 * b.button_one.1;
        let d1 = prize.0 * b.button_two.1 - b.button_two.0 * prize.1;
        let d2 = b.button_one.0 * prize.1 - prize.0 * b.button_one.1;

        if d1 % det == 0 && d2 % det == 0 && d1/det >= 0 && d2/det >= 0 {
            3 * (d1 / det) + (d2 / det)
        }else {
            0
        }
    }).sum();


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(buttons),
        time_ms,
    }
}

#[derive(Debug)]
struct Machine {
    button_one: (i64, i64),
    button_two: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let b1 = lines.next().unwrap();
        let (x, y) = b1.split_once(", ").unwrap();
        let x = x.split_once("+").unwrap().1.parse::<i64>().unwrap();
        let y = y.split_once("+").unwrap().1.parse::<i64>().unwrap();
        let button_one = (x, y);

        let b2 = lines.next().unwrap();
        let (x, y) = b2.split_once(", ").unwrap();
        let x = x.split_once("+").unwrap().1.parse::<i64>().unwrap();
        let y = y.split_once("+").unwrap().1.parse::<i64>().unwrap();
        let button_two = (x, y);

        let prize = lines.next().unwrap();
        let (x, y) = prize.split_once(", ").unwrap();
        let x = x.split_once("=").unwrap().1.parse::<i64>().unwrap();
        let y = y.split_once("=").unwrap().1.parse::<i64>().unwrap();
        let prize = (x, y);

        Ok(Self {
            button_one,
            button_two,
            prize,
        })
    }
}