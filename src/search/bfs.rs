use std::collections::VecDeque;


#[derive(Debug,Clone)]
pub struct BFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
}


impl BFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if self.seen[cur] {
                continue;
            }

            self.seen[cur] = true;
            for &next in &self.graph[cur] {
                queue.push_back(next);
            }
        }
    }
}



#[cfg(test)]
mod test_bfs {
    #[test]
    fn it_works() {
        use crate::search::bfs::BFS;
        {
            let mut graph = vec![vec![]; 5];
            graph[0].push(1);
            graph[1].push(2);
            graph[2].push(4);

            let mut bfs = BFS::new(graph);
            bfs.search(0);
            assert_eq!(bfs.seen, vec![true, true, true, false, true]);
        }
    }
}
