use crate::graph::graph::Graph;

pub struct StoronglyConnectedComponent {
    forward_graph: Graph,
    forward_seen: Vec<bool>,
    forward_visited_nodes: Vec<usize>,
    backward_graph: Graph,
    backward_seen: Vec<bool>,
    component_ids: Vec<usize>,
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
            forward_graph,
            forward_seen: vec![false; n],
            forward_visited_nodes: vec![],
            backward_graph,
            backward_seen: vec![false; n],
            component_ids: vec![0; n],
        }
    }

    pub fn scc(&mut self) -> usize {
        for u in 0..self.forward_graph.n {
            if self.forward_seen[u] {
                continue;
            }

            self.fdfs(u);
        }

        let mut component_id = 0;
        let mut revisit_orders = self.forward_visited_nodes.clone();
        revisit_orders.reverse();

        for u in revisit_orders {
            if self.backward_seen[u] {
                continue;
            }

            self.rdfs(u, component_id);
            component_id += 1;
        }

        component_id
    }

    fn fdfs(&mut self, u: usize) {
        self.forward_seen[u] = true;

        for i in 0..self.forward_graph.graph[u].len() {
            let (v, _) = self.forward_graph.graph[u][i];

            if self.forward_seen[v] {
                continue;
            }

            self.fdfs(v);
        }

        self.forward_visited_nodes.push(u);
    }

    fn rdfs(&mut self, u: usize, k: usize) {
        self.backward_seen[u] = true;
        self.component_ids[u] = k;

        for i in 0..self.backward_graph.graph[u].len() {
            let (v, _) = self.backward_graph.graph[u][i];

            if self.backward_seen[v] {
                continue;
            }

            self.rdfs(v, k);
        }
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
        assert_eq!(scc.component_ids, vec![3, 1, 2, 3, 1, 0]);
    }
}
