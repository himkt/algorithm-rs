use crate::graph::graph::Graph;
use std::collections::{BinaryHeap, VecDeque};

pub enum VisitType {
    Forward(usize),
    Backtrack(usize),
}

pub struct StoronglyConnectedComponent {
    n: usize,
    forward_graph: Graph,
    backward_graph: Graph,
    topological_ranks: Vec::<usize>,
}

impl StoronglyConnectedComponent {
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
            n,
            forward_graph,
            backward_graph,
            topological_ranks: vec![0; n],
        }
    }

    pub fn scc(&mut self) -> usize {
        let mut heap = BinaryHeap::new();
        let mut priority: usize = 0;

        let mut forward_visited = vec![false; self.n];
        let mut stack = VecDeque::new();
        for u in 0..self.forward_graph.n {
            if forward_visited[u] {
                continue;
            }

            stack.push_front(VisitType::Backtrack(u));
            stack.push_front(VisitType::Forward(u));

            while let Some(node) = stack.pop_front() {
                match node {
                    VisitType::Forward(u) => {
                        for &(v, _) in self.forward_graph.graph[u].iter() {
                            if forward_visited[v] {
                                continue;
                            }
                            stack.push_front(VisitType::Backtrack(v));
                            stack.push_front(VisitType::Forward(v));
                            forward_visited[v] = true;
                        }
                    }
                    VisitType::Backtrack(u) => {
                        heap.push((priority, u));
                        priority += 1;
                    }
                }
            }
        }

        let mut topological_rank = 0;
        let mut backward_visited = vec![false; self.n];
        while let Some((_, u)) = heap.pop() {
            if backward_visited[u] {
                continue;
            }

            let mut stack = VecDeque::new();
            stack.push_front(u);
            self.topological_ranks[u] = topological_rank;
            backward_visited[u] = true;

            while let Some(v) = stack.pop_front() {
                for &(r, _) in self.backward_graph.graph[v].iter() {
                    if backward_visited[r] {
                        continue;
                    }
                    stack.push_front(r);
                    self.topological_ranks[r] = topological_rank;
                    backward_visited[r] = true;
                }
            }

            topological_rank += 1;
        }

        topological_rank
    }

}

#[cfg(test)]
mod test_scc {
    use crate::graph::graph::Graph;
    use crate::graph::scc::StoronglyConnectedComponent;

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

        let mut scc = StoronglyConnectedComponent::new(graph);
        assert_eq!(scc.scc(), 4);
        assert_eq!(scc.topological_ranks, vec![3, 1, 2, 3, 1, 0]);
    }
}
