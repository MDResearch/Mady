use std::collections::LinkedList;
use std::marker::{Copy, PhantomData};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node(usize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge(usize, usize);

pub struct IterTopological<'a, N, E> {
    zero: LinkedList<usize>,
    in_degree: Vec<usize>,
    graph: &'a Graph<N, E>,
}

/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    table: Vec<(N, Vec<(E, usize)>)>, // extend of children
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            table: vec![], //children
        }
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> Node {
        let index = self.table.len();
        self.table.push((value, vec![]));
        Node::new(index)
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, from_to: (Node, Node)) -> Edge {
        let index = self.table[from_to.0.index()].1.len();
        self.table[from_to.0.index()]
            .1
            .push((value, from_to.1.index()));
        Edge::new(from_to.0.index(), index)
    }

    /// read node weight value
    pub fn node_weight(&self, node: Node) -> &N {
        &self.table[node.index()].0
    }

    /// modify node weight value
    pub fn node_weight_mut(&mut self, node: Node) -> &mut N {
        &mut self.table[node.index()].0
    }

    /// edge - to - node
    pub fn to_node(&self, edge: Edge) -> Node {
        let index = edge.index();
        Node::new(self.table[index.0].1[index.1].1)
    }

    /// read edge weight value
    pub fn edge_weight(&self, edge: Edge) -> &E {
        let index = edge.index();
        &self.table[index.0].1[index.1].0
    }

    /// modify edge weight value
    pub fn edge_weight_mut(&mut self, edge: Edge) -> &mut E {
        let index = edge.index();
        &mut self.table[index.0].1[index.1].0
    }

    /// find all edges by node
    pub fn to_edges(&self, node: Node) -> impl Iterator<Item = Edge> + '_ {
        let index = node.index();
        self.table[index]
            .1
            .iter()
            .map(move |(_, i)| Edge::new(index, *i))
    }

    /// get graph root
    pub fn roots(&self) -> impl Iterator<Item = Node> + '_ {
        let mut in_degree = vec![false; self.table.len()];
        self.table
            .iter()
            .map(|x| &x.1)
            .for_each(|y| y.iter().for_each(|z| in_degree[z.1] = true));
        in_degree
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| (!x).then(|| Node::new(i)))
    }

    // get all node
    pub fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        self.table.iter().enumerate().map(|(i, ..)| Node::new(i))
    }

    // O(N^2)
    // N is amount of edge
    /// use topolohival sort to get the order of caculation
    pub fn topological_iter<'a>(&'a self) -> impl Iterator<Item = Node> + 'a {
        IterTopological::new(self)
    }
}

impl<'a, N, E> IterTopological<'a, N, E> {
    pub fn new(graph: &'a Graph<N, E>) -> Self {
        let mut in_degree = vec![0; graph.table.len()];
        graph
            .table
            .iter()
            .map(|x| &x.1)
            .for_each(|y| y.iter().for_each(|z| in_degree[z.1] += 1));
        let mut zero = std::collections::LinkedList::new();
        for c in 0..in_degree.len() {
            if in_degree[c] == 0 {
                zero.push_back(c);
            }
        }
        Self {
            zero,
            graph,
            in_degree,
        }
    }
}

impl<'a, N, E> Iterator for IterTopological<'a, N, E> {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.zero.is_empty() {
            let out = self.zero.pop_back().unwrap();
            for c in self.graph.table[out].1.iter().map(|x| x.1) {
                self.in_degree[c] -= 1;
                if self.in_degree[c] == 0 {
                    self.zero.push_back(c);
                }
            }
            return Some(Node::new(out));
        }
        None
    }
}

impl Node {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

impl Edge {
    pub fn new(from: usize, index: usize) -> Self {
        Self(from, index)
    }

    pub fn index(&self) -> (usize, usize) {
        (self.0, self.1)
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

        assert_eq!(g.topological_iter().next().unwrap(), node_e);
    }

    #[test]
    fn topological() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));

        let nodes: Vec<_> = g.topological_iter().collect();

        dbg!(&nodes);

        assert_eq!(nodes, vec![node_c, node_b, node_a]);
    }

    #[test]
    fn topological_cyclic() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");
        let node_d = g.add_node("d");
        let node_e = g.add_node("e"); // isolated
        let node_f = g.add_node("f");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));
        g.add_edge("", (node_d, node_c));
        g.add_edge("", (node_a, node_d));

        g.add_edge("", (node_a, node_c));
        g.add_edge("", (node_f, node_c));

        let nodes: Vec<_> = g.topological_iter().collect();

        dbg!(&nodes);

        assert_eq!(nodes, vec![node_f, node_e]);
    }
}
