use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use core::fmt;

pub mod config;
pub mod point;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    point: point::Point,
    is_empty: bool,
}

impl Position {
    fn new(x: isize, y: isize, is_empty: bool) -> Self {
        Self {
            point: point::Point::new(x, y),
            is_empty
        }
    }
}

#[derive(Debug)]
struct Map {
    positions: HashMap<(isize, isize), Position>,
}

impl Map {
    fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    fn add(&mut self, position: Position) {
        self.positions.insert((position.point.x(), position.point.y()), position);
    }

    fn get(&self, x: isize, y: isize) -> Option<&Position> {
        self.positions.get(&(x, y))
    }

    fn width(&self) -> isize {
        if self.positions.keys().len() == 0 {
            return 0;
        }

        // TODO: Don't unwrap
        let min = self.positions.keys().into_iter().min_by_key(|(x, _)| x).unwrap().0;
        let max = self.positions.keys().into_iter().max_by_key(|(x, _)| x).unwrap().0;

        let width = max - min + 1;

        width.abs()
    }

    fn height(&self) -> isize {
        if self.positions.keys().len() == 0 {
            return 0;
        }

        // TODO: Don't unwrap
        let min = self.positions.keys().into_iter().min_by_key(|(_, y)| y).unwrap().1;
        let max = self.positions.keys().into_iter().max_by_key(|(_, y)| y).unwrap().1;

        let height = max - min + 1;

        height.abs()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            let mut line = String::new();

            for x in 0..self.width() {
                let position = self.get(x, y).unwrap();
                if position.is_empty {
                    line.push('.');
                } else {
                    line.push('#');
                }
            }

            writeln!(f, "{}", line);
        }

        Ok(())
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;

    match config.part {
        config::Part::PartOne => {
            let _map = parse_input(&input);
            // TODO: Part one
        }
        config::Part::PartTwo => {
            // TODO: Part two
        }
    }

    Ok(())
}

fn parse_input(input: &str) -> Map {
    let mut map = Map::new();

    let mut y = 0;
    let lines = input.lines();

    for line in lines {
        let mut x = 0;
        let chars = line.chars();

        for c in chars {
            let is_empty = c != '#';
            let position = Position::new(x, y, is_empty);

            map.add(position);

            x += 1;
        }

        y += 1;
    }

    println!("{}", map.width());
    println!("{}", map.height());

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_map() {
        let input = "\
.#..#
.....
#####
....#
...##";
        let map = parse_input(input);

        assert_eq!(input, map.to_string().trim());
    }
}
