use std::{collections::VecDeque, ops::Sub, cmp::Ordering};

#[derive(Debug, Copy, Clone, Eq)]
pub struct Point {
    x: i64,
    y: i64,
}


impl Sub for Point {
    type Output = Self;

    fn sub(self, _rhs: Point) -> Self {
        Self { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}


impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x != other.x {
            return self.x.cmp(&other.x);
        }

        self.y.cmp(&other.y)
    }
}


impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


impl Point {
    pub fn det(self, _rhs: Point) -> i64 {
        self.x*_rhs.y - self.y*_rhs.x
    }
}


pub struct ConvexHull {
    ps: Vec<Point>,
    n: usize,
}


impl ConvexHull {
    pub fn new(ps: Vec<Point>) -> Self {
        let n: usize = ps.len();
        ConvexHull { ps, n }
    }

    pub fn construct(&mut self) -> Vec<Point> {
        self.ps.sort_unstable();

        let mut k = 0;
        let mut deque: VecDeque<Point> = VecDeque::new();

        for i in 0..self.n {
            while k > 1 && (deque[k-1] - deque[k-2]).det(self.ps[i] - deque[k-1]) <= 0 {
                k -= 1;
            }
            deque.push_back(self.ps[i]);
        }

        let t = k;
        for i in (0..self.n-2).rev() {
            while k > t && (deque[k-1] - deque[k-2]).det(self.ps[i] - deque[k-1]) <= 0 {
                k -= 1;
            }
            deque.push_back(self.ps[i]);
        }

        deque.into_iter().collect()
    }
}


#[cfg(test)]
mod test_convex_hull {
    #[test]
    fn it_works() {
        use crate::geometry::convex_hull;

        {
            let p1 = convex_hull::Point { x: 0, y: 0 };
            let p2 = convex_hull::Point { x: 2, y: 2 };
            let p3 = convex_hull::Point { x: 3, y: 1 };
            let p4 = convex_hull::Point { x: 1, y: 4 };
            let p5 = convex_hull::Point { x: 4, y: 4 };
            let ps = vec![p1, p2, p3, p4, p5];
            let mut builder = convex_hull::ConvexHull::new(ps);
            let convex_hull = builder.construct();
            println!("{:?}", convex_hull);
        }
    }
}
