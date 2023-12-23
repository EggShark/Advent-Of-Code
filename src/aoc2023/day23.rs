use crate::{Solution, SolutionType};
use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day23").unwrap();
    let time = Instant::now();

//     let text = "#.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#";

    let grid = text
        .lines()
        .map(|line| {
            line.bytes().map(|b| b.into()).collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();

    let starting_pos = (1, 0);
    let end_pos = (grid[0].len() as i32 - 2, grid.len() as i32 - 1);

    let mut points = Vec::new();
    points.extend([starting_pos, end_pos]);



    for (y_idx, row) in grid.iter().enumerate() {
        for (x_idx, tile) in row.iter().enumerate() {
            if tile == &Tile::Wall {
                continue;
            }
            let mut neighbhoors = 0;
            for (new_row, new_colloum) in [((y_idx as i32 - 1), x_idx as i32), (y_idx as i32 + 1, x_idx as i32), (y_idx as i32, x_idx as i32 - 1), (y_idx as i32, x_idx as i32 + 1)] {
                if new_row >= 0
                    && new_row < grid.len() as i32
                    && new_colloum >= 0
                    && new_colloum < row.len() as i32
                    && grid[new_row as usize][new_colloum as usize] != Tile::Wall
                {
                    neighbhoors += 1;
                }
            }
            if neighbhoors >= 3 {
                points.push((x_idx as i32, y_idx as i32));
            }
        }
    }

    let graph = create_graph(&grid, &points, false);
    let p2_graph = create_graph(&grid, &points, true);

    let sol1: u64 = dfs(starting_pos, end_pos, &mut HashSet::new(), &graph) as u64;
    let sol2: u64 = dfs(starting_pos, end_pos, &mut HashSet::new(), &p2_graph) as u64;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

impl Tile {
    pub fn valid_dirs(&self, p2: bool) -> Box<dyn Iterator<Item = (i32, i32)>> {
        if p2 {
            return Box::new([(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter());
        }

        match self {
            Tile::Empty => Box::new([(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter()),
            Tile::UpSlope => Box::new([(0, -1)].into_iter()),
            Tile::DownSlope => Box::new([(0, 1)].into_iter()),
            Tile::RightSlope => Box::new([(1, 0)].into_iter()),
            Tile::LeftSlope => Box::new([(0, -1)].into_iter()),
            _ => unreachable!()
        }
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::Wall,
            b'<' => Self::LeftSlope,
            b'>' => Self::RightSlope,
            b'^' => Self::UpSlope,
            b'v' => Self::DownSlope,
            _ => unreachable!(),
        }
    }
}

fn dfs(
    point: (i32, i32),
    end_point: (i32, i32),
    seen: &mut HashSet<(i32, i32)>,
    graph: &HashMap<(i32, i32), HashMap<(i32, i32), i32>>
) -> f32 {
    if point == end_point {
        return 0.0;
    }

    // neat trick with floats and additions
    let mut max: f32 = f32::NEG_INFINITY;

    seen.insert(point);
    for pt in graph.get(&point).unwrap() {
        if !seen.contains(pt.0) {
            max = max.max(dfs(*pt.0, end_point, seen, &graph) + *pt.1 as f32);
        }
    }
    seen.remove(&point);

    max
}

fn create_graph(grid: &[Vec<Tile>], points: &[(i32, i32)], p2: bool) -> HashMap<(i32, i32), HashMap<(i32, i32), i32>> {
    let mut graph: HashMap<(i32, i32), HashMap<(i32, i32), i32>> = HashMap::new();
    for point in points.iter() {
        graph.insert(*point, HashMap::new());
    }

    for (start_x, start_y) in points.iter() {
        let mut stack = Vec::new();
        stack.push((*start_x, *start_y, 0));
        let mut seen = HashSet::new();
        seen.insert((*start_x, *start_y));

        while let Some((x, y, step_count)) = stack.pop() {
            if step_count != 0 && points.contains(&(x, y)) {
                let e = graph.entry((*start_x, *start_y)).or_default();
                e.insert((x, y), step_count);
                continue;
            }

            for (dx, dy) in grid[y as usize][x as usize].valid_dirs(p2) {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_y >= 0
                    && new_y < grid.len() as i32
                    && new_x >= 0
                    && new_x < grid[0].len() as i32
                    && grid[new_y as usize][new_x as usize] != Tile::Wall
                    && seen.insert((new_x, new_y))
                {
                    stack.push((new_x, new_y, step_count + 1));
                }
            }
        }
    }

    graph
}