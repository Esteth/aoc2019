use std::error::Error;
use std::io::BufRead;
use crossbeam_channel::{Sender, Receiver};
use std::thread;
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
    let chans: Vec<(Sender<i32>, Receiver<i32>)> =
        phases.iter()
            .map(|p| crossbeam_channel::bounded(1))
            .collect();
    for i in 0..4 {
        let memory = memory.clone();
        chans[i].0.send(phases[i]);
        let rx = chans[i].1.clone();
        let tx = chans[i + 1].0.clone();
        thread::spawn(move || {
            println!("running computer {}", i);
            let mut computer = Computer::new(memory.clone(), rx, tx);
            computer.run_to_completion().unwrap();
            println!("computer {} finished", i);
        });
    }
    chans[0].0.send(phases[4]);
    println!("running final computer");
    let mut computer = Computer::new(memory.clone(), chans[4].1.clone(), chans[0].0.clone());
    computer.run_to_completion()?;
    println!("done");

    let val = chans[0].1.recv()?;
    Ok(val)
}