use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    pub rev: Vec<Vec<usize>>,
    pub in_degrees: Vec<usize>,
    pub out_degrees: Vec<usize>,
    pub directed: bool,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        let in_degrees = vec![0; n];
        let out_degrees = vec![0; n];
        let rev = vec![vec![]; n];
        Self {
            n,
            graph,
            rev,
            in_degrees,
            out_degrees,
            directed,
        }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, weight));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn connect_unweighted(&mut self, from: usize, to: usize) {
        self.graph[from].push((to, 1));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, 1));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn connect_with_residual(&mut self, from: usize, to: usize, weight: usize) {
        assert!(
            self.directed,
            "connect_with_residual only works in directed graph."
        );

        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        self.graph[to].push((from, 0));
        self.out_degrees[to] += 1;
        self.in_degrees[from] += 1;

        self.rev[from].push(self.graph[to].len() - 1);
        self.rev[to].push(self.graph[from].len() - 1);
    }

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&(k, _)| &v == k).count() > 0
    }
}

#[cfg(test)]
mod test_graph {
    use crate::graph::Graph;

    #[test]
    fn it_works_directed() {
        let mut graph = Graph::new(6, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(2, 5);
        graph.connect(1, 0, 10);
        graph.connect(3, 4, 3);

        let expected = vec![
            vec![(1, 1)],
            vec![(0, 10)],
            vec![(5, 1)],
            vec![(4, 3)],
            vec![],
            vec![],
        ];
        assert_eq!(graph.graph, expected);

        assert!(graph.connected(0, 1));
        assert!(graph.connected(1, 0));
        assert!(graph.connected(2, 5));
        assert!(!graph.connected(5, 2));
        assert!(graph.connected(3, 4));
        assert!(!graph.connected(4, 3));
        assert!(!graph.connected(0, 2));
    }

    #[test]
    fn it_works_undirected() {
        let mut graph = Graph::new(6, false);
        graph.connect_unweighted(0, 1);
        graph.connect(2, 4, 10);
        graph.connect(5, 3, 5);

        let expected = vec![
            vec![(1, 1)],
            vec![(0, 1)],
            vec![(4, 10)],
            vec![(5, 5)],
            vec![(2, 10)],
            vec![(3, 5)],
        ];
        assert_eq!(graph.graph, expected);

        assert!(graph.connected(0, 1));
        assert!(graph.connected(1, 0));
        assert!(graph.connected(2, 4));
        assert!(graph.connected(4, 2));
        assert!(graph.connected(3, 5));
        assert!(graph.connected(5, 3));
        assert!(!graph.connected(1, 3));
    }

    #[test]
    fn it_works_circle() {
        let mut graph = Graph::new(2, false);
        graph.connect_unweighted(0, 1);

        let expected = vec![vec![(1, 1)], vec![(0, 1)]];

        assert_eq!(graph.graph, expected);
    }

    #[test]
    fn it_works_multiple_edge() {
        let mut graph = Graph::new(2, true);
        graph.connect(0, 1, 10);
        graph.connect_unweighted(0, 1);

        let expected = vec![vec![(1, 10), (1, 1)], vec![]];
        assert_eq!(graph.graph, expected);
    }

    #[test]
    #[should_panic(expected = "connect_with_residual only works in directed graph.")]
    fn it_does_not_work_residual() {
        let mut graph = Graph::new(2, false);
        graph.connect_with_residual(0, 1, 0)
    }
}

#[derive(Debug, Clone)]
pub struct BreadthFirstSearch {
    graph: Graph,
    dist: Vec<usize>,
}

impl BreadthFirstSearch {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            dist: vec![Self::INF; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        self.dist[root] = 0;

        while let Some(cur) = queue.pop_front() {
            for &(next, _) in self.graph.graph[cur].iter() {
                if self.dist[next] <= self.dist[cur] + 1 {
                    continue;
                }

                self.dist[next] = self.dist[next].min(self.dist[cur] + 1);
                queue.push_back(next);
            }
        }
    }
}

#[cfg(test)]
mod test_bfs {
    use crate::graph::BreadthFirstSearch;
    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(5, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);

