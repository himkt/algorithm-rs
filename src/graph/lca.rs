use std::collections::VecDeque;

use crate::graph::graph::Graph;

pub struct LowestCommonAncestor {
    parents: Vec<Vec<usize>>,
    depth: Vec<usize>,
    graph: Graph,
}

impl LowestCommonAncestor {
    const ROOT: usize = 0;
    const LOGV: usize = 30;

    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let parents = vec![vec![Self::ROOT; n]; Self::LOGV];
        let depth = vec![Self::ROOT; n];
        Self {
            parents,
            depth,
            graph,
        }
    }

    pub fn init(&mut self) {
        self.dfs(Self::ROOT);

        for k in 0..Self::LOGV - 1 {
            for v in 0..self.graph.n {
                self.parents[k + 1][v] = self.parents[k][self.parents[k][v]];
            }
        }
    }

    fn dfs(&mut self, u: usize) {
        let mut stack = VecDeque::new();
        self.parents[0][u] = u;
        self.depth[u] = 0;
        stack.push_front((u, u, 1));

        while let Some((u, p, d)) = stack.pop_front() {
            for &(v, _) in self.graph.graph[u].iter() {
                if v == p {
                    continue;
                }
                self.parents[0][v] = u;
                self.depth[v] = d + 1;
                stack.push_front((v, u, d + 1));
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        for k in 0..Self::LOGV {
            if (((self.depth[v] - self.depth[u]) >> k) & 1) == 1 {
                v = self.parents[k][v];
            }
        }

        if u == v {
            return u;
        }

        for k in (0..Self::LOGV).rev() {
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
    use crate::graph::graph::Graph;
    use crate::graph::lca::LowestCommonAncestor;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(8, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(0, 2);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(1, 4);
        graph.connect_unweighted(2, 5);
        graph.connect_unweighted(2, 6);
        graph.connect_unweighted(3, 7);

        let mut lca = LowestCommonAncestor::new(graph);
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
        let mut graph = Graph::new(5, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 3);
        graph.connect_unweighted(3, 4);

        let mut lca = LowestCommonAncestor::new(graph);
        lca.init();

        assert_eq!(lca.lca(0, 0), 0);
        assert_eq!(lca.lca(0, 1), 0);
        assert_eq!(lca.lca(0, 2), 0);
        assert_eq!(lca.lca(0, 3), 0);
        assert_eq!(lca.lca(1, 4), 1);
        assert_eq!(lca.lca(4, 1), 1);
    }
}
