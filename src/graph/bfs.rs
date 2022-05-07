use std::collections::VecDeque;


const INF: usize = 1001001001;


#[derive(Debug,Clone)]
pub struct BFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    dist: Vec<usize>,
}


impl BFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));

        while !queue.is_empty() {
            let (cur, dist) = queue.pop_front().unwrap();
            if self.seen[cur] {
                continue;
            }

            self.seen[cur] = true;
            self.dist[cur] = self.dist[cur].min(dist);
            for &next in &self.graph[cur] {
                queue.push_back((next, self.dist[cur] + 1));
            }
        }
    }
}



#[cfg(test)]
mod test_bfs {
    #[test]
    fn it_works() {
        use crate::graph::bfs::BFS;
        use crate::graph::bfs::INF;
        {
            let mut graph = vec![vec![]; 5];
            graph[0].push(1);
            graph[1].push(2);
            graph[2].push(4);

            let mut bfs = BFS::new(graph);
            bfs.search(0);
            assert_eq!(bfs.seen, vec![true, true, true, false, true]);
            assert_eq!(bfs.dist, vec![0, 1, 2, INF, 3]);
        }
    }
}
