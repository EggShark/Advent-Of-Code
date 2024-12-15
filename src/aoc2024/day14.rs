use crate::Solve;
use std::{i32, time::Instant};
///////////////////////////////////////////////////////////////////////////////

const WIDTH: i8 = 101;
const HIEGHT: i8 = 103;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut bots = data_in.lines().map(|l| Bot::from_line(l)).collect::<Vec<_>>();

    for bot in bots.iter_mut() {
        for _ in 0..100 {
            bot.pos.0 = (bot.pos.0 + bot.v.0).rem_euclid(WIDTH);
            bot.pos.1 = (bot.pos.1 + bot.v.1).rem_euclid(HIEGHT);
        }
    }

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut qaud3 = 0;
    let mut quad4 = 0;
    for bot in bots {
        if (0..WIDTH/2).contains(&bot.pos.0) && (0..HIEGHT/2).contains(&bot.pos.1) {
            quad1 += 1;
        } else if ((WIDTH/2 + 1)..WIDTH).contains(&bot.pos.0) && (0..HIEGHT/2).contains(&bot.pos.1) {
            quad2 += 1;
        } else if (0..WIDTH/2).contains(&bot.pos.0) && ((HIEGHT/2 + 1)..HIEGHT).contains(&bot.pos.1) {
            qaud3 += 1;
        } else if ((WIDTH/2 + 1)..WIDTH).contains(&bot.pos.0) && ((HIEGHT/2 + 1)..HIEGHT).contains(&bot.pos.1) {
            quad4 += 1;
        }
    }
    
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(quad1 * quad2 * qaud3 * quad4),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut bots = data_in.lines().map(|l| Bot::from_line(l)).collect::<Vec<_>>();
    let bc = bots.len();

    let mut min_variance = i32::MAX;
    let mut min_sec = 0;

    for sec in 1..(WIDTH as i32 * HIEGHT as i32) {
        for i in 0..bc {
            bots[i].pos.0 = (bots[i].pos.0 + bots[i].v.0).rem_euclid(WIDTH);
            bots[i].pos.1 = (bots[i].pos.1 + bots[i].v.1).rem_euclid(HIEGHT);
        }

        // how much the left differs from the right
        let mut variaince = 0;
        for bot in bots.iter() {

            // assumes mirrored posiitons
            if bot.pos.0 > WIDTH / 2 {
                continue;
            }

            let pos = (WIDTH - bot.pos.0, bot.pos.1);
            if !bots.iter().any(|b| b.pos == pos) {
                variaince += 1;
            }
        }

        if variaince < min_variance {
            min_variance = variaince;
            min_sec = sec;
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(min_sec),
        time_ms,
    }
}

#[derive(Debug)]
struct Bot {
    pos: (i8, i8),
    v: (i8, i8),
}

impl Bot {
    pub fn from_line(line: &str) -> Self {
        let (pos, vel) = line.split_once(' ').unwrap();

        let (x_pos, y_pos) = pos.split_once(',').unwrap();
        let x_pos = x_pos.split_once('=').unwrap().1.parse::<i8>().unwrap();
        let y_pos = y_pos.parse::<i8>().unwrap();

        let (x_vel, y_vel) = vel.split_once(',').unwrap();
        let x_vel = x_vel.split_once('=').unwrap().1.parse::<i8>().unwrap();
        let y_vel = y_vel.parse::<i8>().unwrap();

        Self {
            pos: (x_pos, y_pos),
            v: (x_vel, y_vel),
        }
    }
}