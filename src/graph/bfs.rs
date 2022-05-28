use crate::graph::graph::Graph;

const INF: usize = 100_000_000_000_000_000;


#[derive(Debug, Clone)]
pub struct BFS {
    graph: Graph,
    seen: Vec<bool>,
    dist: Vec<usize>,
}


impl BFS {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![INF; n],
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
    use crate::graph::graph::Graph;
    use crate::graph::bfs::BFS;
    use crate::graph::bfs::INF;

    #[test]
    fn it_works() {

        let mut graph = Graph::new(5, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);

        let mut bfs = BFS::new(graph);
        bfs.search(0);
        assert_eq!(bfs.seen, vec![true, true, true, false, true]);
        assert_eq!(bfs.dist, vec![0, 1, 2, INF, 3]);
    }
}
