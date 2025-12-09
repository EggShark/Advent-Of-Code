use crate::Solve;

use std::time::Instant;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let points = data_in.lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        (l.parse().unwrap(),r.parse().unwrap())
    }).collect::<Vec<(i64, i64)>>();

    let mut max_area = 0;
    for &(x1, y1) in points.iter() {
        for &(x2, y2) in points.iter() {
            let area = ((x1-x2+1)*(y1-y2+1)).abs();
            max_area = std::cmp::max(area, max_area);
        }
    }

    let solve = max_area;

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let points = data_in.lines().map(|l| {
        let (l, r) = l.split_once(',').unwrap();
        (l.parse().unwrap(),r.parse().unwrap())
    }).collect::<Vec<(i64, i64)>>();

    let edges = get_edges(&points);

    let mut max_area = 0;

    for &(x1, y1) in points.iter() {
        for &(x2, y2) in points.iter() {
            let area = ((x1-x2).abs() + 1) * ((y1-y2).abs() + 1);
            if area <= max_area {
                continue;
            }

            let min_x = std::cmp::min(x1, x2);
            let max_x = std::cmp::max(x1, x2);
            let min_y = std::cmp::min(y1, y2);
            let max_y = std::cmp::max(y1, y2);

            if is_fully_contained(&edges, min_x, min_y, max_x, max_y) {
                max_area = area;
            }
        } 
    }

    let solve = max_area;

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn get_edges(points: &[(i64, i64)]) -> Vec<(i64, i64, i64, i64)> {
    let mut edges = Vec::with_capacity(points.len() / 2 + 1);
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i+1)%points.len()];
        edges.push((
            p1.0.min(p2.0),
            p1.1.min(p2.1),
            p1.0.max(p2.0),
            p1.1.max(p2.1),
        ));
    }
    edges
}

fn is_fully_contained(edges: &[(i64, i64, i64, i64)], min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> bool {
    for &(e_min_x, e_min_y, e_max_x, e_max_y) in edges {
        if min_x < e_max_x && max_x > e_min_x && min_y < e_max_y && max_y > e_min_y {
            return false;
        }
    }
    true
}
