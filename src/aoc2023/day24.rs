use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day24").unwrap();
    let time = Instant::now();

//     let text = "19, 13, 30 @ -2, 1, -2
// 18, 19, 22 @ -1, -1, -2
// 20, 25, 34 @ -2, -2, -4
// 12, 31, 28 @ -1, -2, -1
// 20, 19, 15 @ 1, -5, -3";


    let particles = text
        .lines()
        .map(|line| {
            let (pos, speed) = line.split_once(" @ ").unwrap();
            let mut pos = pos.split(", ").map(|i| i.parse::<i128>().unwrap());
            let mut speed = speed.split(", ").map(|i| i.parse::<i128>().unwrap());
            Particle::new(
                pos.next().unwrap(),
                pos.next().unwrap(),
                pos.next().unwrap(),
                speed.next().unwrap(),
                speed.next().unwrap(),
                speed.next().unwrap(),
            )
        })
        .collect::<Vec<Particle>>();
     
    let min_bound: i128 = 200000000000000;
    let max_bound: i128 = 400000000000000;

    let mut total = 0;
    for (idx, hs1) in particles.iter().enumerate() {
        for hs2 in particles[0..idx].iter() {
            // lines are parallel
            if hs1.a * hs2.b == hs1.b * hs2.a {
                continue;
            }
            // LINEAR ALEGBRA SOLVE SYSTEMS OF EQUATIONS :(
            let x = (hs1.c * hs2.b - hs2.c * hs1.b) / (hs1.a * hs2.b - hs2.a * hs1.b);
            let y = (hs2.c * hs1.a - hs1.c * hs2.a) / (hs1.a * hs2.b - hs2.a * hs1.b);
            // check if collision is the past as the x, y should have same sign as 
            // veloicty if its in the present
            if x >= min_bound && x <= max_bound && y >= min_bound && y <= max_bound {
                if [hs1, hs2].into_iter().all(|hs| (x - hs.x) * hs.vx >= 0 && (y - hs.y) * hs.vy >= 0) {
                    total += 1;
                }
            }
        }
    }

    // used this to format output for a linear equation solver :pensive:
    // let ts = ['r', 's', 't'];
    // for (idx, hs) in particles.iter().enumerate() {
    //     println!("x + u {} = {} {} {} {}", ts[idx], hs.x, if hs.vx < 0 {'-'} else {'+'}, if hs.vx.abs() == 1 {"".to_string()} else {hs.vx.abs().to_string()}, ts[idx]);
    //     println!("y + v {} = {} {} {} {}", ts[idx], hs.y, if hs.vy < 0 {'-'} else {'+'}, if hs.vy.abs() == 1 {"".to_string()} else {hs.vy.abs().to_string()}, ts[idx]);
    //     println!("z + w {} = {} {} {} {}", ts[idx], hs.z, if hs.vz < 0 {'-'} else {'+'}, if hs.vz.abs() == 1 {"".to_string()} else {hs.vz.abs().to_string()}, ts[idx]);
    //     if idx >= 2 {
    //         break;
    //     }
    // }

    let sol1: u64 = total;
    // link to linear equation solver
    let sol2: String = "https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_solveequationsadvanced&v1=x%2520%2B%2520u%2520r%2520%253D%2520219051609191782%2520%2B%2520146%2520r%250Ay%2520%2B%2520v%2520r%2520%253D%252068260434807407%2520%2B%2520364%2520r%250Az%2520%2B%2520w%2520r%2520%253D%2520317809635461867%2520-%252022%2520r%250Ax%2520%2B%2520u%2520s%2520%253D%2520292151991892724%2520-%252043%2520s%250Ay%2520%2B%2520v%2520s%2520%253D%2520394725036264709%2520-%2520280%2520s%250Az%2520%2B%2520w%2520s%2520%253D%2520272229701860796%2520-%252032%2520s%250Ax%2520%2B%2520u%2520t%2520%253D%2520455400538938496%2520-%2520109%2520t%250Ay%2520%2B%2520v%2520t%2520%253D%2520167482380286201%2520%2B%2520219%2520t%250Az%2520%2B%2520w%2520t%2520%253D%2520389150487664328%2520-%252058%2520t&v2=x%250Ay%250Az%250Ar%250As%250At%250Au%250Aw%250Av".to_string();

    let solution = (SolutionType::U64(sol1), SolutionType::Str(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}


#[derive(Debug)]
struct Particle {
    x: i128,
    y: i128,
    z: i128,
    vx: i128,
    vy: i128,
    vz: i128,
    a: i128,
    b: i128,
    c: i128
}

impl Particle {
    fn new(x: i128, y: i128, z: i128, vx: i128, vy: i128, vz: i128) -> Self {
        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
            a: vy,
            b: -vx,
            c: vy * x - vx * y,
        }
    }
}