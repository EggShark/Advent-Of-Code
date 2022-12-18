use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::{HashSet, VecDeque};
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day18.txt").unwrap();
    let time = Instant::now();
    let cubes = text.lines().map(|line| Point3D::from_str(line)).collect::<Vec<Point3D>>();
    let cubes_set = text.lines().map(|line| Point3D::from_str(line)).collect::<HashSet<Point3D>>();

    let sol1: i32 = part_1(cubes);
    let sol2: i32 = part_2(cubes_set);

    let solution = (SolutionType::I32(sol1), SolutionType::I32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn part_1(cubes: Vec<Point3D>) -> i32 {
    let mut sfa = 0;

    for p in cubes.iter() {
        // checks point around it prob not very time effecient
        // its slow but it works
        if !cubes.contains(&Point3D{x: p.x + 1, y: p.y, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x - 1, y: p.y, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y + 1, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y - 1, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z + 1}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z - 1}) {
            sfa += 1;
        }
    }

    sfa
}

fn part_2(mut cubes: HashSet<Point3D>) -> i32 {
    // makes a bounding box a bit bigger than the droplet
    let max_x = cubes.iter().map(|p| p.x).max().unwrap()+2;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap()+2;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap()+2;
    println!("{}, {}, {}", max_x, max_y, max_z);
    let mut sfa = 0;
    // same as part one
    for p in cubes.iter() {
        if !cubes.contains(&Point3D{x: p.x + 1, y: p.y, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x - 1, y: p.y, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y + 1, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y - 1, z: p.z}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z + 1}) {
            sfa += 1;
        }
        if !cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z - 1}) {
            sfa += 1;
        }
    }
    let mut que = VecDeque::new();
    que.push_back(Point3D{x:-1, y:-1, z:-1});
    cubes.insert(Point3D{x:-1, y:-1, z:-1});
    while let Some(point) = que.pop_front() {
        for p3 in [Point3D{x:1,y:0,z:0}, Point3D{x:-1,y:0,z:0}, Point3D{x:0,y:1,z:0}, Point3D{x:0,y:-1,z:0}, Point3D{x:0,y:0,z:1}, Point3D{x:0,y:0,z:-1}] {
            let new_point = Point3D{x: point.x + p3.x, y: point.y + p3.y, z: point.z + p3.z};
            if !cubes.contains(&new_point) && (-1..max_x).contains(&new_point.x) && (-1..max_y).contains(&new_point.y) && (-1..max_z).contains(&new_point.z) {
                cubes.insert(new_point);
                que.push_back(new_point);
            }
        }
    }
    let mut not_cubes = HashSet::new();
    // "fills" in the air gaps
    for x in -1..max_x {
        for y in -1..max_y {
            for z in -1..max_z {
                if !cubes.contains(&Point3D{x,y,z}) {
                    not_cubes.insert(Point3D{x,y,z});
                }
            }
        }
    }
    for p in not_cubes {
        if cubes.contains(&Point3D{x: p.x + 1, y: p.y, z: p.z}) {
            sfa -= 1;
        }
        if cubes.contains(&Point3D{x: p.x - 1, y: p.y, z: p.z}) {
            sfa -= 1;
        }
        if cubes.contains(&Point3D{x: p.x, y: p.y + 1, z: p.z}) {
            sfa -= 1;
        }
        if cubes.contains(&Point3D{x: p.x, y: p.y - 1, z: p.z}) {
            sfa -= 1;
        }
        if cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z + 1}) {
            sfa -= 1;
        }
        if cubes.contains(&Point3D{x: p.x, y: p.y, z: p.z - 1}) {
            sfa -= 1;
        }
    }
    sfa
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i8,
    y: i8,
    z: i8,
}

impl Point3D {
    fn from_str(line: &str) -> Self {
        let mut s = line.split(",").map(|u| u.parse::<i8>().unwrap());
        let x = s.next().unwrap();
        let y = s.next().unwrap();
        let z = s.next().unwrap();
        Self {
            x,
            y,
            z,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    const TESTINPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn p1() {
        let cubes = TESTINPUT.lines().map(|line| Point3D::from_str(line)).collect::<Vec<Point3D>>();
        assert_eq!(part_1(cubes), 64)
    }

    #[test]
    fn p2() {
        let cubes = TESTINPUT.lines().map(|line| Point3D::from_str(line)).collect::<HashSet<Point3D>>();
        assert_eq!(part_2(cubes), 58)
    }

}