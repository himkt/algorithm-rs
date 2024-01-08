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
    use crate::collection::union_find::UnionFind;

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
