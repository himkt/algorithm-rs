#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..=n).collect()
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
        let px = self.find(x);
        let py = self.find(y);

        if px != py {
            self.parents[x] = py;
        }
    }
}


#[cfg(test)]
mod test_union_find {
    #[test]
    fn it_works() {
        use crate::tree::union_find::UnionFind;
        {
            let mut uf = UnionFind::new(5);
            uf.unite(0, 1);
            assert_eq!(uf.find(0), uf.find(1));
            assert_ne!(uf.find(0), uf.find(2));
            uf.unite(0, 2);
            assert_eq!(uf.find(0), uf.find(2));
            uf.unite(3, 4);
            assert_ne!(uf.find(0), uf.find(3));
            uf.unite(0, 3);
            assert_eq!(uf.find(0), uf.find(3));
            assert_eq!(uf.find(0), uf.find(4));

        }
    }
}
