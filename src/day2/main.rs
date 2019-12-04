use std::boxed::Box;
use std::result::Result;
use std::io::BufRead;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().lock().read_line(&mut input)?;
    let memory: Vec<usize> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let mut computer = aoc::computer::Computer::new(memory);
            let result = computer.run_to_completion()?;
            if result == 19690720 {
                println!("noun={}, verb={}", noun, verb);
                return Ok(())
            }
        }
    }
    println!("could not find valid inputs");
    Ok(())
}