const INF: usize = 1001001001;

#[derive(Debug,Clone)]
pub struct DFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    dist: Vec<usize>,
}

impl DFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            dist: vec![INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root, 0);
    }

    pub fn dfs(&mut self, v: usize, dist: usize) {
        if self.seen[v] {
            return;
        }

        println!("visit {}", v);
        self.seen[v] = true;
        self.dist[v] = self.dist[v].min(dist);

        for i in 0..self.graph[v].len() {
            self.dfs(self.graph[v][i], dist+1);
        }
    }
}


#[cfg(test)]
mod test_dfs {
    #[test]
    fn it_works() {
        use crate::search::dfs::DFS;
        use crate::search::dfs::INF;
        {
            let mut graph = vec![vec![]; 5];
            graph[0].push(1);
            graph[1].push(2);
            graph[2].push(4);

            let mut dfs = DFS::new(graph);
            dfs.search(0);
            assert_eq!(dfs.seen, vec![true, true, true, false, true]);
            assert_eq!(dfs.dist, vec![0, 1, 2, INF, 3]);
        }
    }
}
