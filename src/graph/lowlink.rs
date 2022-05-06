#[derive(Debug,Clone)]
pub struct Lowlink {
    n: usize,
    graph: Vec<Vec<usize>>,
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}

#[allow(clippy::needless_range_loop)]
impl Lowlink {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n: usize = graph.len();
        let used = vec![false; n];
        let ord: Vec<usize> = vec![0; n];
        let low: Vec<usize> = vec![0; n];
        let bridges: Vec<(usize, usize)> = vec![];

        Self {
            n,
            graph,
            used,
            ord,
            low,
            bridges,
        }
    }

    pub fn search(&mut self) {
        let mut k: usize = 0;

        for u in 0..self.n {
            if self.used[u] {
                continue;
            }
            k = self.dfs(u, k, None);
        }
    }

    pub fn dfs(&mut self, u: usize, mut k: usize, p: Option<usize>) -> usize {
        self.used[u] = true;

        self.ord[u] = k;
        self.low[u] = self.ord[u];
        k += 1;

        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];

            if !self.used[v] {
                k = self.dfs(v, k, Some(u));
                self.low[u] = self.low[u].min(self.low[v]);

                if self.ord[u] < self.low[v] {
                    self.bridges.push((u.min(v), u.max(v)));

                }
            }
            else if p.is_some() && v != p.unwrap() {
                self.low[u] = self.low[u].min(self.ord[v]);
            }
        }

        k
    }

    pub fn num_bridges(&mut self) -> usize {
        self.bridges.len()
    }
}


#[cfg(test)]
mod test_lowlink {
    use crate::graph::lowlink::Lowlink;

    pub fn build_graph(n: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; n];
        for (u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }

        graph
    }

    #[test]
    fn it_works() {
        let edges = vec![
            (0, 2),
            (1, 6),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 5),
            (5, 6),
        ];

        let graph = build_graph(7, edges);
        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.num_bridges(), 4);
    }

    #[test]
    fn it_works_without_bridge() {
        let edges = vec![
            (0, 1),
            (0, 2),
            (1, 2),
        ];

        let graph = build_graph(3, edges);
        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.num_bridges(), 0);
    }

    #[test]
    fn it_works_with_all_edges_are_bridges() {
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
        ];

        let graph = build_graph(6, edges);
        let mut lowlinke = Lowlink::new(graph);
        lowlinke.search();

        assert_eq!(lowlinke.num_bridges(), 5);
    }
}
