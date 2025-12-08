use crate::Solve;

use std::time::Instant;
use std::collections::HashSet;

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();


    let coords = data_in.lines().map(|l| {
        let mut h = l.split(',');
        let x = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        let y = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        let z = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        (x, y, z)
    }).collect::<Vec<(i64, i64, i64)>>();

    let mut circuts: Vec<HashSet<usize>> = Vec::new();
    let mut combs = HashSet::new();

    for (idx, &(x, y, z)) in coords.iter().enumerate() {
        for (idx_s, &(xs, ys, zs)) in coords.iter().enumerate() {
            if idx == idx_s {
                continue;
            }
            let d = (xs-x)*(xs-x) + (ys-y)*(ys-y) + (zs-z)*(zs-z);
            if !combs.contains(&(d, idx_s, idx)) {
                combs.insert((d, idx, idx_s));
            }
        }
    }

    let mut combs = combs.into_iter().collect::<Vec<_>>();

    combs.sort_unstable_by_key(|(d, _, _)| *d);

    for &(_, idx, idx_s) in combs.iter().take(1000) {
        let a = circuts.iter().position(|c| c.contains(&idx));
        let b = circuts.iter().position(|c| c.contains(&idx_s));

        if let Some(a) = a && let Some(b) = b && a != b {
            let c_to_mege = circuts.remove(b);
            if a > b {
                circuts[a-1].extend(c_to_mege);
            } else {
                circuts[a].extend(c_to_mege);
            }
        } else if let Some(a) = a {
            circuts[a].insert(idx_s);
        } else if let Some(b) = b {
            circuts[b].insert(idx);
        } else {
            let mut c = HashSet::new();
            c.insert(idx);
            c.insert(idx_s);
            circuts.push(c);
        }
    }

    circuts.sort_unstable_by_key(|c| c.len());  
    let l1 = circuts.pop().unwrap().len();
    let l2 = circuts.pop().unwrap().len();
    let l3 = circuts.pop().unwrap().len();

    let solve = l1 * l2 * l3;

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let coords = data_in.lines().map(|l| {
        let mut h = l.split(',');
        let x = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        let y = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        let z = h.next().map(|c| c.parse::<i64>().unwrap()).unwrap();
        (x, y, z)
    }).collect::<Vec<(i64, i64, i64)>>();

    let mut circuts: Vec<HashSet<usize>> = Vec::new();
    let mut combs = HashSet::new();

    for (idx, &(x, y, z)) in coords.iter().enumerate() {
        for (idx_s, &(xs, ys, zs)) in coords.iter().enumerate() {
            if idx == idx_s {
                continue;
            }
            let d = (xs-x)*(xs-x) + (ys-y)*(ys-y) + (zs-z)*(zs-z);
            if !combs.contains(&(d, idx_s, idx)) {
                combs.insert((d, idx, idx_s));
            }
        }
    }

    let mut combs = combs.into_iter().collect::<Vec<_>>();

    combs.sort_unstable_by_key(|(d, _, _)| *d);

    for &(_, idx, idx_s) in combs.iter() {
        let a = circuts.iter().position(|c| c.contains(&idx));
        let b = circuts.iter().position(|c| c.contains(&idx_s));

        if let Some(a) = a && let Some(b) = b && a != b {
            let c_to_mege = circuts.remove(b);
            if a > b {
                circuts[a-1].extend(c_to_mege);
            } else {
                circuts[a].extend(c_to_mege);
            }
        } else if let Some(a) = a {
            circuts[a].insert(idx_s);
        } else if let Some(b) = b {
            circuts[b].insert(idx);
        } else {
            let mut c = HashSet::new();
            c.insert(idx);
            c.insert(idx_s);
            circuts.push(c);
        }

        if circuts.len() == 1 && circuts[0].len()==coords.len() {
            solve = coords[idx].0 * coords[idx_s].0;
            break;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}
