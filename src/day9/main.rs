use std::boxed::Box;
use std::result::Result;
use std::io::BufRead;
use std::error::Error;
use crossbeam_channel::{Sender, Receiver};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;
    let memory: Vec<i64> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();
    let mut stdout = std::io::stdout();

    let (tx1, rx1) = crossbeam_channel::unbounded();
    let (tx2, rx2) = crossbeam_channel::unbounded();
    tx1.send(2);
    let mut computer = aoc::computer::Computer::new(memory, rx1, tx2);
    computer.run_to_completion()?;
    loop {
        let val = rx2.recv()?;
        println!("{}", val);
    }
    Ok(())
}