/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<(Vec<usize>, N)>,
    edges: Vec<(usize, E)>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// iterate node
    pub fn node_iter<'a>(&'a self) -> impl Iterator + 'a {
        self.nodes.iter()
    }

    /// mut iterate node
    pub fn node_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        self.nodes.iter_mut()
    }

    /// iterate edge
    pub fn edge_iter<'a>(&'a self) -> impl Iterator + 'a {
        self.edges.iter()
    }

    /// mut iterate edge
    pub fn edge_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        self.edges.iter_mut()
    }

    /// push a node to the end
    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push((Vec::new(), value));
        index
    }

    /// nodes (from, to)
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        let (from, to) = nodes;
        let index = self.edges.len();
        self.nodes[from].0.push(index);
        self.edges.push((to, value));
        index
    }

    pub fn edit_node(&mut self, id: usize, value: N) {
        self.nodes[id].1 = value;
    }

    pub fn edit_edge(&mut self, id: usize, value: E) {
        self.edges[id].1 = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, id: usize) -> &(Vec<usize>, N) {
        &self.nodes[id]
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, id: usize) -> &(usize, E) {
        &self.edges[id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut g = Graph::<usize, usize>::new();
        for i in 0..5 {
            g.add_node(i);
            for j in 0..5 {
                if i != j {
                    g.add_edge(i * j, (i, j));
                }
            }
        }
        dbg!(g);
    }
}
