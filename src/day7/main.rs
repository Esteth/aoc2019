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
    for i1 in 5..10 {
        for i2 in 5..10 {
            if i2 == i1 {
                continue;
            }
            for i3 in 5..10 {
                if i3 == i2 || i3 == i1 {
                    continue;
                }
                for i4 in 5..10 {
                    if i4 == i3 || i4 == i2 || i4 == i1 {
                        continue;
                    }
                    for i5 in 5..10 {
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

fn run_pipeline(memory: &Vec<i32>, phases: [i32; 5]) -> Result<(i32), Box<dyn Error>> {
    let mut cursors: Vec<Cursor<Vec<u8>>> =
        phases.iter()
            .map(|p| Cursor::new(format!("{}\n", p).into_bytes()))
            .collect();
    for i in 0..5 {
        let mut computer = Computer::new(memory.clone(), &mut cursors[i], &mut cursors[(i + 1) % 5]);
        computer.run_to_completion()?;
    }

    cursors[0].set_position(0);
    let mut lines: Vec<String> = cursors[0].lines().map(|line| { line.unwrap() }).collect();
    let val: i32 = lines[lines.len() - 1].trim().parse().unwrap();
    Ok(val)
}