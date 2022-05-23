use crate::geometry::float_point::FloatPoint;

pub struct FloatLine(pub FloatPoint, pub FloatPoint);

impl FloatLine {
    pub fn contains(&self, p: FloatPoint) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }

    pub fn distance(&self, p: FloatPoint) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        v1.det(v2) / l1
    }
}

#[cfg(test)]
mod test_plane {
    #[test]
    fn it_works_line() {
        use crate::geometry::float_line::FloatLine;
        use crate::geometry::float_point::FloatPoint;

        let p1 = FloatPoint(0.0, 0.0);
        let p2 = FloatPoint(2.0, 3.0);

        let line = FloatLine(p1, p2);

        assert!(line.contains(FloatPoint(4.0,  6.0)));
        assert!(line.contains(FloatPoint(-2.0, -3.0)));
        assert!(!line.contains(FloatPoint(1.0, 1.0)));

        let p3 = FloatPoint(0.0, 3.0);
        let d = line.distance(p3);
        let d_truth = 1.664_100_588_675_687_4;

        let i1 = (d * 1_000_000_000.0) as i64;
        let i2 = (d_truth * 1_000_000_000.0) as i64;
        assert_eq!(i1, i2);
    }
}
