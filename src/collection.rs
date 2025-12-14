use std::{collections::VecDeque, ops::Range};

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
mod test_bitset {
    use crate::collection::bitset;

    #[test]
    fn it_works() {
        let mut bitset = bitset(vec![1, 2, 3]);
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

#[derive(Debug)]
pub enum Item {
    Pre(usize),
    Post(usize),
}

#[derive(PartialEq)]
pub enum CollectionType {
    Permutation,
    Combination,
}

pub struct CollectionIter<'a> {
    pub a: Range<usize>,
    pub n: usize,
    pub k: usize,
    pub data: &'a Vec<usize>,
    pub depth: usize,
    pub stack: VecDeque<Item>,
    pub permutation: Vec<usize>,
    pub used: Vec<bool>,
    pub collection_type: CollectionType,
    pub allow_duplication: bool,
}

impl<'a> CollectionIter<'a> {
    pub fn permutation(data: &'a Vec<usize>, allow_duplication: bool) -> Self {
        let n: usize = data.len();
        let a = 0..n;

        let mut stack = VecDeque::new();
        for i in a.clone().rev() {
            stack.push_front(Item::Post(i));
            stack.push_front(Item::Pre(i));
        }

        CollectionIter {
            a,
            n,
            k: n,
            data,
            depth: 0,
            stack,
            permutation: vec![0; n],
            used: vec![false; n],
            collection_type: CollectionType::Permutation,
            allow_duplication,
        }
    }

    pub fn combination(data: &'a Vec<usize>, k: usize, allow_duplication: bool) -> Self {
        let n: usize = data.len();
        let a = 0..n;

        let mut stack = VecDeque::new();
        for i in a.clone().rev() {
            stack.push_front(Item::Post(i));
            stack.push_front(Item::Pre(i));
        }

        CollectionIter {
            a,
            n,
            k,
            data,
            depth: 0,
            stack,
            permutation: vec![0; k],
            used: vec![false; n],
            collection_type: CollectionType::Combination,
            allow_duplication,
        }
    }

    pub fn should_skip(&self, ni: usize) -> bool {
        let is_permutation = self.collection_type == CollectionType::Permutation;
        is_permutation && !self.allow_duplication && self.used[ni]
    }

