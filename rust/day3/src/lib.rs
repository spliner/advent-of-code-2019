use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::config::Part;

pub mod config;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(String::from(format!("Invalid value: {}", s)))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Movement {
    direction: Direction,
    count: i32,
}

impl Movement {
    fn new(direction: Direction, count: i32) -> Self {
        Self { direction, count }
    }
}

impl Movement {
    fn all_points(&self, origin: &Point) -> Vec<Point> {
        let mut points = Vec::new();

        match self.direction {
            Direction::Up => {
                for i in 1..=self.count {
                    points.push(Point::new(origin.x, origin.y + i));
                }
            },
            Direction::Down => {
                for i in 1..=self.count {
                    points.push(Point::new(origin.x, origin.y - i));
                }
            },
            Direction::Left => {
                for i in 1..=self.count {
                    points.push(Point::new(origin.x - i, origin.y));
                }
            },
            Direction::Right => {
                for i in 1..=self.count {
                    points.push(Point::new(origin.x + i, origin.y));
                }
            },
        }

        points
    }
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s[0..1].parse::<Direction>().unwrap();
        let count = s[1..].parse::<i32>().unwrap();
        Ok(Self::new(direction, count))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Wire {
    current_position: Point,
    path: Vec<Point>,
}

impl Wire {
    fn new(initial_position: Point) -> Self {
        Self {
            current_position: initial_position.clone(),
            path: vec![initial_position.clone()]
        }
    }

    fn apply(&mut self, movement: &Movement) {
        for p in movement.all_points(&self.current_position) {
            self.path.push(p.clone());
        }

        if self.path.len() > 0 {
            self.current_position = self.path[self.path.len() - 1].clone();
        }
    }

    fn steps_to(&self, point: &Point) -> Option<usize> {
        for (i, p) in self.path.iter().enumerate() {
            if p == point {
                return Some(i);
            }
        }

        None
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let input = parse_input(&contents);

    match config.part {
        Part::Part1 => {
            let result = part1(&input);
            println!("{:?}", result);
        },
        Part::Part2 => {
            let result = part2(&input);
            println!("{:?}", result);
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> (Point, Vec<Wire>) {
    let origin = Point::new(0, 0);
    let wires = input
        .trim()
        .lines()
        .map(|l| {
            let mut wire = Wire::new(origin.clone());
            let movements: Vec<Movement> = l.split(",")
                .map(|s| {
                    s.trim().parse::<Movement>().unwrap()
                })
                .collect();

            for movement in movements {
                wire.apply(&movement);
            }

            wire
        })
        .collect();

    (origin, wires)
}

fn part1(input: &(Point, Vec<Wire>)) -> i32 {
    let (origin, wires) = input;

    let sets: Vec<HashSet<Point>> = wires
        .iter()
        .map(|w| HashSet::from_iter(w.path.iter().cloned()))
        .collect();

    let s1 = &sets[0];
    let s2 = &sets[1];

    s1.intersection(s2)
        .filter(|p| p.x != 0 && p.y != 0)
        .map(|p| {
            (origin.x - p.x).abs() + (origin.y - p.y).abs()
        })
        .min()
        .unwrap()

}

fn part2(input: &(Point, Vec<Wire>)) -> usize {
    let (_, wires) = input;

    let wire1 = &wires[0];
    let wire2 = &wires[1];

    let s1: HashSet<Point> = HashSet::from_iter(wire1.path.iter().cloned());
    let s2: HashSet<Point> = HashSet::from_iter(wire2.path.iter().cloned());

    s1.intersection(&s2)
        .filter(|p| p.x != 0 && p.y != 0)
        .map(|p| {
            wire1.steps_to(p).unwrap() + wire2.steps_to(p).unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_parse() {
        let input = "U56";
        let movement = input.parse::<Movement>();

        let expected = Movement::new(Direction::Up, 56);
        assert_eq!(Ok(expected), movement);
    }

    #[test]
    fn part1_example_1_should_return_6() {
        let input = "\
R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(6, part1(&parse_input(&input)));
    }

    #[test]
    fn part1_example_2_should_return_159() {
        let input = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(159, part1(&parse_input(&input)));
    }

    #[test]
    fn part1_example_3_should_return_135() {
        let input = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(135, part1(&parse_input(&input)));
    }

    #[test]
    fn part2_example_1_should_return_30() {
        let input = "\
R8,U5,L5,D3
U7,R6,D4,L4";
        assert_eq!(30, part2(&parse_input(&input)));
    }

    #[test]
    fn part2_example_2_should_return_610() {
        let input = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(610, part2(&parse_input(&input)));
    }

    #[test]
    fn part2_example_3_should_return_410() {
        let input = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(410, part2(&parse_input(&input)));
    }
}
