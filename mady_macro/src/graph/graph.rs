use std::collections::LinkedList;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node<N, E>(usize, PhantomData<N>, PhantomData<E>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge<N, E>(usize, usize, PhantomData<N>, PhantomData<E>);

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
    node_mix: Vec<(N, Vec<(E, usize)>)>, // extend of children
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            node_mix: vec![], //children
        }
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> Node<N, E> {
        let index = self.node_mix.len();
        self.node_mix.push((value, vec![]));
        Node::new(index)
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, from_to: (&Node<N, E>, &Node<N, E>)) -> Edge<N, E> {
        self.node_mix[from_to.0 .0].1.push((value, from_to.1 .0));
        Edge::new(from_to.0 .0, from_to.1 .0)
    }

    /// modify node value
    pub fn edit_node(&mut self, node: &Node<N, E>, value: N) {
        self.node_mix[node.index()].0 = value;
    }

    /// modify edge value
    pub fn edit_edge(&mut self, edge: &Edge<N, E>, value: E) {
        self.node_mix[edge.0].1[edge.1].0 = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, node: &Node<N, E>) -> &N {
        &self.node_mix[node.index()].0
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, edge: &Edge<N, E>) -> &E {
        &self.node_mix[edge.0].1[edge.1].0
    }

    // O(n^2)
    // N is amount of edge
    /// use topolohival sort to get the order of caculation
    pub fn topological_iter<'a>(&'a self) -> impl Iterator<Item = Node<N, E>> + 'a {
        // let in_degree: Vec<usize> = self.parents.iter().map(|x| x.len()).collect();
        let mut in_degree = vec![0; self.node_mix.len()];
        self.node_mix
            .iter()
            .map(|x| &x.1)
            .for_each(|y| y.iter().for_each(|z| in_degree[z.1] = in_degree[z.1] + 1));
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
            for c in self.graph.node_mix[out].1.iter().map(|x| x.1) {
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

    pub fn children<'a>(&mut self, graph: &'a Graph<N, E>) -> Vec<Edge<N, E>> {
        let parent = self.index();
        let h: Vec<Edge<N, E>> = graph.node_mix[self.index()]
            .1
            .iter()
            .map(|x| Edge::new(parent, x.1))
            .collect();
        h
    }

    // pub fn parents<'a>(&mut self, graph: &'a Graph<N, E>) -> &'a Vec<usize> {
    //     &graph.parents[self.index()]
    // }

    pub fn link(&self, graph: &mut Graph<N, E>, value: E, to: &Node<N, E>) {
        graph.add_edge(value, (self, to));
    }
}

impl<N, E> Edge<N, E> {
    pub fn new(f: usize, t: usize) -> Self {
        Self(f, t, PhantomData, PhantomData)
    }

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn value<'a>(&self, graph: &'a Graph<N, E>) -> &'a E {
        &graph.node_mix[self.0].1[self.1].0
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