        let mut bfs = BreadthFirstSearch::new(graph);
        bfs.search(0);
        assert_eq!(bfs.dist, vec![0, 1, 2, BreadthFirstSearch::INF, 3]);
    }
}

#[derive(Debug, Clone)]
pub struct DepthFirstSearch {
    graph: Graph,
    seen: Vec<bool>,
    backptr: Vec<Option<usize>>,
    forward_history: Vec<usize>,
    backward_history: Vec<usize>,
}

#[derive(Debug)]
pub enum NodeType {
    Forward(usize),
    Backward(usize),
}

impl DepthFirstSearch {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        Self {
            graph,
            seen: vec![false; n],
            backptr: vec![None; n],
            forward_history: vec![],
            backward_history: vec![],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root);
    }

    pub fn dfs(&mut self, u: usize) {
        let mut stack = VecDeque::new();
        stack.push_front(NodeType::Backward(u));
        stack.push_front(NodeType::Forward(u));

        self.seen[u] = true;
        self.forward_history.push(u);

        while let Some(node_type) = stack.pop_front() {
            match node_type {
                NodeType::Forward(u)  => {
                    for &(v, _) in self.graph.graph[u].iter() {
                        if self.seen[v] {
                            continue;
                        }
                        stack.push_front(NodeType::Backward(v));
                        stack.push_front(NodeType::Forward(v));
                        self.backptr[v] = Some(u);
                        self.seen[v] = true;
                        self.forward_history.push(v);
                    }
                }
                NodeType::Backward(u) => {
                    self.backward_history.push(u);
                }
            }
        }
    }
}

#[cfg(test)]
mod test_dfs {
    use crate::graph::DepthFirstSearch;
    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(6, true);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 4);
        graph.connect_unweighted(2, 5);

        let mut dfs = DepthFirstSearch::new(graph);
        dfs.search(0);
        assert_eq!(dfs.seen, vec![true, true, true, false, true, true]);
        assert_eq!(dfs.backptr, vec![None, Some(0), Some(1), None, Some(2), Some(2)]);
        assert_eq!(dfs.forward_history, vec![0, 1, 2, 4, 5]);
        assert_eq!(dfs.backward_history, vec![5, 4, 2, 1, 0]);
    }
}

#[derive(Debug, Clone)]
pub struct Dijkstra {
    source: usize,
    graph: Graph,
    pub dist: Vec<usize>,
    backptrs: Vec<usize>,
}

impl Dijkstra {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let dist: Vec<usize> = vec![Self::INF; graph.n];
        let backptrs: Vec<usize> = (0..graph.n).collect();
        Self {
            source: Self::INF,
            graph,
            dist,
            backptrs,
        }
    }

    pub fn search(&mut self, src: usize) {
        self.source = src;

        let mut dist = vec![Self::INF; self.graph.n];
        dist[src] = 0;

        let mut queue = std::collections::BinaryHeap::new();
        queue.push((std::cmp::Reverse(0), src));

        while let Some((std::cmp::Reverse(current_cost), current_v)) = queue.pop() {
            for &(v, cost) in self.graph.graph[current_v].iter() {
                if dist[v] <= current_cost + cost {
                    continue;
                }
                dist[v] = current_cost + cost;
                queue.push((std::cmp::Reverse(dist[v]), v));
                self.backptrs[v] = current_v;
            }
        }

        self.dist = dist;
    }

    pub fn shortest_path(&self, u: usize, v: usize) -> Vec<(usize, usize)> {
        assert_eq!(u, self.source);

        if self.dist[v] == Self::INF {
            return vec![];
        }

        let mut path = std::collections::VecDeque::new();

        let mut cursor = v;
        while cursor != u {
            path.push_front((self.backptrs[cursor], cursor));
            cursor = self.backptrs[cursor];
        }

        Vec::from(path)
    }
}

