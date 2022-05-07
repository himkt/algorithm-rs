#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n]
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        }
        else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);

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
        let x = self.find(x);
        self.sizes[x]
    }
}


#[cfg(test)]
mod test_union_find {
    #[test]
    fn it_works() {
        use crate::collection::union_find::UnionFind;
        const N: usize = 5;

        {
            // helper
            fn sizes(uf: &mut UnionFind) -> Vec<usize> {
                (0..N)
                    .map(|i| uf.size(i))
                    .collect()
            }

            let mut uf = UnionFind::new(N);
            assert_eq!(sizes(&mut uf), [1, 1, 1, 1, 1]);

            uf.unite(0, 1);
            assert_eq!(uf.find(0), uf.find(1));
            assert_ne!(uf.find(0), uf.find(2));
            assert_eq!(sizes(&mut uf), [2, 2, 1, 1, 1]);

            // check noop
            uf.unite(0, 1);
            assert_eq!(uf.find(0), uf.find(1));
            assert_ne!(uf.find(0), uf.find(2));
            assert_eq!(sizes(&mut uf), [2, 2, 1, 1, 1]);

            uf.unite(0, 2);
            assert_eq!(uf.find(0), uf.find(2));
            assert_eq!(sizes(&mut uf), [3, 3, 3, 1, 1]);

            uf.unite(3, 4);
            assert_ne!(uf.find(0), uf.find(3));
            assert_eq!(sizes(&mut uf), [3, 3, 3, 2, 2]);

            uf.unite(0, 3);
            assert_eq!(uf.find(0), uf.find(3));
            assert_eq!(uf.find(0), uf.find(4));
            assert_eq!(sizes(&mut uf), [5, 5, 5, 5, 5]);

        }
    }
}
