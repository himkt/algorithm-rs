use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
pub struct DepthFirstSearch {
    graph: Graph,
    seen: Vec<bool>,
    dist: Vec<usize>,
}

impl DepthFirstSearch {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![Self::INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.seen[root] = true;
        self.dist[root] = 0;
        self.dfs(root, 0);
    }

    pub fn dfs(&mut self, u: usize, dist: usize) {
        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];
            if self.seen[v] {
                continue;
            }
            self.seen[v] = true;
            self.dist[v] = self.dist[v].min(dist + 1);
            self.dfs(v, dist + 1);
        }
    }
}

#[cfg(test)]
mod test_dfs {
    use crate::graph::dfs::DepthFirstSearch;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(5, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);

        let mut dfs = DepthFirstSearch::new(graph);
        dfs.search(0);
        assert_eq!(dfs.seen, vec![true, true, true, false, true]);
        assert_eq!(dfs.dist, vec![0, 1, 2, DepthFirstSearch::INF, 3]);
    }
}
