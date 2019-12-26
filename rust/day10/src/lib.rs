use std::error::Error;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use core::fmt;
use std::iter::FromIterator;

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

    fn occupied_positions(&self) -> HashSet<&Position> {
        self.positions.iter()
            .filter_map(|(_, p)| {
                if p.is_empty {
                    None
                } else {
                    Some(p)
                }
            })
            .collect()
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

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(config.filename)?;

    match config.part {
        config::Part::PartOne => {
            let map = parse_input(&input);
            let result = part_one(&map);
            println!("{}", result);
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
            let is_empty = c == '.';
            let position = Position::new(x, y, is_empty);

            map.add(position);

            x += 1;
        }

        y += 1;
    }

    map
}

fn part_one(map: &Map) -> isize {
    let mut max = 0;
    let occupied_positions = map.occupied_positions();
    let asteroid_positions = Vec::from_iter(occupied_positions.iter());

    for i in 0..asteroid_positions.len() {
        let mut count = 0;

        let position = &asteroid_positions[i];
        for other_position in asteroid_positions.iter().filter(|&p| p != position) {
            let is_in_sight = is_in_sight(position, other_position, &occupied_positions);

            if is_in_sight {
                count += 1;
            }
        }

        if count > max {
            max = count;
        }
    }

    max
}

fn is_in_sight(p1: &Position, p2: &Position, occupied_positions: &HashSet<&Position>) -> bool {
    let line = point::Point::line(&p1.point, &p2.point);
    // Filter p1 and p2 out of line
    let line = line
        .iter()
        .filter(|&p| !p.equal_to_point(&p1.point) && !p.equal_to_point(&p2.point))
        .map(|p| p.clone())
        .collect::<Vec<point::FuzzyPoint>>();

    // Filter p1 and p2 out of occupied positions
    let occupied_positions = occupied_positions
        .iter()
        .filter_map(|p| {
            if p == &p1 || p == &p2 {
                None
            } else {
                Some(point::FuzzyPoint::from_point(&p.point))
            }
        })
        .collect::<Vec<point::FuzzyPoint>>();

    for p in line {
        if occupied_positions.contains(&p) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

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

    #[test]
    fn occupied_positions() {
        let input = "\
.#..#
.....
#####
....#
...##";
        let map = parse_input(input);

        let expected: HashSet<Position> = HashSet::from_iter(vec![
            Position::new(1, 0, false),
            Position::new(4, 0, false),

            Position::new(0, 2, false),
            Position::new(1, 2, false),
            Position::new(2, 2, false),
            Position::new(3, 2, false),
            Position::new(4, 2, false),

            Position::new(4, 3, false),

            Position::new(3, 4, false),
            Position::new(4, 4, false),
        ]);

        assert_eq!(expected.iter().collect::<HashSet<&Position>>(), map.occupied_positions());
    }

    #[test]
    fn part_one_example_one_should_return_8() {
        let input = "\
.#..#
.....
#####
....#
...##";
        let map = parse_input(input);

        assert_eq!(8, part_one(&map));
    }

    #[test]
    fn part_one_example_two_should_return_33() {
        let input = "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let map = parse_input(input);

        assert_eq!(33, part_one(&map));
    }

    #[test]
    fn part_one_example_three_should_return_35() {
        let input = "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let map = parse_input(input);

        assert_eq!(35, part_one(&map));
    }

    #[test]
    fn is_in_sight_uppercase_a_should_be_true() {
        let input = "\
#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c";
        let map = parse_input(input);

        let origin = Position::new(0, 0, false);
        let test = Position::new(3, 1, false);

        assert_eq!(true, is_in_sight(&origin, &test, &map.occupied_positions()));
    }

    #[test]
    fn is_in_sight_first_lowercase_a_should_be_false() {
        let input = "\
#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c";
        let map = parse_input(input);

        let origin = Position::new(0, 0, false);
        let test = Position::new(6, 2, false);

        assert_eq!(false, is_in_sight(&origin, &test, &map.occupied_positions()));
    }

    #[test]
    fn is_in_sight_second_lowercase_a_should_be_false() {
        let input = "\
#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c";
        let map = parse_input(input);

        let origin = Position::new(0, 0, false);
        let test = Position::new(9, 3, false);

        assert_eq!(false, is_in_sight(&origin, &test, &map.occupied_positions()));
    }

    #[test]
    fn is_in_sight_uppercase_g_should_be_true() {
        let input = "\
#.........
...A......
...B..a...
.EDCG....a
..F.c.b...
.....c....
..efd.c.gb
.......c..
....f...c.
...e..d..c";
        let map = parse_input(input);

        let origin = Position::new(0, 0, false);
        let test = Position::new(4, 3, false);

        assert_eq!(true, is_in_sight(&origin, &test, &map.occupied_positions()));
    }
}
