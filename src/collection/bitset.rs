pub struct Bitset<T: Copy> {
    curr: usize,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr >= 1 << self.len {
            return None;
        }

        let mut ret = Vec::<T>::new();
        let r_array = self.array.clone();
        for (i, ai) in r_array.iter().enumerate() {
            let patch = self.curr >> i & 1;
            if patch == 1 {
                ret.push(*ai);
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
    #[test]
    fn it_works() {
        use crate::collection::bitset;
        {
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
}
