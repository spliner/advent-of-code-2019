use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn line(p1: &Point, p2: &Point) -> HashSet<Point> {
        let mut points = HashSet::new();
        let distance = Point::diagonal_distance(p1, p2);

        for step in 0..=distance {
            let t = if distance == 0 {
                0.0
            } else {
                step as f64 / distance as f64
            };

            let (x, y) = Point::lerp_point(p1, p2, t);
            let rounded_point = Point::round_point(x, y);

            points.insert(rounded_point);
        }

        points
    }

    fn diagonal_distance(p1: &Point, p2: &Point) -> isize {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;

        max(dx.abs(), dy.abs())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn line_test_1() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(4, 0);

        let expected = HashSet::from_iter(vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
        ]);

        assert_eq!(expected, Point::line(&p1, &p2));
    }

    #[test]
    fn line_test_2() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(11, 4);

        let expected = HashSet::from_iter(vec![
            Point::new(0, 0),
            Point::new(1, 0),

            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(4, 1),

            Point::new(5, 2),
            Point::new(6, 2),

            Point::new(7, 3),
            Point::new(8, 3),
            Point::new(9, 3),

            Point::new(10, 4),
            Point::new(11, 4),
        ]);

        assert_eq!(expected, Point::line(&p1, &p2));
    }
}
