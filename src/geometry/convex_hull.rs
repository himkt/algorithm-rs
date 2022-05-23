use crate::geometry::int_point::IntPoint;
use std::collections::VecDeque;

pub fn convex_hull(ps: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let n = ps.len();

    let mut ps: Vec<IntPoint> = ps
        .iter()
        .map(|&(x, y)| IntPoint(x, y))
        .collect();

    ps.sort();

    let mut k = 0;
    let mut deque: VecDeque<IntPoint> = VecDeque::new();

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

    let mut ret: Vec<(i64, i64)> = deque
        .into_iter()
        .take(k - 1)
        .map(|pair| (pair.0, pair.1))
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

            let ps = vec![(0, 0), (2, 2), (4, 4)];
            let hull = convex_hull::convex_hull(ps);
            assert_eq!(hull, vec![(0, 0), (4, 4)]);

            let ps = vec![(0, 0), (0, 1), (0, 2), (1, 1)];
            let hull = convex_hull::convex_hull(ps);
            assert_eq!(hull, vec![(0, 0), (0, 2), (1, 1)]);
        }
    }
}
