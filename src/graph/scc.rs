use crate::graph::graph::Graph;


pub struct SCC {
    fgraph: Graph,
    rgraph: Graph,
    fused: Vec<bool>,
    rused: Vec<bool>,
    cmp: Vec<usize>,
    vs: Vec<usize>,
    n: usize,
}


impl SCC {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let fused = vec![false; n];
        let rused = vec![false; n];
        let cmp = vec![0; n];
        let vs = vec![];

        let fgraph = graph;
        let mut rgraph = Graph::new(n, true);

        for u in 0..n {
            for &(v, _) in fgraph.graph[u].iter() {
                rgraph.connect_unweighted(v, u);
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

        for i in 0..self.fgraph.graph[u].len() {
            let (v, _) = self.fgraph.graph[u][i];

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

        for i in 0..self.rgraph.graph[u].len() {
            let (v, _) = self.rgraph.graph[u][i];

            if self.rused[v] {
                continue;
            }

            self.rdfs(v, k);
        }
    }
}


#[cfg(test)]
mod test_scc {
    use crate::graph::graph::Graph;
    use crate::graph::scc::SCC;

    #[test]
    fn it_works() {

        let mut graph = Graph::new(6, true);
        graph.connect_unweighted(1, 4);
        graph.connect_unweighted(5, 2);
        graph.connect_unweighted(3, 0);
        graph.connect_unweighted(5, 5);
        graph.connect_unweighted(4, 1);
        graph.connect_unweighted(0, 3);
        graph.connect_unweighted(4, 2);

        let mut scc = SCC::new(graph);
        assert_eq!(scc.scc(), 4);
        assert_eq!(scc.cmp, vec![3, 1, 2, 3, 1, 0]);
    }
}