#[cfg(test)]
mod test_dijkstra {
    use crate::graph::Dijkstra;
    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(7, true);
        graph.connect(0, 1, 2);
        graph.connect(0, 2, 5);
        graph.connect(1, 2, 4);
        graph.connect(1, 3, 5);
        graph.connect(1, 4, 10);
        graph.connect(2, 3, 2);
        graph.connect(3, 5, 1);
        graph.connect(4, 5, 5);
        graph.connect(4, 6, 5);
        graph.connect(5, 6, 10);

        let mut dijkstra = Dijkstra::new(graph);
        dijkstra.search(0);
        assert_eq!(dijkstra.dist, vec![0, 2, 5, 7, 12, 8, 17]);

        let expected = vec![(0, 1), (1, 4), (4, 6)];
        assert_eq!(dijkstra.shortest_path(0, 6), expected);
    }

    #[test]
    fn it_works_unreachable_path() {
        let mut graph = Graph::new(9, true);
        graph.connect(0, 1, 1);
        graph.connect(0, 6, 10);
        graph.connect(1, 2, 5);
        graph.connect(1, 3, 2);
        graph.connect(3, 4, 2);
        graph.connect(4, 5, 3);
        graph.connect(5, 6, 3);
        graph.connect(7, 0, 1);
        graph.connect(8, 7, 2);

        let mut dijkstra = Dijkstra::new(graph);
        dijkstra.search(0);
        assert_eq!(
            dijkstra.dist,
            vec![0, 1, 6, 3, 5, 8, 10, Dijkstra::INF, Dijkstra::INF]
        );

        assert_eq!(dijkstra.shortest_path(0, 3), vec![(0, 1), (1, 3)]);
        assert_eq!(dijkstra.shortest_path(0, 8), vec![]);
    }
}

pub struct EulerTour {
    graph: Graph,
    l: Vec<usize>,
    r: Vec<usize>,
    t: usize,
}

impl EulerTour {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(n: usize, graph: Graph) -> Self {
        let l = vec![Self::INF; n];
        let r = vec![Self::INF; n];
        Self { graph, l, r, t: 1 }
    }

    pub fn traverse(&mut self, root: usize) -> (&[usize], &[usize]) {
        self._dfs(root, None);

        for i in 0..self.l.len() {
            if self.r[i] == Self::INF {
                self.r[i] = self.l[i];
            }
        }

        (&self.l, &self.r)
    }

    fn _dfs(&mut self, u: usize, p: Option<usize>) {
        self.l[u] = self.t;
        self.t += 1;

        for i in 0..self.graph.graph[u].len() {
            let (v, _) = self.graph.graph[u][i];
            if p != Some(v) {
                self._dfs(v, Some(u));
                self.r[u] = self.t;
                self.t += 1;
            }
        }
    }
}

#[cfg(test)]
mod test_euler_tour {
    use crate::graph::EulerTour;
    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(7, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(3, 4);
        graph.connect_unweighted(0, 5);
        graph.connect_unweighted(5, 6);

        let mut euler_tour = EulerTour::new(7, graph);
        let (l, r) = euler_tour.traverse(0);

        assert_eq!(l, &vec![1, 2, 3, 5, 6, 10, 11]);
        assert_eq!(r, &vec![13, 8, 3, 7, 6, 12, 11]);
    }
}

pub struct FordFullkerson {
    pub graph: Graph,
    pub used: Vec<bool>,
    pub flow: usize,
}

impl FordFullkerson {
    const INF: usize = 100_000_000_000_000_000;

    pub fn new(graph: Graph) -> Self {
        let used = vec![false; graph.n];
        let flow = Self::INF;
        Self { graph, used, flow }
    }

    pub fn dfs(&mut self, u: usize, t: usize, f: usize) -> usize {
        if u == t {
            return f;
        }

        self.used[u] = true;
        for i in 0..self.graph.graph[u].len() {
            let (v, cap) = self.graph.graph[u][i];
            let r = self.graph.rev[u][i];
            if !self.used[v] && cap > 0 {
                let d = self.dfs(v, t, f.min(cap));

                if d > 0 {
                    self.graph.graph[u][i].1 -= d;
                    self.graph.graph[v][r].1 += d;
                    return d;
                }
            }
        }

        0
    }

