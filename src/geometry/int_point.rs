// Note: Eq and Ord for i64
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct IntPoint(pub i64, pub i64);

impl std::ops::Add for IntPoint {
    type Output = Self;
    fn add(self, _rhs: IntPoint) -> Self {
        Self(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl std::ops::Sub for IntPoint {
    type Output = Self;
    fn sub(self, _rhs: IntPoint) -> Self {
        Self(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl std::ops::Mul<i64> for IntPoint {
    type Output = Self;
    fn mul(self, k: i64) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl std::ops::Div<i64> for IntPoint {
    type Output = Self;
    fn div(self, k: i64) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl IntPoint {
    pub fn det(self, _rhs: IntPoint) -> i64 {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}

#[test]
fn it_works_point() {
    use crate::geometry::int_point::IntPoint;

    let p1 = IntPoint(4, 6);
    let p2 = IntPoint(6, 4);

    assert_eq!(p1 + p2,  IntPoint(10, 10));
    assert_eq!(p1 - p2,  IntPoint(-2, 2));
    assert_eq!(p1 * 4, IntPoint(16, 24));
    assert_eq!(p1 / 2, IntPoint(2,  3));
}
