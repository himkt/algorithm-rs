#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatPoint(pub f64, pub f64);

impl std::ops::Add for FloatPoint {
    type Output = Self;
    fn add(self, _rhs: FloatPoint) -> Self {
        Self(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl std::ops::Sub for FloatPoint {
    type Output = Self;
    fn sub(self, _rhs: FloatPoint) -> Self {
        Self(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl std::ops::Mul<f64> for FloatPoint {
    type Output = Self;
    fn mul(self, k: f64) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl std::ops::Div<f64> for FloatPoint {
    type Output = Self;
    fn div(self, k: f64) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl FloatPoint {
    pub fn det(self, _rhs: FloatPoint) -> f64 {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}

#[test]
fn it_works_point() {
    use crate::geometry::float_point::FloatPoint;

    let p1 = FloatPoint(4.0, 6.0);
    let p2 = FloatPoint(6.0, 4.0);

    assert_eq!(p1 + p2,  FloatPoint(10.0, 10.0));
    assert_eq!(p1 - p2,  FloatPoint(-2.0, 2.0));
    assert_eq!(p1 * 4.0, FloatPoint(16.0, 24.0));
    assert_eq!(p1 / 2.0, FloatPoint(2.0,  3.0));
}
