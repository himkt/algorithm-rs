use crate::graph::graph::Graph;


#[derive(Debug, Clone)]
pub struct Lowlink {
    graph: Graph,
    used: Vec<bool>,
    ord: Vec<usize>,
    low: Vec<usize>,
    bridges: Vec<(usize, usize)>,
}


#[allow(clippy::needless_range_loop)]
impl Lowlink {
    pub fn new(graph: Graph) -> Self {
        let n: usize = graph.n;
        let used = vec![false; n];
        let ord: Vec<usize> = vec![0; n];
        let low: Vec<usize> = vec![0; n];
        let bridges: Vec<(usize, usize)> = vec![];

        Self {
            graph,
            used,
            ord,
            low,
            bridges,
        }
    }

    pub fn search(&mut self) {
        let mut k = 0;
        for u in 0..self.graph.n {
            if self.used[u] {
                continue;
            }
            k = self.dfs(u, k, None);
        }
    }

    pub fn dfs(&mut self, u: usize, mut k: usize, p: Option<usize>) -> usize {
        self.used[u] = true;

        self.ord[u] = k;
        self.low[u] = k;
        k += 1;

        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];

            if !self.used[v] {
                k = self.dfs(v, k, Some(u));
                self.low[u] = self.low[u].min(self.low[v]);

                if self.ord[u] < self.low[v] {
                    self.bridges.push((u.min(v), u.max(v)));
                }
            } else if p.is_some() && v != p.unwrap() {
                self.low[u] = self.low[u].min(self.ord[v]);
            }
        }

        k
    }

    pub fn num_bridges(&self) -> usize {
        self.bridges.len()
    }
}

#[cfg(test)]
mod test_lowlink {
    use crate::graph::graph::Graph;
    use crate::graph::lowlink::Lowlink;

    #[test]
    fn it_works() {

        let mut graph = Graph::new(7, false);
        graph.connect_unweighted(0, 2);
        graph.connect_unweighted(1, 6);
        graph.connect_unweighted(2, 3);
        graph.connect_unweighted(3, 4);
        graph.connect_unweighted(3, 5);
        graph.connect_unweighted(4, 5);
        graph.connect_unweighted(5, 6);

        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.ord, vec![0, 6, 1, 2, 3, 4, 5]);
        assert_eq!(lowlink.low, vec![0, 6, 1, 2, 2, 2, 5]);
        assert_eq!(lowlink.num_bridges(), 4);
    }

    #[test]
    fn it_works_without_bridge() {

        let mut graph = Graph::new(3, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(0, 2);
        graph.connect_unweighted(1, 2);

        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.ord, vec![0, 1, 2]);
        assert_eq!(lowlink.low, vec![0, 0, 0]);
        assert_eq!(lowlink.num_bridges(), 0);
    }

    #[test]
    fn it_works_with_all_edges_are_bridges() {

        let mut graph = Graph::new(6, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 3);
        graph.connect_unweighted(3, 4);
        graph.connect_unweighted(4, 5);

        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.ord, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(lowlink.low, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(lowlink.num_bridges(), 5);
    }

    #[test]
    fn it_works_hand() {

        let mut graph = Graph::new(7, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(2, 4);
        graph.connect_unweighted(4, 5);
        graph.connect_unweighted(4, 6);
        graph.connect_unweighted(5, 6);

        let mut lowlink = Lowlink::new(graph);
        lowlink.search();

        assert_eq!(lowlink.ord, vec![0, 1, 2, 6, 3, 4, 5]);
        assert_eq!(lowlink.low, vec![0, 1, 2, 6, 3, 3, 3]);
        assert_eq!(lowlink.num_bridges(), 4);
    }
}
