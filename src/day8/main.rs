use std::error::Error;
use std::str::FromStr;
use simple_error::SimpleError;
use std::io::BufRead;

const LAYER_LEN: usize = 25 * 6;

// TODO: Remove this, just use the slices directly. Might need in part2 anyway though.
struct Layer {
    pixels: Vec<u8>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;

    let fewest_zeroes: Layer =
        input.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .chunks(LAYER_LEN)
            .map(|chunk| Layer {pixels: chunk.to_vec()})
            .min_by_key(|l| l.pixels.iter().filter(|p| **p == 0).count())
            .unwrap();
    let ones_count = fewest_zeroes.pixels.iter().filter(|p| **p == 1).count();
    let twos_count = fewest_zeroes.pixels.iter().filter(|p| **p == 2).count();
    println!("{}", ones_count * twos_count);
    Ok(())
}