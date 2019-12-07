use std::error::Error;
use std::io::{BufRead, Cursor};
use aoc::computer::Computer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;
    let memory: Vec<i32> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();

    // Why is there no permutations() function in the standard library?
    // Do the dumb thing for now.
    let mut max_score = 0;
    for i1 in 0..5 {
        for i2 in 0..5 {
            if i2 == i1 {
                continue;
            }
            for i3 in 0..5 {
                if i3 == i2 || i3 == i1 {
                    continue;
                }
                for i4 in 0..5 {
                    if i4 == i3 || i4 == i2 || i4 == i1 {
                        continue;
                    }
                    for i5 in 0..5 {
                        if i5 == i4 || i5 == i3 || i5 == i2 || i5 == i1 {
                            continue;
                        }
                        let out = run_pipeline(&memory, [i1, i2, i3, i4, i5])?;
                        if out > max_score {
                            max_score = out;
                        }
                    }
                }
            }
        }
    }

    println!("final output: {}", max_score);

    Ok(())
}

fn run_pipeline(memory: &Vec<i32>, phases: [i32; 5]) -> Result<i32, Box<dyn Error>> {
    let mut prev_output = 0;
    for i in 0..5 {
        let mut input = Cursor::new(format!("{}\n{}\n", phases[i], prev_output));
        let mut output = Vec::new();
        let mut computer = Computer::new(memory.clone(), &mut input, &mut output);
        computer.run_to_completion()?;
        prev_output = String::from_utf8(output).unwrap().trim().parse()?
    }
    Ok(prev_output)
}