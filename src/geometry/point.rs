use std::ops::{Add, Sub, Mul, Div};


#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point<T>(pub T, pub T);


impl<T: Add<T, Output=T>> std::ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, _rhs: Point<T>) -> Self::Output {
        Point(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl<T: Sub<T, Output=T>> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, _rhs: Point<T>) -> Self {
        Point(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl<T: Copy + Mul<T, Output=T>> std::ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, k: T) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl<T: Copy + Div<T, Output=T>> std::ops::Div<T> for Point<T> {
    type Output = Self;
    fn div(self, k: T) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl<T: Mul<T, Output=T> + Sub<T, Output=T>> Point<T> {
    pub fn det(self, _rhs: Point<T>) -> T {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}
