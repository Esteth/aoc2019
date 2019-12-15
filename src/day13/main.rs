#[macro_use]
extern crate crossbeam_channel;

use std::error::Error;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::thread;
use crossbeam_channel::{RecvError, TryRecvError, Select};
use simple_error::SimpleError;
use crossbeam_channel::internal::select;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Pixel {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;
    let memory: Vec<i64> =
        input.split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();

    let mut panels: HashMap<(i32, i32), bool> = HashMap::new();

    let (input_sender, input_receiver) = crossbeam_channel::bounded(1);
    let (output_sender, output_receiver) = crossbeam_channel::unbounded();
    let (input_request_sender, input_request_receiver) = crossbeam_channel::bounded(1);
    thread::spawn(move || {
        let mut computer = aoc::computer::Computer::new(memory, input_receiver, input_request_sender, output_sender);
        computer.run_to_completion();
    });

    let mut pixels: HashMap<(i64, i64), Pixel> = HashMap::new();
    let mut score = 0;
    loop {
        let mut sel = Select::new();
        let in_req_op = sel.recv(&input_request_receiver);
        let out_op = sel.recv(&output_receiver);
        let oper = sel.select();
        match oper.index() {
            i if i == in_req_op => {
                let msg = oper.recv(&input_request_receiver);
                match msg {
                    Ok(_) => {
                        // TODO: get ball and paddle position
                        input_sender.send(-1);
                    }
                    Err(_) => break,
                }
            }
            i if i == out_op => {
                let msg = oper.recv(&output_receiver);

                let x = match msg {
                    Ok(x) => x,
                    Err(_) => break,
                };
                let y = output_receiver.recv()?;
                if x == -1 && y == 0 {
                    score = output_receiver.recv()?;
                } else {
                    let pixel = match output_receiver.recv() {
                        Ok(0) => Pixel::Empty,
                        Ok(1) => Pixel::Wall,
                        Ok(2) => Pixel::Block,
                        Ok(3) => Pixel::Paddle,
                        Ok(4) => Pixel::Ball,
                        _ => break,
                    };
                    pixels.insert((x, y), pixel);
                }
            }
            _ => unreachable!(),
        }
    }

//    let mut blocks_count = pixels.iter()
//        .filter(|(p, pixel)| **pixel == Pixel::Block)
//        .count();
    println!("score: {}", score);
//        .map(|(p, _)| *p)
//        .collect();
//    let min_x = white_panels.iter().min_by_key(|(x, _)| *x).unwrap().0;
//    let max_x = white_panels.iter().max_by_key(|(x, _)| *x).unwrap().0;
//    let min_y = white_panels.iter().min_by_key(|(_, y)| *y).unwrap().1;
//    let max_y = white_panels.iter().max_by_key(|(_, y)| *y).unwrap().1;
//
//    for y in (min_y..=max_y).rev() {
//        for x in min_x..=max_x {
//            if white_panels.contains(&(x, y)) {
//                print!("█");
//            } else {
//                print!(" ");
//            }
//        }
//        print!("\n");
//    }
    Ok(())
}