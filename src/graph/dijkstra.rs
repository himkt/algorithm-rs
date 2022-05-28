use crate::graph::graph::Graph;

const INF: usize = 100_000_000_000_000_000;

#[derive(Debug, Clone)]
pub struct Dijkstra {
    source: usize,
    graph: Graph,
    pub dist: Vec<usize>,
    backptrs: Vec<usize>,
}

impl Dijkstra {
    pub fn new(graph: Graph) -> Self {
        let dist: Vec<usize> = vec![INF; graph.n];
        let backptrs: Vec<usize> = (0..graph.n).collect();
        Self {
            source: INF,
            graph,
            dist,
            backptrs,
        }
    }

    pub fn search(&mut self, src: usize) {
        self.source = src;

        let mut dist = vec![INF; self.graph.n];
        dist[src] = 0;

        let mut queue = std::collections::BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while let Some((std::cmp::Reverse(current_cost), current_v)) = queue.pop() {
            if dist[current_v] < current_cost {
                continue;
            }

            for &(v, cost) in self.graph.graph[current_v].iter() {
                if dist[v] > current_cost + cost {
                    dist[v] = current_cost + cost;
                    queue.push((std::cmp::Reverse(dist[v]), v));
                    self.backptrs[v] = current_v;
                }
            }
        }

        self.dist = dist;
    }

    pub fn shortest_path(&self, u: usize, v: usize) -> Vec<(usize, usize)> {
        assert_eq!(u, self.source);

        if self.dist[v] == INF {
            return vec![];
        }

        let mut path = std::collections::VecDeque::new();

        let mut cursor = v;
        while cursor != u {
            path.push_front((self.backptrs[cursor], cursor));
            cursor = self.backptrs[cursor];
        }

        Vec::from(path)
    }
}

#[cfg(test)]
mod test_dijkstra {
    use crate::graph::dijkstra::Dijkstra;
    use crate::graph::dijkstra::INF;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(7, true);
        graph.connect(0, 1, 2);
        graph.connect(0, 2, 5);
        graph.connect(1, 2, 4);
        graph.connect(1, 3, 5);
        graph.connect(1, 4, 10);
        graph.connect(2, 3, 2);
        graph.connect(3, 5, 1);
        graph.connect(4, 5, 5);
        graph.connect(4, 6, 5);
        graph.connect(5, 6, 10);

        let mut dijkstra = Dijkstra::new(graph);
        dijkstra.search(0);
        assert_eq!(dijkstra.dist, vec![0, 2, 5, 7, 12, 8, 17]);

        let expected = vec![(0, 1), (1, 4), (4, 6)];
        assert_eq!(dijkstra.shortest_path(0, 6), expected);
    }

    #[test]
    fn it_works_unreachable_path() {
        let mut graph = Graph::new(9, true);
        graph.connect(0, 1, 1);
        graph.connect(0, 6, 10);
        graph.connect(1, 2, 5);
        graph.connect(1, 3, 2);
        graph.connect(3, 4, 2);
        graph.connect(4, 5, 3);
        graph.connect(5, 6, 3);
        graph.connect(7, 0, 1);
        graph.connect(8, 7, 2);

        let mut dijkstra = Dijkstra::new(graph);
        dijkstra.search(0);
        assert_eq!(dijkstra.dist, vec![0, 1, 6, 3, 5, 8, 10, INF, INF]);

        assert_eq!(dijkstra.shortest_path(0, 3), vec![(0, 1), (1, 3)]);
        assert_eq!(dijkstra.shortest_path(0, 8), vec![]);
    }
}
