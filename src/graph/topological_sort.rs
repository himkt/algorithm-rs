use std::{cmp::Reverse, collections::BinaryHeap};

pub struct TopologicalSort {
    g: Vec<Vec<usize>>,
    deg: Vec<usize>,
}

impl TopologicalSort {
    pub fn new(g: Vec<Vec<usize>>, deg: Vec<usize>) -> Self {
        TopologicalSort { g, deg }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s: BinaryHeap<_> = BinaryHeap::new();

        for v in 0..self.g.len() {
            if self.deg[v] == 0 {
                s.push(Reverse(v));
            }
        }

        while let Some(Reverse(v)) = s.pop() {
            ans.push(v);

            for &nv in self.g[v].iter() {
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
    #[test]
    fn it_works() {
        use crate::graph::topological_sort::TopologicalSort;
        {
            let g: Vec<Vec<usize>> = vec![vec![], vec![0, 3], vec![3], vec![]];
            let deg: Vec<usize> = vec![1, 0, 0, 2];
            let mut sorter = TopologicalSort::new(g, deg);
            assert_eq!(sorter.sort(), vec![1, 0, 2, 3]);

            let g: Vec<Vec<usize>> = vec![vec![1], vec![0]];
            let deg: Vec<usize> = vec![1, 1];
            let mut sorter = TopologicalSort::new(g, deg);
            assert_eq!(sorter.sort(), vec![]);
        }
    }
}
