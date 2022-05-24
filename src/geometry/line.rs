use crate::geometry::point::Point;


pub struct Line<T>(pub Point<T>, pub Point<T>);


pub trait LineDistance<T> {
    fn distance(&self, p: Point<T>) -> f64;
}

impl LineDistance<i64> for Line<i64> {
    fn distance(&self, p: Point<i64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = ((v1.0 * v1.0 + v1.1 * v1.1) as f64).sqrt();
        let det = v1.det(v2) as f64;
        det / l1
    }
}

impl LineDistance<f64> for Line<f64> {
    fn distance(&self, p: Point<f64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let det = v1.det(v2);
        det / l1
    }
}

pub trait LineContain<T> {
    fn contains_point(&self, p: Point<T>) -> bool;
}

impl LineContain<f64> for Line<f64> {
    fn contains_point(&self, p: Point<f64>) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}


#[cfg(test)]
mod test_plane {
    use crate::geometry::line::Line;
    use crate::geometry::line::Point;
    use crate::geometry::line::LineDistance;
    use crate::geometry::line::LineContain;

    #[test]
    fn it_works_line_f64() {
        let p1 = Point(0.0, 0.0);
        let p2 = Point(2.0, 3.0);

        let line = Line(p1, p2);

        let p3 = Point(0.0, 3.0);
        let d = line.distance(p3);
        let d_truth = 1.664_100_588_675_687_4;

        let i1 = (d * 1_000_000_000.0) as i64;
        let i2 = (d_truth * 1_000_000_000.0) as i64;
        assert_eq!(i1, i2);

        let line = Line(p1, p2);

        assert!(line.contains_point(Point(4.0,  6.0)));
        assert!(line.contains_point(Point(-2.0, -3.0)));
        assert!(!line.contains_point(Point(1.0, 1.0)));
    }

    #[test]
    fn it_works_line_i64() {
        let p3 = Point(0, 0);
        let p4 = Point(2, 3);
        let line = Line(p3, p4);

        let p5 = Point(0, 3);
        let d = line.distance(p5);
        let d_truth = 1.664_100_588_675_687_4;

        let i3 = (d * 1_000_000_000.0) as i64;
        let i4 = (d_truth * 1_000_000_000.0) as i64;
        assert_eq!(i3, i4);
    }
}
