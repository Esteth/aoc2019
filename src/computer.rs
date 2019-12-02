use std::result::Result;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
struct ComputeError {}

impl Display for ComputeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ComputeError")
    }
}

impl std::error::Error for ComputeError {}

#[derive(Debug)]
pub struct Computer {
    ip: usize,
    memory: Vec<usize>,
}

impl Computer {
    pub fn new(memory: Vec<usize>) -> Computer {
        Computer {
            ip: 0,
            memory,
        }
    }

    pub fn run_to_completion(&mut self) -> Result<usize, Box<dyn Error>> {
        loop {
            if self.step()? {
                break;
            }
        }
        Ok(self.memory[0])
    }

    fn step(&mut self) -> Result<bool, ComputeError> {
        match self.memory[self.ip] {
            1 => {
                let x = self.memory[self.memory[self.ip + 1]];
                let y = self.memory[self.memory[self.ip + 2]];
                let dest = self.memory[self.ip + 3];
                self.memory[dest] = x + y;
            }
            2 => {
                let x = self.memory[self.memory[self.ip + 1]];
                let y = self.memory[self.memory[self.ip + 2]];
                let dest = self.memory[self.ip + 3];
                self.memory[dest] = x * y;
            }
            99 => return Ok(true),
            _ => {
                println!("error: pc={}, memory={:?}", self.ip, self.memory);
                return Err(ComputeError {});
            }
        }
        self.ip += 4;

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let mut c =
            Computer::new(vec![1, 9, 10, 3,
                               2, 3, 11, 0,
                               99,
                               30, 40, 50]);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![3500, 9, 10, 70,
                                  2, 3, 11, 0,
                                  99,
                                  30, 40, 50]);
    }

    #[test]
    fn second_example() {
        let mut c = Computer::new(vec![1, 0, 0, 0, 99]);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn third_example() {
        let mut c = Computer::new(vec![2, 3, 0, 3, 99]);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn fourth_example() {
        let mut c = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn fifth_example() {
        let mut c = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}