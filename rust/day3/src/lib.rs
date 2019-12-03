use std::error::Error;
use std::fs;
use std::collections::HashSet;

pub mod config;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Step {
    direction: Direction,
    count: i32,
}

impl Step {
    fn new(direction: Direction, count: i32) -> Self {
        Self { direction, count }
    }
}

impl Step {
    fn all_points(&self, origin: Point) -> HashSet<Point> {
        let mut points = HashSet::new();

        match self.direction {
            Direction::Left => {
                for x in origin.x..=origin.x + self.count {
                    points.insert(Point::new(x, origin.y));
                }
            },
            Direction::Right => {
                for x in origin.x..=origin.x + self.count {
                    points.insert(Point::new(x, origin.y));
                }
            },
            Direction::Up => {
                for x in origin.x..=origin.x + self.count {
                    points.insert(Point::new(x, origin.y));
                }
            },
            Direction::Down => {
                for x in origin.x..=origin.x + self.count {
                    points.insert(Point::new(x, origin.y));
                }
            },
        }

        points
    }
}

struct Wire {
    history: HashSet<Point>,
}

impl Wire {

}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

