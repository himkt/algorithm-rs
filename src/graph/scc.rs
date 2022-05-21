pub struct SCC {
    fgraph: Vec<Vec<usize>>,
    rgraph: Vec<Vec<usize>>,
    fused: Vec<bool>,
    rused: Vec<bool>,
    cmp: Vec<usize>,
    vs: Vec<usize>,
    n: usize,
}


impl SCC {
    #[allow(clippy::needless_range_loop)]
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        let fused = vec![false; n];
        let rused = vec![false; n];
        let cmp = vec![0; n];
        let vs = vec![];

        let fgraph = graph;

        let mut rgraph = vec![vec![]; n];
        for u in 0..n {
            for &v in fgraph[u].iter() {
                rgraph[v].push(u);
            }
        }

        Self {
            fgraph,
            rgraph,
            fused,
            rused,
            cmp,
            vs,
            n
        }
    }

    pub fn scc(&mut self) -> usize {
        for u in 0..self.n {
            if self.fused[u] {
                continue;
            }

            self.fdfs(u);
        }

        let mut k = 0;

        for i in (0..self.vs.len()).rev() {
            let u = self.vs[i];

            if self.rused[u] {
                continue;
            }

            self.rdfs(u, k);
            k += 1;
        }

        k
    }

    fn fdfs(&mut self, u: usize) {
        self.fused[u] = true;

        for i in 0..self.fgraph[u].len() {
            let v = self.fgraph[u][i];

            if self.fused[v] {
                continue;
            }

            self.fdfs(v);
        }

        self.vs.push(u);
    }

    fn rdfs(&mut self, u: usize, k: usize) {
        self.rused[u] = true;
        self.cmp[u] = k;

        for i in 0..self.rgraph[u].len() {
            let v = self.rgraph[u][i];

            if self.rused[v] {
                continue;
            }

            self.rdfs(v, k);
        }
    }
}


#[cfg(test)]
mod test_scc {
    use crate::graph::scc::SCC;

    fn build_graph(n: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n];

        for (u, v) in edges {
            graph[u].push(v);
        }

        graph
    }

    #[test]
    fn it_works() {
        let edges = vec![
            (1, 4),
            (5, 2),
            (3, 0),
            (5, 5),
            (4, 1),
            (0, 3),
            (4, 2),
        ];
        let graph = build_graph(6, edges);

        let mut scc = SCC::new(graph);
        assert_eq!(scc.scc(), 4);
        assert_eq!(scc.cmp, vec![3, 1, 2, 3, 1, 0]);
    }
}
