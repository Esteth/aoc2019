use std::boxed::Box;
use std::result::Result;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Day 3");
    Ok(())
}