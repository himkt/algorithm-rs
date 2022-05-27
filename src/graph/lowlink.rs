#[derive(Debug, Clone)]
pub struct Lowlink {
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

        let mut lowlink = Self {
            graph,
            used,
            ord,
            low,
            bridges,
        };

        let mut k = 0;

        for u in 0..n {
            if lowlink.used[u] {
                continue;
            }
            k = lowlink.dfs(u, k, None);
        }

        lowlink
    }

    pub fn dfs(&mut self, u: usize, mut k: usize, p: Option<usize>) -> usize {
        self.used[u] = true;

        self.ord[u] = k;
        self.low[u] = k;
        k += 1;

        for i in 0..self.graph[u].len() {
            let v = self.graph[u][i];

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
    use crate::graph::graph::GraphBuilder;
    use crate::graph::lowlink::Lowlink;

    #[test]
    fn it_works() {

        let mut graph = GraphBuilder::new(7, false);
        graph.connect(0, 2);
        graph.connect(1, 6);
        graph.connect(2, 3);
        graph.connect(3, 4);
        graph.connect(3, 5);
        graph.connect(4, 5);
        graph.connect(5, 6);

        let lowlink = Lowlink::new(graph.graph);
        assert_eq!(lowlink.ord, vec![0, 6, 1, 2, 3, 4, 5]);
        assert_eq!(lowlink.low, vec![0, 6, 1, 2, 2, 2, 5]);
        assert_eq!(lowlink.num_bridges(), 4);
    }

    #[test]
    fn it_works_without_bridge() {

        let mut graph = GraphBuilder::new(3, false);
        graph.connect(0, 1);
        graph.connect(0, 2);
        graph.connect(1, 2);

        let lowlink = Lowlink::new(graph.graph);
        assert_eq!(lowlink.ord, vec![0, 1, 2]);
        assert_eq!(lowlink.low, vec![0, 0, 0]);
        assert_eq!(lowlink.num_bridges(), 0);
    }

    #[test]
    fn it_works_with_all_edges_are_bridges() {

        let mut graph = GraphBuilder::new(6, false);
        graph.connect(0, 1);
        graph.connect(1, 2);
        graph.connect(2, 3);
        graph.connect(3, 4);
        graph.connect(4, 5);

        let lowlinke = Lowlink::new(graph.graph);

        assert_eq!(lowlinke.ord, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(lowlinke.low, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(lowlinke.num_bridges(), 5);
    }

    #[test]
    fn it_works_hand() {

        let mut graph = GraphBuilder::new(7, false);
        graph.connect(0, 1);
        graph.connect(1, 2);
        graph.connect(1, 3);
        graph.connect(2, 4);
        graph.connect(4, 5);
        graph.connect(4, 6);
        graph.connect(5, 6);

        let lowlink = Lowlink::new(graph.graph);
        assert_eq!(lowlink.ord, vec![0, 1, 2, 6, 3, 4, 5]);
        assert_eq!(lowlink.low, vec![0, 1, 2, 6, 3, 3, 3]);
        assert_eq!(lowlink.num_bridges(), 4);
    }
}
