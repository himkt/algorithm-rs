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
    use crate::geometry::point::Point;

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
