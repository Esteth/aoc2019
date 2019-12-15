use std::result::Result;
use std::error::Error;
use std::fmt::{Display, Formatter};
use simple_error::SimpleError;
use crossbeam_channel::{Sender, Receiver};

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
    ModifyRelativeBase(Parameter),
    Exit,
}

#[derive(Debug)]
enum Parameter {
    Immediate(i64),
    Position(usize),
    Relative(isize),
}

impl Parameter {
    fn new(value: i64, mode_bit: i32) -> Result<Parameter, SimpleError> {
        match mode_bit {
            0 => {
                if value < 0 {
                    Err(SimpleError::new(format!("Negative index: {}", value)))
                } else {
                    Ok(Parameter::Position(value as usize))
                }
            }
            1 => Ok(Parameter::Immediate(value)),
            2 => Ok(Parameter::Relative(value as isize)),
            _ => Err(SimpleError::new("Invalid mode bit")),
        }
    }
}

pub struct Computer {
    ip: usize,
    rb: isize,
    memory: Vec<i64>,
    input: Receiver<i64>,
    input_request: Sender<()>,
    output: Sender<i64>,
}

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Computer{{ip={:?}, memory_at_ip={:?}}}", self.ip, self.memory[self.ip])
    }
}

impl Computer {
    pub fn new(
        memory: Vec<i64>,
        input: Receiver<i64>,
        input_request: Sender<()>,
        output: Sender<i64>) -> Computer {
        let mut large_memory = vec![0; 1_000_000_000];
        large_memory[..memory.len()].copy_from_slice(&memory);
        Computer {
            ip: 0,
            rb: 0,
            memory: large_memory,
            input,
            input_request,
            output,
        }
    }

    pub fn run_to_completion(&mut self) -> Result<i64, Box<dyn Error>> {
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
            modes.push((m % 10) as i32);
            m /= 10;
        }

        let instruction =
            match opcode {
                1 => Instruction::Add(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i64, modes[2])?),
                2 => Instruction::Mult(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i64, modes[2])?),
                3 => Instruction::Input(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?),
                4 => Instruction::Output(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?),
                5 => Instruction::JumpIfTrue(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?),
                6 => Instruction::JumpIfFalse(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?),
                7 => Instruction::LessThan(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i64, modes[2])?),
                8 => Instruction::Equals(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?,
                    Parameter::new((self.memory[self.ip + 2]) as i64, modes[1])?,
                    Parameter::new((self.memory[self.ip + 3]) as i64, modes[2])?),
                9 => Instruction::ModifyRelativeBase(
                    Parameter::new((self.memory[self.ip + 1]) as i64, modes[0])?),
                99 => Instruction::Exit,
                _ => return Err(Box::new(ComputeError {}))
            };
        match instruction {
            Instruction::Add(x, y, dest) => {
                let x = self.resolve(x);
                let y = self.resolve(y);
                let dest = self.resolve_save(dest) as usize;
                self.memory[dest] = x + y;
                self.ip += 4;
            }
            Instruction::Mult(x, y, dest) => {
                let x = self.resolve(x);
                let y = self.resolve(y);
                let dest = self.resolve_save(dest) as usize;
                self.memory[dest] = x * y;
                self.ip += 4;
            }
            Instruction::Input(dest) => {
                let dest = self.resolve_save(dest) as usize;
                println!("sending input request");
                self.input_request.send(())?;
                let val = self.input.recv()?;
                println!("received {}", val);
                self.memory[dest] = val;
                self.ip += 2;
            }
            Instruction::Output(x) => {
                let val = self.resolve(x);
                self.output.send(val)?;
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
            Instruction::LessThan(x, y, dest) => {
                let dest = self.resolve_save(dest) as usize;
                if self.resolve(x) < self.resolve(y) {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4
            }
            Instruction::Equals(x, y, dest) => {
                let dest = self.resolve_save(dest) as usize;
                if self.resolve(x) == self.resolve(y) {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4
            }
            Instruction::ModifyRelativeBase(x) => {
                self.rb += self.resolve(x) as isize;
                self.ip += 2;
            }
            Instruction::Exit => return Ok(true),
            _ => {
                return Err(Box::new(ComputeError {}));
            }
        }

        Ok(false)
    }

    fn resolve(&self, x: Parameter) -> i64 {
        match x {
            Parameter::Immediate(x) => x,
            Parameter::Position(x) => self.memory[x],
            Parameter::Relative(x) => self.memory[(self.rb + x) as usize]
        }
    }

    fn resolve_save(&self, x: Parameter) -> i64 {
        match x {
            Parameter::Immediate(x) =>  panic!("wat"),
            Parameter::Position(x) => x as i64,
            Parameter::Relative(x) => {
                (self.rb + x) as i64
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let mut c =
            Computer::new(
                vec![1, 9, 10, 3,
                     2, 3, 11, 0,
                     99,
                     30, 40, 50],
                rx1,
                tx2);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..12], vec![3500, 9, 10, 70,
                                        2, 3, 11, 0,
                                        99,
                                        30, 40, 50][..]);
    }

    #[test]
    fn second_example() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![1, 0, 0, 0, 99], rx, tx);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..5], vec![2, 0, 0, 0, 99][..]);
    }

    #[test]
    fn third_example() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![2, 3, 0, 3, 99], rx, tx);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..5], vec![2, 3, 0, 6, 99][..]);
    }

    #[test]
    fn fourth_example() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![2, 4, 4, 5, 99, 0], rx, tx);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..6], vec![2, 4, 4, 5, 99, 9801][..]);
    }

    #[test]
    fn fifth_example() {
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], rx, tx);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..9], vec![30, 1, 1, 4, 2, 5, 6, 0, 99][..]);
    }

    #[test]
    fn input() {
        let (tx, rx) = crossbeam_channel::unbounded();
        tx.send(10).unwrap();
        let mut c = Computer::new(vec![3, 0, 99], rx, tx);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..3], vec![10, 0, 99][..]);
    }

    #[test]
    fn output() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![4, 0, 99], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(c.memory[..3], vec![4, 0, 99][..]);
        assert_eq!(rx2.recv().unwrap(), 4)
    }

//    #[test]
//    fn immediate_mode() {
//        let (tx1, rx1) = crossbeam_channel::unbounded();
//        let (tx2, rx2) = crossbeam_channel::unbounded();
//        let mut c = Computer::new(vec![101, 2, 1, 0, 99], rx1, tx2);
//        c.run_to_completion().unwrap();
//        assert_eq!(c.memory, vec![3, 2, 1, 0, 99]);
//    }

    #[test]
    fn equals() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        tx1.send(8);
        let mut c = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 1);
    }

    #[test]
    fn equals_immediate() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        tx1.send(8);
        let mut c = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 1);
    }

    #[test]
    fn less_than() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        tx1.send(8);
        let mut c = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 0);
    }

    #[test]
    fn less_than_immediate() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        tx1.send(8);
        let mut c = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 0);
    }

    #[test]
    fn jump() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        tx1.send(40);
        let mut c = Computer::new(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 1);
    }

    #[test]
    fn relative_mode() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 109);
    }

    #[test]
    fn large_num() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![104, 1125899906842624, 99], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 1125899906842624);
    }

    #[test]
    fn large_calc() {
        let (tx1, rx1) = crossbeam_channel::unbounded();
        let (tx2, rx2) = crossbeam_channel::unbounded();
        let mut c = Computer::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], rx1, tx2);
        c.run_to_completion().unwrap();
        assert_eq!(rx2.recv().unwrap(), 1219070632396864);
    }
}