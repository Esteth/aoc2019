use std::error::Error;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::thread;

#[derive(Copy, Clone)]
enum Direction { Left, Right }

#[derive(Copy, Clone)]
enum Facing { North, South, East, West }

impl Facing {
    fn mv(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match *self {
            Facing::North => (x, y + 1),
            Facing::South => (x, y - 1),
            Facing::East => (x + 1, y),
            Facing::West => (x - 1, y),
        }
    }

    fn turn(&self, direction: Direction) -> Facing {
        match *self {
            Facing::North => {
                match direction {
                    Direction::Left => Facing::West,
                    Direction::Right => Facing::East,
                }
            }
            Facing::South => {
                match direction {
                    Direction::Left => Facing::East,
                    Direction::Right => Facing::West,
                }
            }
            Facing::East => {
                match direction {
                    Direction::Left => Facing::North,
                    Direction::Right => Facing::South,
                }
            }
            Facing::West => {
                match direction {
                    Direction::Left => Facing::South,
                    Direction::Right => Facing::North,
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;
    let memory: Vec<i64> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();

    let mut panels: HashMap<(i32, i32), bool> = HashMap::new();

    let (input_sender, input_receiver) = crossbeam_channel::unbounded();
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let (input_request_sender, input_request_receiver) = crossbeam_channel::unbounded();
    thread::spawn(move || {
        let mut computer = aoc::computer::Computer::new(memory, input_receiver, input_request_sender, output_sender);
        computer.run_to_completion();
    });

    let mut positions = HashSet::new();
    let mut position = (0, 0);
    let mut facing: Facing = Facing::North;
    input_sender.send(0);
    loop {
        let color_to_paint = match output_receiver.recv()? {
            0 => false,
            _ => true,
        };
        panels.insert(position, color_to_paint);
        positions.insert(position);
        let direction_to_turn = match output_receiver.recv()? {
            0 => Direction::Left,
            _ => Direction::Right,
        };
        facing = facing.turn(direction_to_turn);
        position = facing.mv(position);

        let new_color = match panels.get(&position) {
            Some(color) => *color,
            None => false,
        };
        input_sender.send(match new_color {
            true => 1,
            false => 0,
        });
        println!("{}", positions.len());
    }
    Ok(())
}