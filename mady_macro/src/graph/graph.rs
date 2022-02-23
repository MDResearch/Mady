use crate::graph;
use std::collections::BTreeMap;

pub struct Node {
    index: usize,
    parents: usize,
}

pub struct IterTopological<'b> {
    out_degree: Vec<usize>,
    edges: &'b Vec<(usize, usize)>,
}

/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    edge_count: usize,
    node_count: usize,
    // in_degree: Vec<usize>,
    out_degree: Vec<usize>,
    edges: Vec<(usize, usize)>,
    // (parents, children)
    edges_value: Vec<E>,
    // lookup table for edge id and edge value(E)
    nodes_value: Vec<N>,
    // lookup table for edge id and node value(N)
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            edge_count: 0,
            node_count: 0,
            // in_degree: vec![],
            out_degree: vec![],
            edges: vec![],
            nodes_value: vec![],
            edges_value: vec![],
        }
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        self.nodes_value.push(value);
        // self.in_degree.push(0);
        self.out_degree.push(0);
        self.node_count = self.node_count + 1;
        self.node_count - 1
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        self.edges_value.push(value);
        self.edges.push(nodes);
        // self.in_degree[nodes.0] = self.in_degree[nodes.0] + 1;
        self.out_degree[nodes.1] = self.out_degree[nodes.1] + 1;
        self.edge_count = self.edge_count + 1;
        self.edge_count - 1
    }

    /// modify node value
    pub fn edit_node(&mut self, id: usize, value: N) {
        self.nodes_value[id] = value;
    }

    /// modify edge value
    pub fn edit_edge(&mut self, id: usize, value: E) {
        self.edges_value[id] = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, id: usize) -> &N {
        &self.nodes_value[id]
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, id: usize) -> &E {
        &self.edges_value[id]
    }

    /// return the roots of the data-flow graph
    pub fn roots(&self) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();
        for c in 0..self.out_degree.len() {
            if self.out_degree[c] == 0 {
                ans.push(c);
            }
        }
        ans
    }

    // O(n^2)
    // N is amount of edge
    /// use topolohival sort to get the order of caculation
    pub fn topological_iter<'a>(&'a self) -> impl Iterator<Item = Node> + 'a {
        IterTopological {
            out_degree: self.out_degree.clone(),
            edges: &self.edges,
        }
    }
}

impl<'b> Iterator for IterTopological<'b> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        for c in self.edges {
            if (self.out_degree[c.0] == 0) {
                self.out_degree[c.1] = self.out_degree[c.1] - 1;
                self.out_degree[c.0] = usize::max_value();
                return Some(Node {
                    index: c.1,
                    parents: c.0,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let mut g: Graph<&str, &str> = Graph::new();

        let root_name = "root";
        let node_name = vec!["a", "b", "c"];

        let root_id = g.add_node(root_name);

        for c in node_name {
            let id = g.add_node(c);
            g.add_edge(c, (root_id, id));
        }
    }

    #[test]
    pub fn roots() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_name = vec!["a", "b", "c"];

        let root1_id = g.add_node("root1");
        let root2_id = g.add_node("root2");

        for c in node_name.clone() {
            let id = g.add_node(c);
            g.add_edge(c, (root1_id, id));
        }

        for c in node_name.clone() {
            let id = g.add_node(c);
            g.add_edge(c, (root2_id, id));
        }

        assert_eq!(g.roots().sort(), vec![root1_id, root2_id].sort());

        g.add_edge("edge to union two roots", (root2_id, root1_id));

        assert_eq!(g.roots().sort(), vec![root2_id].sort());
    }

    #[test]
    fn roots_cyclic() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");
        let node_d = g.add_node("d");
        let node_e = g.add_node("e");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));
        g.add_edge("", (node_d, node_c));
        g.add_edge("", (node_a, node_d));

        g.add_edge("", (node_a, node_c));

        dbg!(g.roots());

        assert_eq!(g.roots(), vec![node_e]);
    }

    #[test]
    fn topological() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));

        let nodes: Vec<usize> = g.topological_iter().map(|x| x.index).collect();
        assert_eq!(nodes, vec![node_b, node_a]);
    }
}