    pub fn lower(&self, ni: usize) -> usize {
        match (&self.collection_type, self.allow_duplication) {
            (&CollectionType::Permutation, true) => 0,
            (&CollectionType::Permutation, false) => 0,
            (&CollectionType::Combination, true) => ni,
            (&CollectionType::Combination, false) => ni + 1,
        }
    }
}

impl Iterator for CollectionIter<'_> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item_wrapper) = self.stack.pop_front() {
            match item_wrapper {
                Item::Pre(i) => {
                    self.permutation[self.depth] = self.data[i];
                    self.used[i] = true;
                    self.depth += 1;

                    if self.depth == self.k {
                        return Some(self.permutation.clone());
                    }

                    for ni in (self.lower(i)..self.a.end).rev() {
                        if self.should_skip(ni) {
                            continue;
                        }
                        self.stack.push_front(Item::Post(ni));
                        self.stack.push_front(Item::Pre(ni));
                    }
                }
                Item::Post(i) => {
                    self.depth -= 1;
                    self.used[i] = false;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test_iterator {
    use crate::collection::CollectionIter;
    fn check(iterator: CollectionIter, num_expected: usize, expected: Vec<Vec<usize>>) {
        let mut num_count = 0;
        for perm in iterator {
            assert_eq!(expected[num_count], perm);
            num_count += 1;
        }
        assert_eq!(num_count, num_expected);
    }

    mod with_duplication {
        use crate::collection::{test_iterator::check, CollectionIter};

        #[test]
        fn it_works_permutation() {
            let data = vec![1, 2, 4];
            let num_expected = 27; // n^3
            let iterator = CollectionIter::permutation(&data, true);
            let expected = vec![
                vec![1, 1, 1],
                vec![1, 1, 2],
                vec![1, 1, 4],
                vec![1, 2, 1],
                vec![1, 2, 2],
                vec![1, 2, 4],
                vec![1, 4, 1],
                vec![1, 4, 2],
                vec![1, 4, 4],
                vec![2, 1, 1],
                vec![2, 1, 2],
                vec![2, 1, 4],
                vec![2, 2, 1],
                vec![2, 2, 2],
                vec![2, 2, 4],
                vec![2, 4, 1],
                vec![2, 4, 2],
                vec![2, 4, 4],
                vec![4, 1, 1],
                vec![4, 1, 2],
                vec![4, 1, 4],
                vec![4, 2, 1],
                vec![4, 2, 2],
                vec![4, 2, 4],
                vec![4, 4, 1],
                vec![4, 4, 2],
                vec![4, 4, 4],
            ];
            check(iterator, num_expected, expected);
        }

        #[test]
        fn it_works_combination() {
            let k: usize = 3;
            let data = vec![1, 2, 4];
            let num_expected = 10; // c(n + k - 1, k)
            let iterator = CollectionIter::combination(&data, k, true);

            let expected = vec![
                vec![1, 1, 1],
                vec![1, 1, 2],
                vec![1, 1, 4],
                vec![1, 2, 2],
                vec![1, 2, 4],
                vec![1, 4, 4],
                vec![2, 2, 2],
                vec![2, 2, 4],
                vec![2, 4, 4],
                vec![4, 4, 4],
            ];
            check(iterator, num_expected, expected);
        }
    }

    mod without_duplication {
        use crate::collection::{test_iterator::check, CollectionIter};

        #[test]
        fn it_works_permutation() {
            let data = vec![1, 2, 4, 8];
            let num_expected = 24; // 4!
            let iterator = CollectionIter::permutation(&data, false);
            let expected = vec![
                vec![1, 2, 4, 8],
                vec![1, 2, 8, 4],
                vec![1, 4, 2, 8],
                vec![1, 4, 8, 2],
                vec![1, 8, 2, 4],
                vec![1, 8, 4, 2],
                vec![2, 1, 4, 8],
                vec![2, 1, 8, 4],
                vec![2, 4, 1, 8],
                vec![2, 4, 8, 1],
                vec![2, 8, 1, 4],
                vec![2, 8, 4, 1],
                vec![4, 1, 2, 8],
                vec![4, 1, 8, 2],
                vec![4, 2, 1, 8],
                vec![4, 2, 8, 1],
                vec![4, 8, 1, 2],
                vec![4, 8, 2, 1],
                vec![8, 1, 2, 4],
                vec![8, 1, 4, 2],
                vec![8, 2, 1, 4],
                vec![8, 2, 4, 1],
                vec![8, 4, 1, 2],
                vec![8, 4, 2, 1],
            ];
            check(iterator, num_expected, expected);
        }

        #[test]
        fn it_works_combination() {
            let k: usize = 3;
            let num_expected = 4; // c(n, k)
            let data = vec![1, 2, 4, 8];
            let iterator = CollectionIter::combination(&data, k, false);
            let expected = vec![vec![1, 2, 4], vec![1, 2, 8], vec![1, 4, 8], vec![2, 4, 8]];
            check(iterator, num_expected, expected);
        }
    }
}

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

#[cfg(test)]
mod test_ndarray {

    #[test]
    fn it_works() {
        // ndarray!(val; 1) => [val]
        assert_eq!(ndarray!(5; 1), vec![5]);
        // ndarray!(val; 1, 2) => [[val, val]]
        assert_eq!(ndarray!(5; 1, 2), vec![vec![5, 5]]);
        // ndarray!(val; 2, 1) => [[val], [val]]
        assert_eq!(ndarray!(5; 2, 1), vec![vec![5], vec![5]]);
    }
}

#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
        }
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let px = self.parent(x);
        let py = self.parent(y);
        px == py
    }
}

#[cfg(test)]
mod test_union_find {
    use crate::collection::UnionFind;

    // helper function
    fn sizes(uf: &mut UnionFind, n: usize) -> Vec<usize> {
        (0..n).map(|i| uf.size(i)).collect()
    }

    #[test]
    fn it_works() {
        let n: usize = 5;
        let mut uf = UnionFind::new(n);
        assert_eq!(sizes(&mut uf, n), [1, 1, 1, 1, 1]);

        uf.unite(0, 1);
        assert_eq!(uf.parent(0), uf.parent(1));
        assert!(uf.same(0, 1));
        assert_ne!(uf.parent(0), uf.parent(2));
        assert!(!uf.same(0, 2));
        assert_eq!(sizes(&mut uf, n), [2, 2, 1, 1, 1]);

        // check noop
        uf.unite(0, 1);
        assert_eq!(uf.parent(0), uf.parent(1));
        assert!(uf.same(0, 1));
        assert_ne!(uf.parent(0), uf.parent(2));
        assert!(!uf.same(0, 2));
        assert_eq!(sizes(&mut uf, n), [2, 2, 1, 1, 1]);

        uf.unite(0, 2);
        assert_eq!(uf.parent(0), uf.parent(2));
        assert!(uf.same(0, 2));
        assert_eq!(sizes(&mut uf, n), [3, 3, 3, 1, 1]);

        uf.unite(3, 4);
        assert_ne!(uf.parent(0), uf.parent(3));
        assert!(!uf.same(0, 3));
        assert_eq!(sizes(&mut uf, n), [3, 3, 3, 2, 2]);

        uf.unite(0, 3);
        assert_eq!(uf.parent(0), uf.parent(3));
        assert!(uf.same(0, 3));
        assert_eq!(uf.parent(0), uf.parent(4));
        assert!(uf.same(0, 4));
        assert_eq!(sizes(&mut uf, n), [5, 5, 5, 5, 5]);
    }
}
