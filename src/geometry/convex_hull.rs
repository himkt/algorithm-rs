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


pub fn convex_hull(ps: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let n = ps.len();

    let mut ps: Vec<Point> = ps.iter().map(|&(x, y)| Point {x, y} ).collect();
    ps.sort();

    let mut k = 0;
    let mut deque: VecDeque<Point> = VecDeque::new();

    for &pi in &ps {
        while k > 1 && (deque[k-1] - deque[k-2]).det(pi - deque[k-1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let t = k;
    for i in (0..n-2).rev() {
        let pi = ps[i];
        while k > t && (deque[k-1] - deque[k-2]).det(pi - deque[k-1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let mut ret: Vec<(i64, i64)> = deque
        .into_iter()
        .take(k-1)
        .map(|pair| (pair.x, pair.y))
        .collect();

    ret.sort_unstable();
    ret
}


#[cfg(test)]
mod test_convex_hull {
    #[test]
    fn it_works() {
        use crate::geometry::convex_hull;

        {
            let ps = vec![(0, 0), (2, 2), (3, 1), (1, 4), (4, 4)];
            let hull = convex_hull::convex_hull(ps);
            assert_eq!(hull, vec![(0, 0), (1, 4), (3, 1), (4, 4)]);
        }
    }
}
