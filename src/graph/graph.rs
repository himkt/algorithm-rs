#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    directed: bool,
}


impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        Self { n, graph, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        if !self.directed {
            self.graph[to].push((from, weight));
        }
    }

    pub fn connect_unweighted(&mut self, from: usize, to: usize) {
        self.graph[from].push((to, 1));
        if !self.directed {
            self.graph[to].push((from, 1));
        }
    }
}
