use std::result::Result;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use std::path::Iter;
use simple_error::SimpleError;

#[derive(Copy, Clone, Debug)]
struct ComputeError {}

impl Display for ComputeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ComputeError")
    }
}

impl std::error::Error for ComputeError {}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mult(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Exit,
}

#[derive(Debug)]
enum Parameter {
    Immediate(i32),
    Position(usize),
}

impl Parameter {
    fn new(value: i32, mode_bit: i32) -> Result<Parameter, SimpleError> {
        match mode_bit {
            0 => {
                if value < 0 {
                    Err(SimpleError::new(format!("Negative index: {}", value)))
                } else {
                    Ok(Parameter::Position(value as usize))
                }
            }
            1 => Ok(Parameter::Immediate(value)),
            _ => Err(SimpleError::new("Invalid mode bit")),
        }
    }
}

pub struct Computer<'a> {
    ip: usize,
    memory: Vec<i32>,
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
        memory: Vec<i32>,
        input: &'a mut dyn BufRead,
        output: &'a mut dyn Write) -> Computer<'a> {
        Computer {
            ip: 0,
            memory,
            input,
            output,
        }
    }

    pub fn run_to_completion(&mut self) -> Result<i32, Box<dyn Error>> {
        loop {
            if self.step()? {
                break;
            }
        }
        Ok(self.memory[0])
    }

    fn step(&mut self) -> Result<bool, Box<dyn Error>> {
        let opcode = self.memory[self.ip as usize] % 100;
        let mut m = self.memory[self.ip as usize] / 100;
        let mut modes = Vec::new();
        for _ in 0..3 {
            modes.push(m % 10);
            m /= 10;
        }

        let instruction =
            match opcode {
                1 => Instruction::Add(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i32, modes[2])?),
                2 => Instruction::Mult(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i32, modes[2])?),
                3 => Instruction::Input(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?),
                4 => Instruction::Output(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?),
                5 => Instruction::JumpIfTrue(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?),
                6 => Instruction::JumpIfFalse(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?),
                7 => Instruction::LessThan(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i32, modes[2])?),
                8 => Instruction::Equals(
                    Parameter::new((self.memory[self.ip + 1]) as i32, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i32, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i32, modes[2])?),
                99 => Instruction::Exit,
                _ => return Err(Box::new(ComputeError {}))
            };
        match instruction {
            Instruction::Add(x, y, Parameter::Position(dest)) => {
                let x = self.resolve(x);
                let y = self.resolve(y);
                self.memory[dest] = x + y;
                self.ip += 4;
            }
            Instruction::Mult(x, y, Parameter::Position(dest)) => {
                let x = self.resolve(x);
                let y = self.resolve(y);
                self.memory[dest] = x * y;
                self.ip += 4;
            }
            Instruction::Input(Parameter::Position(dest)) => {
                self.output.flush();
                let mut input = String::new();
                self.input.read_line(&mut input).unwrap();
                let val: i32 = input.trim().parse().unwrap();
                self.memory[dest] = val;
                self.ip += 2;
            }
            Instruction::Output(x) => {
                let val = self.resolve(x);
                writeln!(self.output, "{}", val);
                self.ip += 2;
            }
            Instruction::JumpIfTrue(test, loc) => {
                if self.resolve(test) != 0 {
                    self.ip = self.resolve(loc) as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::JumpIfFalse(test, loc) => {
                if self.resolve(test) == 0 {
                    self.ip = self.resolve(loc) as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::LessThan(x, y, Parameter::Position(dest)) => {
                if self.resolve(x) < self.resolve(y) {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4
            }
            Instruction::Equals(x, y, Parameter::Position(dest)) => {
                if self.resolve(x) == self.resolve(y) {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4
            }
            Instruction::Exit => return Ok(true),
            _ => {
                eprintln!("error: pc={}, memory={:?}", self.ip, self.memory);
                return Err(Box::new(ComputeError {}));
            }
        }

        Ok(false)
    }

    fn resolve(&self, x: Parameter) -> i32 {
        match x {
            Parameter::Immediate(x) => x,
            Parameter::Position(x) => self.memory[x],
        }
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

    #[test]
    fn immediate_mode() {
        let mut input = Cursor::new("test\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![101, 2, 1, 0, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory, vec![3, 2, 1, 0, 99]);
    }

    #[test]
    fn equals() {
        let mut input = Cursor::new("8\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "1\n")
    }

    #[test]
    fn equals_immediate() {
        let mut input = Cursor::new("8\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "1\n")
    }

    #[test]
    fn less_than() {
        let mut input = Cursor::new("8\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "0\n")
    }

    #[test]
    fn less_than_immediate() {
        let mut input = Cursor::new("8\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "0\n")
    }

    #[test]
    fn jump() {
        let mut input = Cursor::new("40\n");
        let mut output = Vec::new();
        let mut c = Computer::new(vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut input, &mut output);
        c.run_to_completion().unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "1\n")
    }
}