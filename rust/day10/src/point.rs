use core::fmt;
use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn line(p1: &Point, p2: &Point) -> Vec<FuzzyPoint> {
        let mut points = Vec::new();
        let distance = Point::diagonal_distance(p1, p2);

        for step in 0..=distance {
            let t = if distance == 0 {
                0.0
            } else {
                step as f64 / distance as f64
            };

            let (x, y) = Point::lerp_point(p1, p2, t);
            points.push(FuzzyPoint { x, y, });
        }

        points
    }

    fn diagonal_distance(p1: &Point, p2: &Point) -> isize {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;

        max(dx.abs(), dy.abs())
    }

    #[allow(dead_code)]
    fn round_point(x: f64, y: f64) -> Point {
        Point::new(x.round() as isize, y.round() as isize)
    }

    fn lerp_point(p1: &Point, p2: &Point, t: f64) -> (f64, f64) {
        let x = Point::lerp(p1.x, p2.x, t);
        let y = Point::lerp(p1.y, p2.y, t);

        (x, y)
    }

    fn lerp(start: isize, end: isize, t: f64) -> f64 {
        start as f64 + t * (end - start) as f64
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FuzzyPoint {
    x: f64,
    y: f64,
}

impl FuzzyPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_point(point: &Point) -> Self {
        Self {
            x: point.x() as f64,
            y: point.y() as f64,
        }
    }

    pub fn equal_to_point(&self, p: &Point) -> bool {
        let floor_x = self.x.floor();
        let floor_y = self.y.floor();

        let same_x = self.x == floor_x && floor_x as isize == p.x;
        let same_y = self.y == floor_y && floor_y as isize == p.y;

        same_x && same_y
    }
}

impl Display for FuzzyPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}
