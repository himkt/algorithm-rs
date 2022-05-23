use crate::geometry::int_point::IntPoint;

pub struct IntLine(pub IntPoint, pub IntPoint);

impl IntLine {
    pub fn contains(&self, p: IntPoint) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

#[cfg(test)]
mod test_plane {
    #[test]
    fn it_works_line() {
        use crate::geometry::int_line::IntLine;
        use crate::geometry::int_line::IntPoint;

        let p1 = IntPoint(0, 0);
        let p2 = IntPoint(2, 3);

        let line = IntLine(p1, p2);

        assert!(line.contains(IntPoint(4,  6)));
        assert!(line.contains(IntPoint(-2, -3)));
        assert!(!line.contains(IntPoint(1, 1)));
    }
}
