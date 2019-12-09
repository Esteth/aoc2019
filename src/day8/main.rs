use std::error::Error;
use std::str::FromStr;
use simple_error::SimpleError;
use std::io::BufRead;

const WIDTH: usize = 25;
const HEIGHT:usize = 6;
const LAYER_LEN: usize = WIDTH * HEIGHT;
const TRANSPARENT_LAYER_ARR: [u8; LAYER_LEN] = [2; LAYER_LEN];
const TRANSPARENT_LAYER: Layer = Layer {
    pixels: &TRANSPARENT_LAYER_ARR,
};

// TODO: Remove this, just use the slices directly.
struct Layer<'a> {
    pixels: &'a [u8],
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut stdin_locked = stdin.lock();
    stdin_locked.read_line(&mut input)?;

    let input =
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
    let layers: Vec<Layer> =
        input
            .chunks(LAYER_LEN)
            .map(|chunk| Layer {pixels: chunk})
            .collect();

    let mut image = TRANSPARENT_LAYER_ARR;
    for layer in layers {
        for (i, pixel) in layer.pixels.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *pixel;
            }
        }
    }

    image.chunks(WIDTH).for_each(|line| {
        for p in line {
            if *p == 1 {
                print!("#")
            } else if *p == 0 {
                print!(".")
            } else {
                print!(" ")
            }
        }
        print!("\n")
    });

    Ok(())
}