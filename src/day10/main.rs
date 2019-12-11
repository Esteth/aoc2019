use std::error::Error;
use std::collections::{HashSet, BTreeSet};
use simple_error::SimpleError;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
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
    let field: BTreeSet<Point> = get_field(input);
    let coords =
        field.iter()
            .max_by_key(|Point { x, y }| {
                get_angles(&field, (*x, *y)).len()
            });
    match coords {
        Some(coords) => Ok((coords.x, coords.y)),
        None => Err(Box::new(SimpleError::new("No asteroids found"))),
    }
}

fn get_field(input: &str) -> BTreeSet<Point>{
    (0..)
        .zip(input.lines().map(|l| l.trim()))
        .flat_map(|(y, l)|
            (0..)
                .zip(l.chars())
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point { x, y }))
        .collect()
}

fn get_angles(field: &BTreeSet<Point>, (station_x, station_y): (i32, i32)) -> Vec<(Point, f64)> {
    let mut angles: Vec<_> =
        field.iter()
            .filter(|Point { x: ox, y: oy }| *ox != station_x || *oy != station_y)
            .map(|p| (p, Vector { x: p.x - station_x, y: p.y - station_y }))
            .map(|(p, v)| (*p, v.angle()))
            .collect();
    angles.sort_by(|(_, x), (_, y)| {
        if x < y {
            Ordering::Less
        } else if x > y{
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    angles.dedup_by(|(_, x), (_, y)| (*x - *y).abs() < 1e-5);
    angles
}

fn ith_asteroid(i: i32, (station_x, station_y): (i32, i32), input: &str) -> (i32, i32) {
    let field: BTreeSet<Point> = get_field(input);
    let angles: Vec<(Point, f64)> = get_angles(&field, (station_x, station_y));
    println!("{:?}", angles);

    let mut angle_buckets: Vec<Vec<Point>> = Vec::new();
    let mut last_angle: Option<f64> = None;
    for (p, angle) in angles {
        if let Some(last_angle) = last_angle {
            if (angle - last_angle).abs() < 1e-5 {
                let i = angle_buckets.len();
                angle_buckets.get_mut(i).unwrap().push(p);
            } else {
                angle_buckets.push(vec![p]);
            }
        } else {
            angle_buckets.push(vec![p]);
        }
        last_angle = Some(angle);
    }
    println!("{:?}", angle_buckets);
    let mut destroyed: i32 = 0;
    let mut i: usize = 0 as usize;
    (0, 0)
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

    #[test]
    fn part_2() {
        let input = "###..#.##.####.##..###.#.#..
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
....####...##.#.#...#..#.##.";
        let best_coords = best_asteroid(input).unwrap();
        assert_eq!(best_coords, (22, 19));
        assert_eq!(ith_asteroid(200, best_coords, input), (22, 19));
    }
}