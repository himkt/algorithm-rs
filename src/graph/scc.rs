use crate::graph::graph::Graph;

use std::collections::VecDeque;

pub struct StronglyConnectedComponent {
    forward_graph: Graph,
    forward_visited: Vec<bool>,
    forward_visited_nodes: VecDeque<usize>,
    backward_graph: Graph,
    backward_visited: Vec<bool>,
    topological_ranks: Vec<usize>,
}

impl StronglyConnectedComponent {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let forward_graph = graph;
        let mut backward_graph = Graph::new(n, true);

        for u in 0..n {
            for &(v, _) in forward_graph.graph[u].iter() {
                backward_graph.connect_unweighted(v, u);
            }
        }

        Self {
            forward_graph,
            forward_visited: vec![false; n],
            forward_visited_nodes: VecDeque::new(),
            backward_graph,
            backward_visited: vec![false; n],
            topological_ranks: vec![0; n],
        }
    }

    pub fn scc(&mut self) -> usize {
        for u in 0..self.forward_graph.n {
            if self.forward_visited[u] {
                continue;
            }

            self.fdfs(u);
        }

        let mut topological_rank = 0;
        while let Some(u) = self.forward_visited_nodes.pop_back() {
            if self.backward_visited[u] {
                continue;
            }

            self.rdfs(u, topological_rank);
            topological_rank += 1;
        }

        topological_rank
    }

    fn fdfs(&mut self, u: usize) {
        self.forward_visited[u] = true;

        for i in 0..self.forward_graph.graph[u].len() {
            let (v, _) = self.forward_graph.graph[u][i];

            if self.forward_visited[v] {
                continue;
            }

            self.fdfs(v);
        }

        self.forward_visited_nodes.push_back(u);
    }

    fn rdfs(&mut self, u: usize, topological_rank: usize) {
        self.backward_visited[u] = true;
        self.topological_ranks[u] = topological_rank;

        for i in 0..self.backward_graph.graph[u].len() {
            let (v, _) = self.backward_graph.graph[u][i];

            if self.backward_visited[v] {
                continue;
            }

            self.rdfs(v, topological_rank);
        }
    }
}

#[cfg(test)]
mod test_scc {
    use crate::graph::graph::Graph;
    use crate::graph::scc::StronglyConnectedComponent;

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

        let mut scc = StronglyConnectedComponent::new(graph);
        assert_eq!(scc.scc(), 4);
        assert_eq!(scc.topological_ranks, vec![3, 1, 2, 3, 1, 0]);
    }
}
