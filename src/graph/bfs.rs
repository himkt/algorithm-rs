use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
pub struct BreadthFirstSearch {
    graph: Graph,
    seen: Vec<bool>,
    dist: Vec<usize>,
}

impl BreadthFirstSearch {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![BreadthFirstSearch::INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root, 0));

        while let Some((cur, dist)) = queue.pop_front() {
            if self.seen[cur] {
                continue;
            }

            self.seen[cur] = true;
            self.dist[cur] = self.dist[cur].min(dist);
            for &(next, _) in self.graph.graph[cur].iter() {
                queue.push_back((next, self.dist[cur] + 1));
            }
        }
    }
}

#[cfg(test)]
mod test_bfs {
    use crate::graph::bfs::BreadthFirstSearch;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(5, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);

        let mut bfs = BreadthFirstSearch::new(graph);
        bfs.search(0);
        assert_eq!(bfs.seen, vec![true, true, true, false, true]);
        assert_eq!(bfs.dist, vec![0, 1, 2, BreadthFirstSearch::INF, 3]);
    }
}
