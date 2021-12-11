


#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parents = Vec::<usize>::new();
        parents.resize(n, 0);

        for i in 0..n {
            parents[i] = i;
        }

        Self {
            parents
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let p = self.parents[x];
        if p == x {
            return x;
        }

        self.parents[x] = p;
        self.find(p)
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let px = self.parents[x];
        let py = self.parents[y];

        if px != py {
            self.parents[py] = px;
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

        }
    }
}
