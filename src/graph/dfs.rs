use std::collections::VecDeque;

use crate::graph::graph::Graph;

#[derive(Debug, Clone)]
pub struct DepthFirstSearch {
    graph: Graph,
    seen: Vec<bool>,
    backptr: Vec<Option<usize>>,
    forward_history: Vec<usize>,
    backward_history: Vec<usize>,
}

#[derive(Debug)]
pub enum NodeType {
    Forward(usize),
    Backward(usize),
}

impl DepthFirstSearch {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            seen: vec![false; n],
            backptr: vec![None; n],
            forward_history: vec![],
            backward_history: vec![],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root);
    }

    pub fn dfs(&mut self, u: usize) {
        let mut stack = VecDeque::new();
        stack.push_front(NodeType::Backward(u));
        stack.push_front(NodeType::Forward(u));

        self.seen[u] = true;
        self.forward_history.push(u);

        while let Some(node_type) = stack.pop_front() {
            match node_type {
                NodeType::Forward(u)  => {
                    for &(v, _) in self.graph.graph[u].iter() {
                        if self.seen[v] {
                            continue;
                        }
                        stack.push_front(NodeType::Backward(v));
                        stack.push_front(NodeType::Forward(v));
                        self.backptr[v] = Some(u);
                        self.seen[v] = true;
                        self.forward_history.push(v);
                    }
                }
                NodeType::Backward(u) => {
                    self.backward_history.push(u);
                }
            }
        }
    }
}

#[cfg(test)]
mod test_dfs {
    use crate::graph::dfs::DepthFirstSearch;
    use crate::graph::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(6, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);
        graph.connect_unweighted(2, 5);

        let mut dfs = DepthFirstSearch::new(graph);
        dfs.search(0);
        assert_eq!(dfs.seen, vec![true, true, true, false, true, true]);
        assert_eq!(dfs.backptr, vec![None, Some(0), Some(1), None, Some(2), Some(2)]);
        assert_eq!(dfs.forward_history, vec![0, 1, 2, 4, 5]);
        assert_eq!(dfs.backward_history, vec![5, 4, 2, 1, 0]);
    }
}
