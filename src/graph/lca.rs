const ROOT: usize = 0;
const MAX_LOG_V: usize = 30;


pub struct LCA {
    parents: Vec<Vec<usize>>,
    depth: Vec<usize>,
    graph: Vec<Vec<usize>>,
}


impl LCA {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let parents = vec![vec![ROOT; n]; MAX_LOG_V];
        let depth = vec![ROOT; n];
        LCA { parents, depth, graph }
    }

    pub fn init(&mut self) {
        self.dfs(ROOT, ROOT, 0);

        for k in 0..MAX_LOG_V-1 {
            for v in 0..self.graph.len() {
                self.parents[k+1][v] = self.parents[k][self.parents[k][v]];
            }
        }
    }

    fn dfs(&mut self, v: usize, p: usize, d: usize) {
        self.parents[0][v] = p;
        self.depth[v] = d;

        for i in 0..self.graph[v].len() {
            let nv = self.graph[v][i];

            if nv != p {
                self.dfs(nv, v, d + 1);
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        for k in 0..MAX_LOG_V {
            if (((self.depth[v] - self.depth[u]) >> k) & 1) == 1 {
                v = self.parents[k][v];
            }
        }

        if u == v {
            return u;
        }

        for k in (0..MAX_LOG_V).rev() {
            if self.parents[k][u] != self.parents[k][v] {
                u = self.parents[k][u];
                v = self.parents[k][v];
            }
        }

        self.parents[0][u]
    }
}


#[cfg(test)]
mod test_lca {
    use crate::graph::lca::LCA;

    #[test]
    fn it_works() {
        let graph = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
            vec![7],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let mut lca = LCA::new(graph);
        lca.init();

        assert_eq!(lca.lca(0, 1), 0);
        assert_eq!(lca.lca(7, 4), 1);
        assert_eq!(lca.lca(1, 4), 1);
        assert_eq!(lca.lca(4, 1), 1);
        assert_eq!(lca.lca(7, 6), 0);
        assert_eq!(lca.lca(5, 6), 2);
        assert_eq!(lca.lca(3, 5), 0);
    }

    #[test]
    fn it_works_line() {
        let graph = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![],
        ];

        let mut lca = LCA::new(graph);
        lca.init();

        assert_eq!(lca.lca(0, 0), 0);
        assert_eq!(lca.lca(0, 1), 0);
        assert_eq!(lca.lca(0, 2), 0);
        assert_eq!(lca.lca(0, 3), 0);
        assert_eq!(lca.lca(1, 4), 1);
        assert_eq!(lca.lca(4, 1), 1);
    }
}
