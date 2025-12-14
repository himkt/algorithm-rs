use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

// FIXME (himkt): Eq for Point<f64> won't work as expected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point<T>(pub T, pub T);

impl<T: Add<T, Output = T>> std::ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, _rhs: Point<T>) -> Self::Output {
        Point(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl<T: Sub<T, Output = T>> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, _rhs: Point<T>) -> Self {
        Point(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl<T: Copy + Mul<T, Output = T>> std::ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, k: T) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl<T: Copy + Div<T, Output = T>> std::ops::Div<T> for Point<T> {
    type Output = Self;
    fn div(self, k: T) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl<T: Mul<T, Output = T> + Sub<T, Output = T>> Point<T> {
    pub fn det(self, _rhs: Point<T>) -> T {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}

#[cfg(test)]
mod test_point {
    use crate::geometry::Point;

    #[test]
    fn it_works_i64() {
        let p1: Point<i64> = Point(4, 6);
        let p2 = Point(6, 4);

        assert_eq!(p1 + p2, Point(10, 10));
        assert_eq!(p1 - p2, Point(-2, 2));
        assert_eq!(p1 * 4, Point(16, 24));
        assert_eq!(p1 / 2, Point(2, 3));
    }

    #[test]
    fn it_works_f64() {
        let p1: Point<f64> = Point(4.0, 6.0);
        let p2 = Point(6.0, 4.0);

        assert_eq!(p1 + p2, Point(10.0, 10.0));
        assert_eq!(p1 - p2, Point(-2.0, 2.0));
        assert_eq!(p1 * 4.0, Point(16.0, 24.0));
        assert_eq!(p1 / 2.0, Point(2.0, 3.0));
    }
}

pub struct Line<T>(pub Point<T>, pub Point<T>);

pub trait LineAPI<T> {
    fn distance(&self, p: Point<T>) -> f64;
    fn contains_point(&self, p: Point<T>) -> bool;
}

// i64
impl LineAPI<i64> for Line<i64> {
    fn distance(&self, p: Point<i64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = ((v1.0 * v1.0 + v1.1 * v1.1) as f64).sqrt();
        let det = v1.det(v2) as f64;
        det / l1
    }

    fn contains_point(&self, p: Point<i64>) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

// f64
impl LineAPI<f64> for Line<f64> {
    fn distance(&self, p: Point<f64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let det = v1.det(v2);
        det / l1
    }

    fn contains_point(&self, p: Point<f64>) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

#[cfg(test)]
mod test_line {
    use crate::geometry::Line;
    use crate::geometry::LineAPI;
    use crate::geometry::Point;

    #[test]
    fn it_works_line_f64() {
        let x = Point(0.0, 0.0);
        let y = Point(2.0, 3.0);
        let line = Line(x, y);

        let z = Point(0.0, 3.0);
        let d = line.distance(z);
        let d_truth = 1.664_100_588_675_687_4;

        let result = (d * 1_000_000_000.0) as i64;
        let expected = (d_truth * 1_000_000_000.0) as i64;
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_line_i64() {
        let x = Point(0, 0);
        let y = Point(2, 3);
        let line = Line(x, y);

        let z = Point(0, 3);
        let d = line.distance(z);
        let d_truth = 1.664_100_588_675_687_4;

        let result = (d * 1_000_000_000.0) as i64;
        let expected = (d_truth * 1_000_000_000.0) as i64;
        assert_eq!(result, expected);

        // contain
        assert!(line.contains_point(Point(4, 6)));
        assert!(line.contains_point(Point(-2, -3)));
        assert!(!line.contains_point(Point(1, 1)));
    }
}

pub fn convex_hull(ps: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let n = ps.len();

    let mut ps: Vec<Point<i64>> = ps.iter().map(|&(x, y)| Point::<i64>(x, y)).collect();

    ps.sort();

    let mut k = 0;
    let mut deque: VecDeque<Point<i64>> = VecDeque::new();

    for &pi in &ps {
        while k > 1 && (deque[k - 1] - deque[k - 2]).det(pi - deque[k - 1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let t = k;
    for i in (0..n - 1).rev() {
        let pi = ps[i];
        while k > t && (deque[k - 1] - deque[k - 2]).det(pi - deque[k - 1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let mut ret: Vec<(i64, i64)> = deque.into_iter().take(k - 1).map(|pair| (pair.0, pair.1)).collect();

    ret.sort_unstable();
    ret
}

#[cfg(test)]
mod test_convex_hull {
    use crate::geometry::convex_hull;

    #[test]
    fn it_works() {
        let ps = vec![(0, 0), (2, 2), (3, 1), (1, 4), (4, 4)];
        let hull = convex_hull(ps);
        assert_eq!(hull, vec![(0, 0), (1, 4), (3, 1), (4, 4)]);

        let ps = vec![(0, 0), (2, 2), (4, 4)];
        let hull = convex_hull(ps);
        assert_eq!(hull, vec![(0, 0), (4, 4)]);

        let ps = vec![(0, 0), (0, 1), (0, 2), (1, 1)];
        let hull = convex_hull(ps);
        assert_eq!(hull, vec![(0, 0), (0, 2), (1, 1)]);
    }
}
