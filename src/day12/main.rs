use std::error::Error;
use std::str::FromStr;
use simple_error::SimpleError;
use std::io::BufRead;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Asteroid {
    x: i32,
    y: i32,
    z: i32,
    velocity: Vector,
}

impl Asteroid {
    fn new(x: i32, y: i32, z: i32) -> Asteroid {
        Asteroid {
            x,
            y,
            z,
            velocity: Vector { x: 0, y: 0, z: 0 },
        }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.velocity.energy()
    }
}

impl FromStr for Asteroid {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<i32> = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
        if components.len() == 3 {
            Ok(Asteroid::new(components[0], components[1], components[2]))
        } else {
            Err(SimpleError::new("Must be exactly 3 components"))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Debug)]
struct System {
    asteroids: Vec<Asteroid>,
}

impl System {
    fn energy(&self) -> i32 {
        self.asteroids.iter().map(|asteroid| asteroid.energy()).sum()
    }

    fn step(&mut self) {
        let mut new_asteroids: Vec<Asteroid> = Vec::with_capacity(self.asteroids.len());
        for asteroid in &self.asteroids {
            let mut x_move = 0;
            let mut y_move = 0;
            let mut z_move = 0;
            for other_asteroid in &self.asteroids {
                x_move += match asteroid.x.cmp(&other_asteroid.x) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                y_move += match asteroid.y.cmp(&other_asteroid.y) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                z_move += match asteroid.z.cmp(&other_asteroid.z) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
            }
            let velocity = Vector {
                x: asteroid.velocity.x + x_move,
                y: asteroid.velocity.y + y_move,
                z: asteroid.velocity.z + z_move,
            };
            new_asteroids.push(
                Asteroid {
                    x: asteroid.x + velocity.x,
                    y: asteroid.y + velocity.y,
                    z: asteroid.z + velocity.z,
                    velocity,
                });
        }
        self.asteroids = new_asteroids;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SingleDimensionalSystem {
    asteroids: Vec<(i32, i32)>,
}

impl SingleDimensionalSystem {
    fn step(&mut self) {
        let mut new_asteroids = Vec::new();
        for asteroid in &self.asteroids {
            let mut diff = self.asteroids.iter().fold(0, |acc, other_asteroid| {
                match asteroid.0.cmp(&other_asteroid.0) {
                    Ordering::Less => acc + 1,
                    Ordering::Equal => acc,
                    Ordering::Greater => acc - 1,
                }
            });
            new_asteroids.push((asteroid.0 + asteroid.1 + diff, asteroid.1 + diff));
        }
        self.asteroids = new_asteroids;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    for line in stdin_lock.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        asteroids.push(line.parse()?);
    }
    let original_x_system = SingleDimensionalSystem { asteroids: asteroids.iter().map(|a| (a.x, a.velocity.x)).collect() };
    let mut x_system = original_x_system.clone();
    let mut steps_to_x_cycle = 0;
    loop {
        x_system.step();
        steps_to_x_cycle += 1;
        if x_system == original_x_system {
            break;
        }
    }

    let original_y_system = SingleDimensionalSystem { asteroids: asteroids.iter().map(|a| (a.y, a.velocity.y)).collect() };
    let mut y_system = original_y_system.clone();
    let mut steps_to_y_cycle = 0;
    loop {
        y_system.step();
        steps_to_y_cycle += 1;
        if y_system == original_y_system {
            break;
        }
    }

    let original_z_system = SingleDimensionalSystem { asteroids: asteroids.iter().map(|a| (a.z, a.velocity.z)).collect() };
    let mut z_system = original_z_system.clone();
    let mut steps_to_z_cycle = 0;
    loop {
        z_system.step();
        steps_to_z_cycle += 1;
        if z_system == original_z_system {
            break;
        }
    }

    println!("Cycles for x: {}, y: {}, z: {}", steps_to_x_cycle, steps_to_y_cycle, steps_to_z_cycle);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asteroid_from_string() {
        let input = "  24,  12   , 42 ";
        let asteroid: Asteroid = input.parse().unwrap();
        assert_eq!(asteroid.x, 24);
        assert_eq!(asteroid.y, 12);
        assert_eq!(asteroid.z, 42);
    }

    #[test]
    fn asteroid_move() {
        let mut asteroid = Asteroid::new(10, 20, 30);
        asteroid.velocity = Vector { x: 5, y: -10, z: -50 };
        asteroid.mv();
        assert_eq!(asteroid.x, 15);
        assert_eq!(asteroid.y, 10);
        assert_eq!(asteroid.z, -20);
    }

    #[test]
    fn system_step() {
        let mut system = System {
            asteroids: vec![
                Asteroid::new(10, 1, 11),
                Asteroid::new(5, 20, 30),
                Asteroid::new(4, -19, 10),
                Asteroid::new(15, -4, -4),
            ],
        };
        system.step();
        assert_eq!(
            system.asteroids,
            vec![
                Asteroid { x: 9, y: 0, z: 10, velocity: Vector { x: -1, y: -1, z: -1 } },
                Asteroid { x: 6, y: 17, z: 27, velocity: Vector { x: 1, y: -3, z: -3 } },
                Asteroid { x: 7, y: -16, z: 11, velocity: Vector { x: 3, y: 3, z: 1 } },
                Asteroid { x: 12, y: -3, z: -1, velocity: Vector { x: -3, y: 1, z: 3 } },
            ]
        )
    }

    #[test]
    fn system_energy() {
        let mut system = System {
            asteroids: vec![
                Asteroid::new(10, 1, 11),
                Asteroid::new(5, 20, 30),
                Asteroid::new(4, -19, 10),
                Asteroid::new(15, -4, -4),
            ],
        };
        assert_eq!(system.energy(), 133);
        system.step();
        assert_eq!(system.energy(), 143);
    }

    #[test]
    fn system_energy_eg1() {
        let mut system = System {
            asteroids: vec![
                Asteroid::new(-1, 0, 2),
                Asteroid::new(2, -10, -7),
                Asteroid::new(4, -8, 8),
                Asteroid::new(3, 5, -1),
            ],
        };
        for _ in 0..10 {
            system.step();
        }
        assert_eq!(system.energy(), 179);
    }
}