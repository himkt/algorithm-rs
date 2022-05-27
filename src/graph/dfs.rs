const INF: usize = 100_000_000_000_000_000;


#[derive(Debug, Clone)]
pub struct DFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    dist: Vec<usize>,
}


impl DFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root, 0);
    }

    pub fn dfs(&mut self, v: usize, dist: usize) {
        if self.seen[v] {
            return;
        }

        self.seen[v] = true;
        self.dist[v] = self.dist[v].min(dist);

        for i in 0..self.graph[v].len() {
            self.dfs(self.graph[v][i], dist + 1);
        }
    }
}


#[cfg(test)]
mod test_dfs {

    #[test]
    fn it_works() {
        use crate::graph::graph::GraphBuilder;
        use crate::graph::dfs::DFS;
        use crate::graph::dfs::INF;
        {
            let mut graph = GraphBuilder::new(5, true);
            graph.connect(0, 1);
            graph.connect(1, 2);
            graph.connect(2, 4);

            let mut dfs = DFS::new(graph.graph);
            dfs.search(0);
            assert_eq!(dfs.seen, vec![true, true, true, false, true]);
            assert_eq!(dfs.dist, vec![0, 1, 2, INF, 3]);
        }
    }
}
