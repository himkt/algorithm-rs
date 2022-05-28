#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    pub in_degrees: Vec<usize>,
    pub out_degrees: Vec<usize>,
    directed: bool,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        let in_degrees = vec![0; n];
        let out_degrees = vec![0; n];
        Self {
            n,
            graph,
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

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&&(k, _)| v == k).count() > 0
    }
}

#[cfg(test)]
mod test_graph {
    use crate::graph::graph::Graph;

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
}
