pub struct GraphBuilder {
    pub graph: Vec<Vec<usize>>,
    directed: bool,
}


pub struct WeightedGraphBuilder {
    pub graph: Vec<Vec<(usize, usize)>>,
    directed: bool,
}


impl GraphBuilder {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<usize>> = vec![vec![]; n];
        Self { graph, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        if !self.directed {
            self.graph[to].push(from);
        }
    }
}


impl WeightedGraphBuilder {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        Self { graph, directed }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        if !self.directed {
            self.graph[to].push((from, weight));
        }
    }
}
