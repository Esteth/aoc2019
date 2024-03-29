use std::boxed::Box;
use std::io::BufRead;
use std::error::Error;
use std::result::Result;

fn fuel_required(mass: u64) -> u64 {
    let fuel = (mass as i64) / 3 - 2;
    if fuel <= 0 {
        return 0
    }
    fuel as u64 + fuel_required(fuel as u64)
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut total_fuel: u64 = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mass: u64 = line.parse()?;
        total_fuel += fuel_required(mass);
    }
    println!("Total Fuel Requirement: {}", total_fuel);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_required_zero() {
        assert_eq!(fuel_required(0), 0);
    }

    #[test]
    fn fuel_required_small() {
        assert_eq!(fuel_required(1), 0);
        assert_eq!(fuel_required(8), 0);
    }

    #[test]
    fn fuel_required_smallest_significant() {
        assert_eq!(fuel_required(9), 1);
    }

    #[test]
    fn fuel_required_large() {
        assert_eq!(fuel_required(1969), 966);
        assert_eq!(fuel_required(100756), 50346);
    }
}