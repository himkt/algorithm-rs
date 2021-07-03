use std::collections::BinaryHeap;

const INF: i32 = 1001001001;


#[derive(Debug,Clone)]
pub struct Dijkstra {
    graph: Vec<Vec<(usize, i32)>>,
}

impl Dijkstra {
    pub fn new(graph: Vec<Vec<(usize, i32)>>) -> Self {
        Self {
            graph,
        }
    }

    pub fn search(&mut self, src: usize) -> Vec<i32> {
        let mut dist = vec![INF; self.graph.len()];
        dist[src] = 0;

        let mut queue = BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while !queue.is_empty() {
            let cur = queue.pop().unwrap();

            if dist[cur.1] < cur.0.0 {
                continue;
            }

            for (v, cost) in self.graph[cur.1].clone() {
                if dist[v] > cur.0.0 + cost {
                    dist[v] = cur.0.0 + cost;
                    queue.push((std::cmp::Reverse(dist[v]), v));
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
        use crate::search::dijkstra;
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
    }
}
