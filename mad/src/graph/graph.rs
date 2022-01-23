/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug)]
struct Graph<N, E> {
    nodes: Vec<(Vec<usize>, N)>,
    edges: Vec<(usize, E)>,
}

impl<N, E> Graph<N, E> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// iterate node
    fn node_iter<'a>(&'a self) -> impl Iterator + 'a {
        self.nodes.iter()
    }

    /// mut iterate node
    fn node_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        self.nodes.iter_mut()
    }

    
    /// iterate edge
    fn edge_iter<'a>(&'a self) -> impl Iterator + 'a {
        self.edges.iter()
    }

    /// mut iterate edge
    fn edge_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        self.edges.iter_mut()
    }

    /// push a node to the end
    /// return id for the node
    fn add_node(&mut self, value: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push((Vec::new(), value));
        index
    }

    /// nodes (from, to)
    fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        let (from, to) = nodes;
        let index = self.edges.len();
        self.nodes[from].0.push(index);
        self.edges.push((to, value));
        index
    }

    fn edit_node(&mut self, id: usize, value: N) {
        self.nodes[id].1 = value;
    }

    fn edit_edge(&mut self, id: usize, value: E) {
        self.edges[id].1 = value;
    }
}
