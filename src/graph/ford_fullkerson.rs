use crate::graph::graph::Graph;

pub struct FordFullkerson {
    pub graph: Graph,
    pub used: Vec<bool>,
    pub flow: usize,
}

impl FordFullkerson {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let used = vec![false; graph.n];
        let flow = Self::INF;
        Self { graph, used, flow }
    }

    pub fn dfs(&mut self, u: usize, t: usize, f: usize) -> usize {
        if u == t {
            return f;
        }

        self.used[u] = true;
        for i in 0..self.graph.graph[u].len() {
            let (v, cap) = self.graph.graph[u][i];
            let r = self.graph.rev[u][i];
            if !self.used[v] && cap > 0 {
                let d = self.dfs(v, t, f.min(cap));

                if d > 0 {
                    self.graph.graph[u][i].1 -= d;
                    self.graph.graph[v][r].1 += d;
                    return d;
                }
            }
        }

        0
    }

    pub fn solve(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.graph.n];
            let f: usize = self.dfs(s, t, FordFullkerson::INF);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod test_lowlink {
    use crate::graph::ford_fullkerson::FordFullkerson;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(4, true);
        graph.connect_with_residual(0, 1, 2);
        graph.connect_with_residual(0, 2, 1);
        graph.connect_with_residual(1, 2, 1);
        graph.connect_with_residual(1, 3, 1);
        graph.connect_with_residual(2, 3, 2);

        let mut solver = FordFullkerson::new(graph);
        let maxflow = solver.solve(0, 3);
        assert_eq!(maxflow, 3);
    }
}
