use std::{collections::BinaryHeap, cmp::Reverse};

const INF: usize = 100_000_000_000_000_000;


#[derive(Debug, Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, usize)>>,
}


impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, usize)>>) -> Self {
        Self { graph }
    }

    pub fn search(&mut self, src: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), src));

        while let Some((Reverse(current_cost), current_v)) = queue.pop() {
            if dist[current_v] < current_cost {
                continue;
            }

            for &(v, cost) in self.graph[current_v].iter() {
                if dist[v] > current_cost + cost {
                    dist[v] = current_cost + cost;
                    queue.push((Reverse(dist[v]), v));
                }
            }
        }

        dist
    }
}


#[cfg(test)]
mod test_dijkstra {

    #[test]
    fn it_works() {
        use crate::graph::graph::WeightedGraphBuilder;
        use crate::graph::dijkstra::Dijkstra;

        {
            let mut graph = WeightedGraphBuilder::new(7, true);
            graph.connect(0, 1, 2);
            graph.connect(0, 2, 5);
            graph.connect(1, 2, 4);
            graph.connect(1, 3, 5);
            graph.connect(1, 4, 10);
            graph.connect(2, 3, 2);
            graph.connect(3, 5, 1);
            graph.connect(4, 5, 5);
            graph.connect(4, 6, 5);
            graph.connect(5, 6, 9);

            let mut dijkstra = Dijkstra::new(graph.graph);
            assert_eq!(dijkstra.search(0), vec![0, 2, 5, 7, 12, 8, 17]);
        }

        {
            let mut graph = WeightedGraphBuilder::new(7, true);
            graph.connect(0, 1, 1);
            graph.connect(0, 6, 10);
            graph.connect(1, 2, 5);
            graph.connect(1, 3, 2);
            graph.connect(3, 4, 2);
            graph.connect(4, 5, 3);
            graph.connect(5, 6, 3);

            let mut dijkstra = Dijkstra::new(graph.graph);
            assert_eq!(dijkstra.search(0), vec![0, 1, 6, 3, 5, 8, 10]);
        }
    }
}
