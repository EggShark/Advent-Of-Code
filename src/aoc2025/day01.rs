use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut pos = 50;
    for line in data_in.lines() {
        let i = line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => {
               pos = (pos - i).rem_euclid(100);
            },
            "R" => {
                pos = (pos + i).rem_euclid(100);
            }
            _ => unreachable!() 
        }

        if pos == 0 {
            solve += 1;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut pos = 50;
    for line in data_in.lines() {
        let i = line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => {
                solve += i/100;
                println!("{}", pos - i);
                if pos != 0 && i % 100 >= pos {
                    println!("count down");
                    solve += 1
                    
                }
                pos = (pos - i).rem_euclid(100);
            },
            "R" => {
                println!("{}", pos + i);
                if pos + i >= 100 {
                    println!("count up");
                    solve += (pos + i)/100;
                }
                pos = (pos + i).rem_euclid(100);
            }
            _ => unreachable!() 
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
