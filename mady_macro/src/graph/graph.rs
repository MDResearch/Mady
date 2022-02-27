use std::collections::LinkedList;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node<N, E>(usize, PhantomData<N>, PhantomData<E>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge<N, E>(usize, PhantomData<N>, PhantomData<E>);

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
    children: Vec<Vec<usize>>,
    parents: Vec<Vec<usize>>,
    table_edge: Vec<E>,
    // lookup table for edge id and edge value(E)
    table_node: Vec<N>,
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
            children: vec![],
            parents: vec![],
            table_node: vec![],
            table_edge: vec![],
        }
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> Node<N, E> {
        let index = self.table_node.len();
        self.table_node.push(value);
        self.children.push(vec![]);
        self.parents.push(vec![]);
        Node::new(index)
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, from_to: (&Node<N, E>, &Node<N, E>)) -> Edge<N, E> {
        let index = self.table_edge.len();
        self.children[from_to.0.index()].push(from_to.1.index());
        self.parents[from_to.1.index()].push(from_to.0.index());
        self.table_edge.push(value);
        Edge::new(index)
    }

    /// modify node value
    pub fn edit_node(&mut self, node: &Node<N, E>, value: N) {
        self.table_node[node.index()] = value;
    }

    /// modify edge value
    pub fn edit_edge(&mut self, edge: &Edge<N, E>, value: E) {
        self.table_edge[edge.index()] = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, node: &Node<N, E>) -> &N {
        &self.table_node[node.index()]
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, edge: &Edge<N, E>) -> &E {
        &self.table_edge[edge.index()]
    }

    // O(n^2)
    // N is amount of edge
    /// use topolohival sort to get the order of caculation
    pub fn topological_iter<'a>(&'a self) -> impl Iterator<Item = Node<N, E>> + 'a {
        let in_degree: Vec<usize> = self.parents.iter().map(|x| x.len()).collect();
        // let mut in_degree = vec![0; self.table_node.len()];
        // self.children
        //     .iter()
        //     .for_each(|x| x.iter().for_each(|&y| in_degree[y] = in_degree[y] + 1));
        let mut zero = std::collections::LinkedList::new();
        for c in 0..in_degree.len() {
            if in_degree[c] == 0 {
                zero.push_back(c);
            }
        }
        IterTopological {
            zero,
            graph: self,
            in_degree,
        }
    }
}

impl<'b, N, E> Iterator for IterTopological<'b, N, E> {
    type Item = Node<N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.zero.is_empty() {
            let out = self.zero.pop_back().unwrap();
            for &c in self.graph.children[out].iter() {
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

impl<N, E> Node<N, E> {
    pub fn new(index: usize) -> Self {
        Self(index, PhantomData, PhantomData)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn value<'a>(&self, graph: &'a Graph<N, E>) -> &'a N {
        graph.node(self)
    }

    pub fn children<'a>(&mut self, graph: &'a Graph<N, E>) -> &'a Vec<usize> {
        &graph.children[self.index()]
    }

    pub fn parents<'a>(&mut self, graph: &'a Graph<N, E>) -> &'a Vec<usize> {
        &graph.parents[self.index()]
    }

    pub fn link(&self, graph: &mut Graph<N, E>, value: E, to: &Node<N, E>) {
        graph.add_edge(value, (self, to));
    }
}

impl<N, E> Edge<N, E> {
    pub fn new(index: usize) -> Self {
        Self(index, PhantomData, PhantomData)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn value<'a>(&self, graph: &'a Graph<N, E>) -> &'a E {
        graph.edge(self)
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
            g.add_edge(c, (&root_id, &id));
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

        g.add_edge("", (&node_b, &node_a));
        g.add_edge("", (&node_c, &node_b));
        g.add_edge("", (&node_d, &node_c));
        g.add_edge("", (&node_a, &node_d));

        g.add_edge("", (&node_a, &node_c));

        assert_eq!(g.topological_iter().next().unwrap(), node_e);
    }

    #[test]
    fn topological() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");

        g.add_edge("", (&node_b, &node_a));
        g.add_edge("", (&node_c, &node_b));

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

        g.add_edge("", (&node_b, &node_a));
        g.add_edge("", (&node_c, &node_b));
        g.add_edge("", (&node_d, &node_c));
        g.add_edge("", (&node_a, &node_d));

        g.add_edge("", (&node_a, &node_c));
        g.add_edge("", (&node_f, &node_c));

        let nodes: Vec<_> = g.topological_iter().collect();

        dbg!(&nodes);

        assert_eq!(nodes, vec![node_f, node_e]);
    }
}
