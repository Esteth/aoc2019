extern crate simple_error;

use std::boxed::Box;
use std::result::Result;
use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn x_mod(&self) -> i32 {
        match self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }
    fn y_mod(&self) -> i32 {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    direction: Direction,
    magnitude: u32,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("U", mag) => Ok(Instruction {
                direction: Direction::Up,
                magnitude: mag.parse()?,
            }),
            ("R", mag) => Ok(Instruction {
                direction: Direction::Right,
                magnitude: mag.parse()?,
            }),
            ("D", mag) => Ok(Instruction {
                direction: Direction::Down,
                magnitude: mag.parse()?,
            }),
            ("L", mag) => Ok(Instruction {
                direction: Direction::Left,
                magnitude: mag.parse()?,
            }),
            _ => Err(Box::new(simple_error::SimpleError::new("first char must be one of URDL")))
        }
    }
}

fn manhattan_distance_from_origin(point: (i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}

fn read_wire<R: BufRead>(r: &mut R) -> Vec<Instruction> {
    let mut wire = String::new();
    r.read_line(&mut wire);
    wire.split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let wire1 = read_wire(&mut stdin_lock);

    let mut visited_1: HashSet<(i32, i32)> = HashSet::new();
    let mut dist_1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    let mut total_traveled = 0;
    for (i, instruction) in wire1.iter().enumerate() {
        let x_mod: i32 = instruction.direction.x_mod();
        let y_mod: i32 = instruction.direction.y_mod();

        for _ in 0..instruction.magnitude {
            pos = (pos.0 + x_mod, pos.1 + y_mod);
            total_traveled += 1;
            visited_1.insert(pos);
            dist_1.insert(pos, total_traveled);
        }
    }

    let wire2 = read_wire(&mut stdin_lock);
    let mut visited_2: HashSet<(i32, i32)> = HashSet::new();
    let mut dist_2: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);
    let mut total_traveled = 0;
    for (i, instruction) in wire2.iter().enumerate() {
        let x_mod: i32 = instruction.direction.x_mod();
        let y_mod: i32 = instruction.direction.y_mod();

        for _ in 0..instruction.magnitude {
            pos = (pos.0 + x_mod, pos.1 + y_mod);
            total_traveled += 1;
            visited_2.insert(pos);
            dist_2.insert(pos, total_traveled);
        }
    }

    let intersection = visited_1.intersection(&visited_2);
    println!("intersection: {:?}", intersection);
    let min_dist = intersection.into_iter()
        .map(|p| dist_1[p] + dist_2[p])
        .min();
    println!("min_dist: {:?}", min_dist.unwrap());
    Ok(())
}