    pub fn solve(&mut self, s: usize, t: usize) -> usize {
        let mut flow = 0;
        loop {
            self.used = vec![false; self.graph.n];
            let f: usize = self.dfs(s, t, FordFullkerson::INF);
            if f == 0 {
                return flow;
            }
            flow += f;
        }
    }
}

#[cfg(test)]
mod test_ford_fullkerson {
    use crate::graph::FordFullkerson;
    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(4, true);
        graph.connect_with_residual(0, 1, 2);
        graph.connect_with_residual(0, 2, 1);
        graph.connect_with_residual(1, 2, 1);
        graph.connect_with_residual(1, 3, 1);
        graph.connect_with_residual(2, 3, 2);

        let mut solver = FordFullkerson::new(graph);
        let maxflow = solver.solve(0, 3);
        assert_eq!(maxflow, 3);
    }
}

pub struct LowestCommonAncestor {
    parents: Vec<Vec<usize>>,
    depth: Vec<usize>,
    graph: Graph,
}

impl LowestCommonAncestor {
    const ROOT: usize = 0;
    const LOGV: usize = 30;

    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let parents = vec![vec![Self::ROOT; n]; Self::LOGV];
        let depth = vec![Self::ROOT; n];
        Self {
            parents,
            depth,
            graph,
        }
    }

    pub fn init(&mut self) {
        self.dfs(Self::ROOT);

        for k in 0..Self::LOGV - 1 {
            for v in 0..self.graph.n {
                self.parents[k + 1][v] = self.parents[k][self.parents[k][v]];
            }
        }
    }

    fn dfs(&mut self, u: usize) {
        let mut stack = VecDeque::new();
        self.parents[0][u] = u;
        self.depth[u] = 0;
        stack.push_front((u, u, 1));

        while let Some((u, p, d)) = stack.pop_front() {
            for &(v, _) in self.graph.graph[u].iter() {
                if v == p {
                    continue;
                }
                self.parents[0][v] = u;
                self.depth[v] = d + 1;
                stack.push_front((v, u, d + 1));
            }
        }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        for k in 0..Self::LOGV {
            if (((self.depth[v] - self.depth[u]) >> k) & 1) == 1 {
                v = self.parents[k][v];
            }
        }

        if u == v {
            return u;
        }

        for k in (0..Self::LOGV).rev() {
            if self.parents[k][u] != self.parents[k][v] {
                u = self.parents[k][u];
                v = self.parents[k][v];
            }
        }

        self.parents[0][u]
    }
}

#[cfg(test)]
mod test_lca {
    use crate::graph::Graph;
    use crate::graph::LowestCommonAncestor;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(8, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(0, 2);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(1, 4);
        graph.connect_unweighted(2, 5);
        graph.connect_unweighted(2, 6);
        graph.connect_unweighted(3, 7);

        let mut lca = LowestCommonAncestor::new(graph);
        lca.init();

        assert_eq!(lca.lca(0, 1), 0);
        assert_eq!(lca.lca(7, 4), 1);
        assert_eq!(lca.lca(1, 4), 1);
        assert_eq!(lca.lca(4, 1), 1);
        assert_eq!(lca.lca(7, 6), 0);
        assert_eq!(lca.lca(5, 6), 2);
        assert_eq!(lca.lca(3, 5), 0);
    }

    #[test]
    fn it_works_line() {
        let mut graph = Graph::new(5, false);
        graph.connect_unweighted(0, 1);
        graph.connect_unweighted(1, 2);
        graph.connect_unweighted(2, 3);
        graph.connect_unweighted(3, 4);

        let mut lca = LowestCommonAncestor::new(graph);
        lca.init();

        assert_eq!(lca.lca(0, 0), 0);
        assert_eq!(lca.lca(0, 1), 0);
        assert_eq!(lca.lca(0, 2), 0);
        assert_eq!(lca.lca(0, 3), 0);
        assert_eq!(lca.lca(1, 4), 1);
        assert_eq!(lca.lca(4, 1), 1);
    }
}

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
    use crate::graph::Graph;
    use crate::graph::Lowlink;

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

