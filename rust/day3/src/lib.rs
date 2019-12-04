use std::collections::HashSet;
use std::error::Error;
use std::fs;
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
                for i in 0..=self.count {
                    points.push(Point::new(origin.x, origin.y + i));
                }
            },
            Direction::Down => {
                for i in 0..=self.count {
                    points.push(Point::new(origin.x, origin.y - i));
                }
            },
            Direction::Left => {
                for i in 0..=self.count {
                    points.push(Point::new(origin.x - i, origin.y));
                }
            },
            Direction::Right => {
                for i in 0..=self.count {
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

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let input = parse_input(&contents);

    match config.part {
        Part::Part1 => {
            let result = part1(&input);
            println!("{:?}", result);
        },
        Part::Part2 => {
            // TODO: Part 2
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> (Point, Vec<Vec<Movement>>) {
    let origin = Point::new(0, 0);
    let movements = input
        .trim()
        .lines()
        .map(|l| {
            l
                .split(",")
                .map(|s| s.trim().parse::<Movement>().unwrap())
                .collect()
        })
        .collect();
    (origin, movements)
}

fn part1(input: &(Point, Vec<Vec<Movement>>)) -> i32 {
    let (origin, movements) = input;
    let mut sets = movements
        .iter()
        .map(|movements| {
            let mut current_origin = origin.clone();
            let mut history: HashSet<Point> = HashSet::new();

            for movement in movements {
                let points = movement.all_points(&current_origin);
                let next_origin = points[points.len() - 1].clone();

                for point in points {
                    history.insert(point);
                }

                current_origin = next_origin;
            }

            history
        });

    let s1 = sets.next().unwrap();
    let s2 = sets.next().unwrap();

    s1.intersection(&s2)
        .filter(|p| p.x != 0 && p.y != 0)
        .map(|p| {
            p.x.abs() + p.y.abs()
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
}
