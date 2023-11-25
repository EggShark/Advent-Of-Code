use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

const BEACONROW: i32 = 2000000;

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2022/texts/day15.txt").unwrap();
    let time = Instant::now();

    let pairs = text.lines().map(|line| Pair::from_str(line)).collect::<Vec<Pair>>();

    let mut ranges: Vec<(i32, i32)> = Vec::new();
    for pair in &pairs {
        // gets the totall range it can go
        let range = (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();
        // gets the sensors distance to the row
        let distance = (pair.sensor.y - BEACONROW).abs();
        if distance > range {
            continue;
            // if the row is out of range just ignore it
        }
        let left = pair.sensor.x - range + distance;
        let right = pair.sensor.x + range - distance;
        ranges.push((left, right));
    }
    ranges.sort_by_key(|&(start, _)| start);

    let mut count = 0;
    let mut last_end = i32::MIN;
    for mut range in ranges {
        range.0 = range.0.max(last_end + 1);
        if range.1 < range.0 {
            continue;
        }
        last_end = range.1;
        count += range.1 - range.0 + 1;

        let mut beacons_in_row = pairs.iter().filter(|pair| {
            pair.beacon.y == BEACONROW && (range.0 <= pair.beacon.x && pair.beacon.y <= range.1)
        })
        .map(|pair| pair.beacon)
        .collect::<Vec<Point>>();

        beacons_in_row.dedup();
        count -= beacons_in_row.len() as i32;
    }

    let ranges = pairs.iter()
        .map(|pair| (pair.sensor, (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs()))
        .collect::<Vec<(Point, i32)>>();

    let eval_canidate = |(x,y): (i32, i32)| {
        if x < 0 || y < 0 || x > 4000000 || y > 4000000 {
            return false;
        }
        for &(sensor, range) in &ranges {
            let dist = (sensor.x - x).abs() + (sensor.y - y).abs();
            if dist <= range {
                return false;
            }
        }
        true
    };
    let mut found = pairs.iter()
        .filter_map(|&pair| {
            let range = (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();
            let dist = range + 1;
            for x in (pair.sensor.x - dist)..=(pair.sensor.x + dist) {
                let slack = dist - (pair.sensor.x - x).abs();
                let highy = pair.sensor.y + slack;
                let lowy = pair.sensor.y - slack;
                if eval_canidate((x, highy)) {
                    return Some((x, highy));
                }
                if eval_canidate((x, lowy)) {
                    return Some((x, lowy));
                }
            }

            None
        }).collect::<Vec<(i32,i32)>>();
    found.dedup();

    let sol1: i32 = count;
    let sol2: i64 = found[0].0 as i64 * 4000000 + found[0].1 as i64;

    let solution = (SolutionType::I32(sol1), SolutionType::I64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair {
    beacon: Point,
    sensor: Point,
}

impl Pair {
    pub fn from_str(line: &str) -> Self {
        let (sensor, beacon) = line.split_once(": ").unwrap();
        let (sensor_x, sensor_y) = sensor.split_once(", y=").unwrap();
        let sensor_y = sensor_y.parse::<i32>().unwrap();
        let sensor_x = sensor_x[12..sensor_x.len()].parse::<i32>().unwrap();
        let sensor = Point {
            x: sensor_x,
            y: sensor_y
        };

        let (beacon_x, beacon_y) = beacon.split_once(", y=").unwrap();
        let beacon_y = beacon_y.parse::<i32>().unwrap();
        let beacon_x = beacon_x[23..beacon_x.len()].parse::<i32>().unwrap();
        let beacon = Point {
            x: beacon_x,
            y: beacon_y,
        };

        Self {
            sensor,
            beacon
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}