use std::{collections::BinaryHeap, cmp::Reverse};

const INF: usize = 1001001001;

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
        use crate::graph::dijkstra;
        {
            let mut graph = vec![vec![]; 7];
            graph[0].push((1, 2));
            graph[0].push((2, 5));
            graph[1].push((2, 4));
            graph[1].push((3, 6));
            graph[1].push((4, 10));
            graph[2].push((3, 2));
            graph[3].push((5, 1));
            graph[4].push((5, 5));
            graph[4].push((6, 5));
            graph[5].push((6, 9));

            let mut dijkstra = dijkstra::Dijkstra::new(graph);
            assert_eq!(dijkstra.search(0), vec![0, 2, 5, 7, 12, 8, 17]);
        }

        {
            let mut graph = vec![vec![]; 7];
            graph[0].push((1, 1));
            graph[0].push((6, 10));
            graph[1].push((2, 5));
            graph[1].push((3, 2));
            graph[3].push((4, 2));
            graph[4].push((5, 3));
            graph[5].push((6, 3));

            let mut dijkstra = dijkstra::Dijkstra::new(graph);
            assert_eq!(dijkstra.search(0), vec![0, 1, 6, 3, 5, 8, 10]);
        }
    }
}
