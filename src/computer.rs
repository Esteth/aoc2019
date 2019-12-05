use std::result::Result;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};

#[derive(Copy, Clone, Debug)]
struct ComputeError {}

impl Display for ComputeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ComputeError")
    }
}

impl std::error::Error for ComputeError {}

pub struct Computer<'a> {
    ip: usize,
    memory: Vec<usize>,
    input: &'a mut dyn BufRead,
    output: &'a mut dyn Write,
}
impl std::fmt::Debug for Computer<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Computer{{ip={:?}, memory={:?}}}", self.ip, self.memory)
    }
}

impl Computer<'_> {
    pub fn new<'a>(
        memory: Vec<usize>,
        input: &'a mut dyn BufRead,
        output: &'a mut dyn Write) -> Computer<'a> {
        Computer {
            ip: 0,
            memory,
            input,
            output,
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
                self.ip += 4;
            }
            2 => {
                let x = self.memory[self.memory[self.ip + 1]];
                let y = self.memory[self.memory[self.ip + 2]];
                let dest = self.memory[self.ip + 3];
                self.memory[dest] = x * y;
                self.ip += 4;
            }
            3 => {
                let dest = self.memory[self.ip + 1];
                let mut input = String::new();
                self.input.read_line(&mut input).unwrap();
                let val: usize = input.trim().parse().unwrap();
                self.memory[dest] = val;
                self.ip += 2;
            }
            4 => {
                let loc = self.memory[self.ip + 1];
                writeln!(self.output, "{}", self.memory[loc]);
                self.ip += 2;
            }
            99 => return Ok(true),
            _ => {
                eprintln!("error: pc={}, memory={:?}", self.ip, self.memory);
                return Err(ComputeError {});
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn first_example() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c =
            Computer::new(
                vec![1, 9, 10, 3,
                     2, 3, 11, 0,
                     99,
                     30, 40, 50],
                &mut input,
                &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![3500, 9, 10, 70,
                                  2, 3, 11, 0,
                                  99,
                                  30, 40, 50]);
    }

    #[test]
    fn second_example() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![1, 0, 0, 0, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn third_example() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![2, 3, 0, 3, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn fourth_example() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![2, 4, 4, 5, 99, 0], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn fifth_example() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn input() {
        let mut input = Cursor::new("10\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3, 0, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![10, 0, 99]);
    }

    #[test]
    fn output() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![4, 0, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![4, 0, 99]);
        assert_eq!(String::from_utf8(output).unwrap(), "4\n")
    }
}