pub struct StronglyConnectedComponent {
    forward_graph: Graph,
    forward_visited: Vec<bool>,
    forward_visited_nodes: VecDeque<usize>,
    backward_graph: Graph,
    backward_visited: Vec<bool>,
    topological_ranks: Vec<usize>,
}

impl StronglyConnectedComponent {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let forward_graph = graph;
        let mut backward_graph = Graph::new(n, true);

        for u in 0..n {
            for &(v, _) in forward_graph.graph[u].iter() {
                backward_graph.connect_unweighted(v, u);
            }
        }

        Self {
            forward_graph,
            forward_visited: vec![false; n],
            forward_visited_nodes: VecDeque::new(),
            backward_graph,
            backward_visited: vec![false; n],
            topological_ranks: vec![0; n],
        }
    }

    pub fn scc(&mut self) -> usize {
        for u in 0..self.forward_graph.n {
            if self.forward_visited[u] {
                continue;
            }

            self.fdfs(u);
        }

        let mut topological_rank = 0;
        while let Some(u) = self.forward_visited_nodes.pop_back() {
            if self.backward_visited[u] {
                continue;
            }

            self.rdfs(u, topological_rank);
            topological_rank += 1;
        }

        topological_rank
    }

    fn fdfs(&mut self, u: usize) {
        self.forward_visited[u] = true;

        for i in 0..self.forward_graph.graph[u].len() {
            let (v, _) = self.forward_graph.graph[u][i];

            if self.forward_visited[v] {
                continue;
            }

            self.fdfs(v);
        }

        self.forward_visited_nodes.push_back(u);
    }

    fn rdfs(&mut self, u: usize, topological_rank: usize) {
        self.backward_visited[u] = true;
        self.topological_ranks[u] = topological_rank;

        for i in 0..self.backward_graph.graph[u].len() {
            let (v, _) = self.backward_graph.graph[u][i];

            if self.backward_visited[v] {
                continue;
            }

            self.rdfs(v, topological_rank);
        }
    }
}

#[cfg(test)]
mod test_scc {
    use crate::graph::Graph;
    use crate::graph::StronglyConnectedComponent;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(6, true);
        graph.connect_unweighted(1, 4);
        graph.connect_unweighted(5, 2);
        graph.connect_unweighted(3, 0);
        graph.connect_unweighted(5, 5);
        graph.connect_unweighted(4, 1);
        graph.connect_unweighted(0, 3);
        graph.connect_unweighted(4, 2);

        let mut scc = StronglyConnectedComponent::new(graph);
        assert_eq!(scc.scc(), 4);
        assert_eq!(scc.topological_ranks, vec![3, 1, 2, 3, 1, 0]);
    }
}

pub struct TopologicalSort {
    graph: Graph,
}

#[allow(clippy::needless_range_loop)]
impl TopologicalSort {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s = std::collections::BinaryHeap::new();
        let mut degrees = self.graph.in_degrees.clone();

        for v in 0..self.graph.n {
            if degrees[v] == 0 {
                s.push(std::cmp::Reverse(v));
            }
        }

        while let Some(std::cmp::Reverse(v)) = s.pop() {
            ans.push(v);

            for &(nv, _) in self.graph.graph[v].iter() {
                if degrees[nv] == 0 {
                    continue;
                }

                degrees[nv] -= 1;

                if degrees[nv] == 0 {
                    s.push(std::cmp::Reverse(nv));
                }
            }
        }

        if ans.len() == self.graph.n {
            ans
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test_topological_sort {
    use crate::graph::Graph;
    use crate::graph::TopologicalSort;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(4, true);
        graph.connect_unweighted(1, 0);
        graph.connect_unweighted(1, 3);
        graph.connect_unweighted(2, 3);

        let mut sorter = TopologicalSort::new(graph);
        assert_eq!(sorter.sort(), vec![1, 0, 2, 3]);
    }

    #[test]
    fn it_works_circle() {
        let mut graph = Graph::new(2, false);
        graph.connect_unweighted(0, 1);

        let mut sorter = TopologicalSort::new(graph);
        assert_eq!(sorter.sort(), vec![]);
    }
}
