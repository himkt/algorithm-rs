pub trait BinarySearch<T> {
    fn lower_bound(&self, query: T) -> usize;
}


impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, query: T) -> usize {
        let mut left  = 0;
        let mut right = self.len();

        // update here
        let check = |mid| {
            self[mid] >= query
        };

        while left < right {
            let mid = left + (right - left) / 2;

            if check(mid) {
                right = mid;
            }
            else {
                left = mid + 1;
            }

        }

        left
    }
}


#[cfg(test)]
mod test_lower_bound {
    #[test]
    fn it_works() {
        use crate::search::lower_bound::BinarySearch;
        {
            let vs: Vec<usize> = vec![0, 1, 2, 3, 5, 7, 10];
            assert_eq!(vs.lower_bound(1), 1);
            assert_eq!(vs.lower_bound(3), 3);
            assert_eq!(vs.lower_bound(7), 5);
        }
    }
}
