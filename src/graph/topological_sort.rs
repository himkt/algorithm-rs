use crate::graph::graph::Graph;

pub struct TopologicalSort {
    graph: Graph,
}

#[allow(clippy::needless_range_loop)]
impl TopologicalSort {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s = std::collections::BinaryHeap::new();
        let mut degrees = self.graph.in_degrees.clone();

        for v in 0..self.graph.n {
            if degrees[v] == 0 {
                s.push(std::cmp::Reverse(v));
            }
        }

        while let Some(std::cmp::Reverse(v)) = s.pop() {
            ans.push(v);

            for &(nv, _) in self.graph.graph[v].iter() {
                if degrees[nv] == 0 {
                    continue;
                }

                degrees[nv] -= 1;

                if degrees[nv] == 0 {
                    s.push(std::cmp::Reverse(nv));
                }
            }
        }

        if ans.len() == degrees.len() {
            ans
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test_topological_sort {
    use crate::graph::graph::Graph;
    use crate::graph::topological_sort::TopologicalSort;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(4, true);
        graph.connect_unweighted(1, 0);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(2, 3);

        let mut sorter = TopologicalSort::new(graph);
        assert_eq!(sorter.sort(), vec![1, 0, 2, 3]);
    }

    #[test]
    fn it_works_circle() {
        let mut graph = Graph::new(2, false);
        graph.connect_unweighted(0, 1);

        let mut sorter = TopologicalSort::new(graph);
        assert_eq!(sorter.sort(), vec![]);
    }
}
