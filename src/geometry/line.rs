use crate::geometry::point::Point;

pub struct Line(pub Point, pub Point);

impl Line {
    pub fn contains(&self, p: Point) -> bool {
        let d = self.1 - self.0;
        let e = p      - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

#[cfg(test)]
mod test_plane {
    #[test]
    fn it_works_line () {
        use crate::geometry::line::Line;
        use crate::geometry::line::Point;

        let p1 = Point(0, 0);
        let p2 = Point(2, 3);

        let line = Line(p1, p2);

        assert!(line.contains(Point(4, 6)));
        assert!(line.contains(Point(-2, -3)));
        assert!(line.contains(Point(1, 1)));
    }
}
