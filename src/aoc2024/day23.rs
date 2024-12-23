use crate::Solve;
use std::{collections::{BTreeSet, HashMap, HashSet}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let edges = data_in
        .lines()
        .filter_map(|l| l.split_once('-'));

    let mut graph = HashMap::new();

    for (left, right) in edges {
        let left_entry = graph.entry(left).or_insert(HashSet::new());
        left_entry.insert(right);

        let right_entry = graph.entry(right).or_insert(HashSet::new());
        right_entry.insert(left);
    }

    let mut sets = HashSet::new();
    for x in graph.keys() {
        for y in graph[x].iter() {
            for z in graph[y].iter() {
                if x != z && graph[z].contains(x) {
                    let mut h = [x, y, z];
                    h.sort();
                    sets.insert(h);
                }
            }
        }
    }

    let solve = sets.into_iter().map(|d| d.iter().any(|d| d.starts_with('t')))
        .filter(|d| *d)
        .count();

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let edges = data_in
        .lines()
        .filter_map(|l| l.split_once('-'));

    let mut graph = HashMap::new();

    for (left, right) in edges {
        let left_entry = graph.entry(left).or_insert(HashSet::new());
        left_entry.insert(right);

        let right_entry = graph.entry(right).or_insert(HashSet::new());
        right_entry.insert(left);
    }
    let nodes = graph.keys().copied().collect::<HashSet<_>>();

    let prev_solutions = nodes
        .iter()
        .copied()
        .map(|node| core::iter::once(node).collect::<BTreeSet<_>>())
        .collect::<HashSet<_>>();

    let out = solve_p2(graph, nodes, prev_solutions);

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(out),
        time_ms,
    }
}

fn solve_p2<'a>(graph: HashMap<&'a str, HashSet<&'a str>>, nodes: HashSet<&'a str>, mut prev_solutions: HashSet<BTreeSet<&'a str>>) -> String {
    loop {
        let solutions = prev_solutions
            .iter()
            .flat_map(|prev_solution| {
                nodes
                    .iter()
                    .copied()
                    .filter(|&node| {
                        prev_solution.iter().copied().all(|sol_node| {
                            sol_node != node && graph.get(sol_node).unwrap().contains(node)
                        })
                    })
                    .map(|node| {
                        let mut new_solution = prev_solution.clone();
                        new_solution.insert(node);
                        new_solution
                    })
            })
            .collect::<HashSet<_>>();

        if solutions.is_empty() {
            let solution = prev_solutions
                .iter()
                .next()
                .unwrap()
                .iter()
                .copied()
                .collect::<Vec<_>>();
            return solution.join(",");
        }

        prev_solutions = solutions;
    }
}