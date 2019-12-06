use std::error::Error;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

fn distance_from_root(parents: &HashMap<String, String>, node: &String) -> i32 {
    if *node == "COM".to_string() {
        0
    } else {
        1 + distance_from_root(parents, parents.get(node).unwrap())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut parents: HashMap<String, String> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let input: Vec<&str> = line.splitn(2, ')').collect();
        let orbitee = input[0];
        let orbiter = input[1];
        parents.insert(orbiter.to_string(), orbitee.to_string());
    }

    let total: i32 = parents.keys().map(|node| distance_from_root(&parents, node)).sum();

    println!("{:?}", total);
    Ok(())
}