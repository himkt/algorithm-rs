use std::{cmp::Reverse, collections::BinaryHeap};

pub struct TopologicalSort {
    graph: Vec<Vec<usize>>,
    deg: Vec<usize>,
}

impl TopologicalSort {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n: usize = graph.len();
        let mut deg = vec![0; n];

        for row in graph.iter() {
            for &v in row.iter() {
                deg[v] += 1;
            }
        }

        TopologicalSort { graph, deg }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s: BinaryHeap<_> = BinaryHeap::new();

        for v in 0..self.graph.len() {
            if self.deg[v] == 0 {
                s.push(Reverse(v));
            }
        }

        while let Some(Reverse(v)) = s.pop() {
            ans.push(v);

            for &nv in self.graph[v].iter() {
                if self.deg[nv] == 0 {
                    continue;
                }

                self.deg[nv] -= 1;

                if self.deg[nv] == 0 {
                    s.push(Reverse(nv));
                }
            }
        }

        if ans.len() == self.deg.len() {
            ans
        } else {
            vec![]
        }
    }
}


#[cfg(test)]
mod test_topological_sort {
    use crate::graph::graph::GraphBuilder;
    use crate::graph::topological_sort::TopologicalSort;

    #[test]
    fn it_works() {

        let mut graph = GraphBuilder::new(4, true);
        graph.connect(1, 0);
        graph.connect(1, 3);
        graph.connect(2, 3);

        let mut sorter = TopologicalSort::new(graph.graph);
        assert_eq!(sorter.sort(), vec![1, 0, 2, 3]);
    }

    #[test]
    fn it_works_circle() {

        let mut graph = GraphBuilder::new(2, false);
        graph.connect(0, 1);

        let mut sorter = TopologicalSort::new(graph.graph);
        assert_eq!(sorter.sort(), vec![]);
    }
}
