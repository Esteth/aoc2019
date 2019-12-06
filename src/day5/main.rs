use std::boxed::Box;
use std::result::Result;
use std::io::BufRead;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;
    let memory: Vec<i32> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();
    let mut stdout = std::io::stdout();

    let mut computer = aoc::computer::Computer::new(memory, &mut stdin_locked, &mut stdout);
    computer.run_to_completion()?;
    Ok(())
}