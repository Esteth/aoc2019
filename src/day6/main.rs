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

fn path_to_root(parents: &HashMap<String, String>, node: &String) -> HashMap<String, i32> {
    let mut path = HashMap::new();
    let mut i = 0;
    let mut node = node;
    while *node != "COM".to_string() {
        path.insert(node.to_string(), i);
        i += 1;
        node = parents.get(node).unwrap();
    }
    path
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

    let path_you = path_to_root(&parents, &"YOU".to_string());
    let mut node = &"SAN".to_string();
    let mut i = 0;
    loop {
        if path_you.contains_key(node) {
            println!("{}", path_you.get(node).unwrap() + i - 2);
            break;
        }
        i += 1;
        node = parents.get(node).unwrap();
    }

    Ok(())
}