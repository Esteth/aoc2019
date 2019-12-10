use std::error::Error;
use std::collections::{HashSet, BTreeSet};
use simple_error::SimpleError;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn angle(&self) -> f64 {
        (self.y as f64).atan2((self.x as f64))
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.angle() - other.angle()).abs() < 1e-5
    }
}

impl Eq for Vector {}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.angle().partial_cmp(&other.angle()) {
            Some(x) => x,
            None => Ordering::Equal,
        }
    }
}

fn best_asteroid(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let field: BTreeSet<Point> =
        (0..)
            .zip(input.lines().map(|l| l.trim()))
            .flat_map(|(y, l)|
                (0..)
                    .zip(l.chars())
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| Point { x, y }))
            .collect();
    let coords =
        field.iter()
            .max_by_key(|Point { x, y }| {
                let mut angles: Vec<_> =
                    field.iter()
                        .filter(|Point { x: ox, y: oy }| *ox != *x || *oy != *y)
                        .map(|Point { x: ox, y: oy }| Vector { x: *ox - *x, y: *oy - *y })
                        .map(|p| p.angle())
                        .collect();
                angles.sort_by(|x, y| {
                    if x < y {
                        Ordering::Less
                    } else if x > y{
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });
                angles.dedup_by(|x, y| (*x - *y).abs() < 1e-5);
                println!("{}, {}: {:?}", x, y, angles.len());
                angles.len()
            });
    match coords {
        Some(coords) => Ok((coords.x, coords.y)),
        None => Err(Box::new(SimpleError::new("No asteroids found"))),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Day 10.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_parallel() {
        let vec1 = Vector { x: 0, y: -2 };
        let vec2 = Vector { x: 0, y: -1 };
        assert_eq!(vec1.eq(&vec2), true);

        let vec1 = Vector { x: 1, y: 1 };
        let vec2 = Vector { x: 4, y: 4 };
        assert_eq!(vec1.eq(&vec2), true);
    }

    #[test]
    fn vector_ord() {
        let vec1 = Vector { x: 0, y: -2 };
        let vec2 = Vector { x: 0, y: -1 };
        assert_eq!(vec1.eq(&vec2), true);

        let vec1 = Vector { x: 1, y: 1 };
        let vec2 = Vector { x: 4, y: 4 };
        assert_eq!(vec1.eq(&vec2), true);
    }

    #[test]
    fn first_example() {
        assert_eq!(
            best_asteroid(
                ".#..#
                .....
                #####
                ....#
                ...##")
                .unwrap(),
            (3, 4));
    }

    #[test]
    fn second_example() {
        assert_eq!(
            best_asteroid(
                "......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####")
                .unwrap(),
            (5, 8));
    }

    #[test]
    fn third_example() {
        assert_eq!(
            best_asteroid(
                "#.#...#.#.
                .###....#.
                .#....#...
                ##.#.#.#.#
                ....#.#.#.
                .##..###.#
                ..#...##..
                ..##....##
                ......#...
                .####.###.")
                .unwrap(),
            (1, 2));
    }

    #[test]
    fn fourth_example() {
        assert_eq!(
            best_asteroid(
                ".#..#..###
                ####.###.#
                ....###.#.
                ..###.##.#
                ##.##.#.#.
                ....###..#
                ..#.#..#.#
                #..#.#.###
                .##...##.#
                .....#.#..")
                .unwrap(),
            (6, 3));
    }

    #[test]
    fn fifth_example() {
        assert_eq!(
            best_asteroid(
                ".#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##")
                .unwrap(),
            (11, 13));
    }

    #[test]
    fn part_1() {
        assert_eq!(
            best_asteroid(
                "###..#.##.####.##..###.#.#..
#..#..###..#.......####.....
#.###.#.##..###.##..#.###.#.
..#.##..##...#.#.###.##.####
.#.##..####...####.###.##...
##...###.#.##.##..###..#..#.
.##..###...#....###.....##.#
#..##...#..#.##..####.....#.
.#..#.######.#..#..####....#
#.##.##......#..#..####.##..
##...#....#.#.##.#..#...##.#
##.####.###...#.##........##
......##.....#.###.##.#.#..#
.###..#####.#..#...#...#.###
..##.###..##.#.##.#.##......
......##.#.#....#..##.#.####
...##..#.#.#.....##.###...##
.#.#..#.#....##..##.#..#.#..
...#..###..##.####.#...#..##
#.#......#.#..##..#...#.#..#
..#.##.#......#.##...#..#.##
#.##..#....#...#.##..#..#..#
#..#.#.#.##..#..#.#.#...##..
.#...#.........#..#....#.#.#
..####.#..#..##.####.#.##.##
.#.######......##..#.#.##.#.
.#....####....###.#.#.#.####
....####...##.#.#...#..#.##.")
                .unwrap(),
            (22, 19));
    }
}