use crate::graph::graph::Graph;

pub struct EulerTour {
    graph: Graph,
    l: Vec<usize>,
    r: Vec<usize>,
    t: usize,
}

impl EulerTour {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(n: usize, graph: Graph) -> Self {
        let l = vec![EulerTour::INF; n];
        let r = vec![EulerTour::INF; n];
        EulerTour { graph, l, r, t: 1 }
    }

    /// Euler tour entrypoint that returns two vectors `(&l, &r)`.
    /// Note that timestamp starts from `1`.
    ///
    /// - `l`: vector indicates the timestamp that visits a node `u` at the first time.
    /// - `r`: vector indicates the timestamp that visits a node `u` at the last time.
    pub fn traverse(&mut self, root: usize) -> (&[usize], &[usize]) {
        self._dfs(root, None);

        for i in 0..self.l.len() {
            if self.r[i] == EulerTour::INF {
                self.r[i] = self.l[i];
            }
        }

        (&self.l, &self.r)
    }

    fn _dfs(&mut self, u: usize, p: Option<usize>) {
        self.l[u] = self.t;
        self.t += 1;

        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];
            if p != Some(v) {
                self._dfs(v, Some(u));
                self.r[u] = self.t;
                self.t += 1;
            }
        }
    }
}

#[cfg(test)]
mod test_euler_tour {
    use crate::graph::euler_tour::EulerTour;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(7, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(3, 4);
        graph.connect_unweighted(0, 5);
        graph.connect_unweighted(5, 6);

        let mut euler_tour = EulerTour::new(7, graph);
        let (l, r) = euler_tour.traverse(0);

        assert_eq!(l, &vec![1, 2, 3, 5, 6, 10, 11]);
        assert_eq!(r, &vec![13, 8, 3, 7, 6, 12, 11]);
    }
}
