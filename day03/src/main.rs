use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use std::collections::HashSet;

use std::convert::{From, Into};

fn main() {
    let file = File::open("res/input.txt").unwrap();
    let mut bufreader = BufReader::new(file);

    let mut line = String::new();
    bufreader.read_line(&mut line).unwrap();
    let first_wire = Wire::from(line.trim_end());

    let first_wire = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    
    line.clear();
    bufreader.read_line(&mut line).unwrap();
    let second_wire = Wire::from(line.trim_end());

    let second_wire = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");

    let first_wire_points: HashSet<_> = first_wire.points.iter().map(|p| p.clone()).collect();
    let second_wire_points: HashSet<_> = second_wire.points.iter().map(|p| p.clone()).collect();
    let mut intersections: Vec<Point> = first_wire_points.intersection(&second_wire_points).map(|p| p.clone()).collect();

    intersections.sort_by(|a, b| (a.0.abs() + a.1.abs()).cmp(&(b.0.abs() + b.1.abs())));
    let closest = intersections.first().unwrap();
    println!("Part 1: {}", closest.0.abs() + closest.1.abs());

    let mut point_dists: Vec<usize> = intersections.iter().map(|p|
                                        first_wire.points.iter().position(|x| x == p).unwrap() +
                                        second_wire.points.iter().position(|x| x == p).unwrap())
        .collect::<Vec<usize>>();
    point_dists.sort();
    println!("Part 2: {}", point_dists.first().unwrap() + 2);
}

struct Wire {
    end: Point,
    points: Vec<Point>,
}

impl Wire {
     pub fn new() -> Wire {
         Wire { end: Point(0,0), points: Vec::new(), }
     }
}

impl From<&str> for Wire {
    fn from(input: &str) -> Self {
        let mut wire = Wire::new();
        let movements: Vec<Movement> = input.split(',').map(|s| Movement::from(s)).collect();
        for movement in movements.into_iter() {
            let points: Vec<Point> = Movement::into(movement);
            let end_adjustment = points.last().unwrap().clone();
            let mut points: Vec<Point> = points.into_iter().map(|point| point + wire.end.clone()).collect();
            wire.points.append(&mut points);
            wire.end = wire.end + end_adjustment;
        }
        wire
    }
}


#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point(i64, i64);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(PartialEq, Debug)]
enum Movement {
    Right(u32),
    Down(u32),
    Left(u32),
    Up(u32)
}

impl From<&str> for Movement {
    fn from(input: &str) -> Self {
        let (dir, dist) = input.split_at(1);
        let dist = dist.parse::<u32>().unwrap();
        match dir {
            "U" => Movement::Up(dist),
            "D" => Movement::Down(dist),
            "R" => Movement::Right(dist),
            "L" => Movement::Left(dist),
            c => panic!("Invalid direction {}", c),
        }
    }
}

impl Into<Vec<Point>> for Movement {
    fn into(self) -> Vec<Point> {
        match self {
            Movement::Right(dist) => std::ops::Range { start: 1, end: (dist as i64) + 1 }.map(|n| Point(n, 0)).collect(),
            Movement::Down(dist) => std::ops::Range { start: 1, end: (dist as i64) + 1 }.map(|n| Point(0, -1 * n)).collect(),
            Movement::Left(dist) => std::ops::Range { start: 1, end: (dist as i64) + 1 }.map(|n| Point(-1 * n, 0)).collect(),
            Movement::Up(dist) => std::ops::Range { start: 1, end: (dist as i64) + 1 }.map(|n| Point(0, n)).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn movement_generation() {
        assert_eq!(Movement::Up(25), Movement::from("U25"));
        assert_eq!(Movement::Down(15), Movement::from("D15"));
        assert_eq!(Movement::Right(99), Movement::from("R99"));
        assert_eq!(Movement::Left(99999), Movement::from("L99999"));
    }

    #[test]
    #[should_panic]
    fn movement_generation_invalid_input() {
        Movement::from("UU25");
    }

    #[test]
    fn parse_input_to_vec() {
        let actual: Vec<Point> = Movement::into(Movement::Up(3));
        assert_eq!(vec![Point(0,1), Point(0,2), Point(0,3)], actual);
        let actual: Vec<Point> = Movement::into(Movement::Down(3));
        assert_eq!(vec![Point(0,-1), Point(0,-2), Point(0,-3)], actual);
        let actual: Vec<Point> = Movement::into(Movement::Right(3));
        assert_eq!(vec![Point(1,0), Point(2,0), Point(3,0)], actual);
        let actual: Vec<Point> = Movement::into(Movement::Left(3));
        assert_eq!(vec![Point(-1,0), Point(-2,0), Point(-3,0)], actual);
    }

    #[test]
    fn build_wire_from_str() {
        let actual = Wire::from("U3");
        assert_eq!(Point(0,3), actual.end);
        let actual = Wire::from("U3,R1");
        assert_eq!(Point(1,3), actual.end);
        let actual = Wire::from("U3,R1,D3");
        assert_eq!(Point(1,0), actual.end);
        let mut expected_points = Vec::new();
        expected_points.push(Point(0,1));
        expected_points.push(Point(0,2));
        expected_points.push(Point(0,3));
        expected_points.push(Point(1,3));
        expected_points.push(Point(1,2));
        expected_points.push(Point(1,1));
        expected_points.push(Point(1,0));
        assert_eq!(expected_points, actual.points);
    }
}
