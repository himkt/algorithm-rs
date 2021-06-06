#[derive(Debug,Clone)]
pub struct DFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
}

impl DFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root, &self.graph.clone());
    }

    fn dfs(&mut self, v: usize, graph: &[Vec<usize>]) {
        if self.seen[v] {
            return;
        }

        println!("visit {}", v);
        self.seen[v] = true;

        for i in 0..graph[v].len() {
            self.dfs(graph[v][i], graph);
        }
    }
}


#[cfg(test)]
mod test_dfs {
    #[test]
    fn it_works() {
        use crate::search::dfs::DFS;
        {
            let mut graph = vec![vec![]; 5];
            graph[0].push(1);
            graph[1].push(2);
            graph[2].push(4);

            let mut dfs = DFS::new(graph);
            dfs.search(0);
            assert_eq!(dfs.seen, vec![true, true, true, false, true]);
        }
    }
}
