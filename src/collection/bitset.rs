pub struct Bitset<T: Copy> {
    curr: usize,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr == (1 << self.len) {
            return None;
        }

        let mut ret = Vec::<T>::new();
        for (i, &ai) in self.array.iter().enumerate() {
            if (self.curr >> i & 1) == 1 {
                ret.push(ai);
            }
        }

        self.curr += 1;
        Some(ret)
    }
}

pub fn bitset<T: Copy>(a: Vec<T>) -> Bitset<T> {
    let len = a.len();
    Bitset {
        curr: 0,
        array: a,
        len,
    }
}

#[cfg(test)]
mod test_permutation {
    use crate::collection::bitset;

    #[test]
    fn it_works() {
        let mut bitset = bitset::bitset(vec![1, 2, 3]);
        assert_eq!(bitset.next(), Some(vec![]));
        assert_eq!(bitset.next(), Some(vec![1]));
        assert_eq!(bitset.next(), Some(vec![2]));
        assert_eq!(bitset.next(), Some(vec![1, 2]));
        assert_eq!(bitset.next(), Some(vec![3]));
        assert_eq!(bitset.next(), Some(vec![1, 3]));
        assert_eq!(bitset.next(), Some(vec![2, 3]));
        assert_eq!(bitset.next(), Some(vec![1, 2, 3]));
        assert!(bitset.next().is_none());
    }
}
