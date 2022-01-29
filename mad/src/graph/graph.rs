/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug)]
pub struct Graph<N, E> {
    pub children: Vec<Vec<usize>>,
    // Vec<usize> : an vec of out-degree node id(N)
    pub parents: Vec<usize>,
    edges: Vec<E>,
    // lookup table for edge id and edge value(E)
    nodes: Vec<N>,
    // lookup table for edge id and node value(N)
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            parents: Vec::new(),
            edges: Vec::new(),
            nodes: Vec::new(),
        }
    }

    /// iterate node
    pub fn node_iter<'a>(&'a self) -> impl Iterator + 'a {
        todo!();
        self.nodes.iter()
    }

    /// mut iterate node
    pub fn node_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        todo!();
        self.nodes.iter_mut()
    }

    /// iterate edge
    pub fn edge_iter<'a>(&'a self) -> impl Iterator + 'a {
        todo!();
        self.edges.iter()
    }

    /// mut iterate edge
    pub fn edge_iter_mut<'a>(&'a mut self) -> impl Iterator + 'a {
        todo!();
        self.edges.iter_mut()
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(value);

        self.parents.push(index);
        self.children.push(vec![]);

        index
    }

    /// nodes (to, from)
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        let index = self.edges.len();
        self.edges.push(value);

        let (from, to) = nodes;
        self.parents[from] = to;

        self.children[to].push(from);

        self.parents[from] = to;

        index
    }

    pub fn edit_node(&mut self, id: usize, value: N) {
        self.nodes[id] = value;
    }

    pub fn edit_edge(&mut self, id: usize, value: E) {
        self.edges[id] = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, id: usize) -> &N {
        &self.nodes[id]
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, id: usize) -> &E {
        &self.edges[id]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        // I don't know how to test this
        // may be bc of the rule break
        // let mut g = Graph::<usize, usize>::new();
        // for i in 0..5 {
        //     g.add_node(i);
        //     for j in 0..5 {
        //         if i != j {
        //             g.add_edge(i * j, (i, j));
        //         }
        //     }
        // }
        // dbg!(g);
        let mut g: Graph<&str, &str> = Graph::new();

        let root_name = "root";
        let node_name = vec!["a", "b", "c"];

        let root_id = g.add_node(root_name);

        for c in node_name {
            let id = g.add_node(c);
            g.add_edge(c, (root_id, id));
        }

        dbg!(g);
    }
}
