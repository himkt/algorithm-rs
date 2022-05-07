#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(pub i64, pub i64);

impl std::ops::Add for Point {
    type Output=Self;
    fn add(self, _rhs: Point) -> Self {
        Self(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl std::ops::Sub for Point {
    type Output=Self;
    fn sub(self, _rhs: Point) -> Self {
        Self(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl std::ops::Mul<i64> for Point {
    type Output=Self;
    fn mul(self, k: i64) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl std::ops::Div<i64> for Point {
    type Output=Self;
    fn div(self, k: i64) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl Point {
    pub fn det(self, _rhs: Point) -> i64 {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}


#[test]
fn it_works_point () {
    use crate::geometry::point::Point;

    let p1 = Point(4, 6);
    let p2 = Point(6, 4);

    assert_eq!(p1 + p2, Point(10, 10));
    assert_eq!(p1 - p2, Point(-2, 2));
    assert_eq!(p1 * 4, Point(16, 24));
    assert_eq!(p1 / 2, Point(2, 3));
}
