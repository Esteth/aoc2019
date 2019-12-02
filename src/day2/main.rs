use std::boxed::Box;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World!");
    Ok(())
}