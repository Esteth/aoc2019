use std::boxed::Box;
use std::result::Result;
use std::io::BufRead;
use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Debug)]
struct ComputeError {}

impl Display for ComputeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ComputeError")
    }
}

impl std::error::Error for ComputeError {}

#[derive(Debug)]
struct Computer {
    pc: usize,
    memory: Vec<usize>,
}

impl Computer {
    fn run_to_completion(&mut self) -> Result<(), ComputeError> {
        loop {
            if self.step()? {
                break;
            }
        }
        Ok(())
    }

    fn step(&mut self) -> Result<bool, ComputeError> {
        match self.memory[self.pc] {
            1 => {
                let x = self.memory[self.memory[self.pc + 1]];
                let y = self.memory[self.memory[self.pc + 2]];
                let dest = self.memory[self.pc + 3];
                self.memory[dest] = x + y;
            }
            2 => {
                let x = self.memory[self.memory[self.pc + 1]];
                let y = self.memory[self.memory[self.pc + 2]];
                let dest = self.memory[self.pc + 3];
                self.memory[dest] = x * y;
            }
            99 => return Ok(true),
            _ => {
                println!("error: pc={}, memory={:?}", self.pc, self.memory);
                return Err(ComputeError {});
            }
        }
        self.pc += 4;

        Ok(false)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input)?;
    let memory: Vec<usize> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();

    let mut computer = Computer {
        pc: 0,
        memory,
    };

    computer.run_to_completion()?;
    println!("{:?}", computer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let mut c = Computer {
            pc: 0,
            memory: vec![1, 9, 10, 3,
                         2, 3, 11, 0,
                         99,
                         30, 40, 50],
        };
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![3500, 9, 10, 70,
                                  2, 3, 11, 0,
                                  99,
                                  30, 40, 50]);
    }

    #[test]
    fn second_example() {
        let mut c = Computer {
            pc: 0,
            memory: vec![1, 0, 0, 0, 99],
        };
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn third_example() {
        let mut c = Computer {
            pc: 0,
            memory: vec![2, 3, 0, 3, 99],
        };
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn fourth_example() {
        let mut c = Computer {
            pc: 0,
            memory: vec![2, 4, 4, 5, 99, 0],
        };
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn fifth_example() {
        let mut c = Computer {
            pc: 0,
            memory: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}