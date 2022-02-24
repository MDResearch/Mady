use crate::graph;

pub struct Node<'a, N, E> {
    pub index: usize,
    g: &'a Graph<N, E>,
}

pub struct IterTopological<'a, N, E> {
    zero: std::collections::LinkedList<usize>,
    in_degree: Vec<usize>,
    g: &'a Graph<N, E>,
}

/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    children: Vec<Vec<usize>>,
    // (parents, children)
    table_edge: Vec<E>,
    // lookup table for edge id and edge value(E)
    tabel_node: Vec<N>,
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
            tabel_node: vec![],
            table_edge: vec![],
        }
    }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        self.tabel_node.push(value);
        self.children.push(vec![]);
        self.tabel_node.len() - 1
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        self.children[nodes.0].push(nodes.1);
        self.table_edge.push(value);
        self.table_edge.len() - 1
    }

    /// modify node value
    pub fn edit_node(&mut self, id: usize, value: N) {
        self.tabel_node[id] = value;
    }

    /// modify edge value
    pub fn edit_edge(&mut self, id: usize, value: E) {
        self.table_edge[id] = value;
    }

    /// read node value
    /// ([edge id], value)
    pub fn node(&self, id: usize) -> &N {
        &self.tabel_node[id]
    }

    /// read edge value
    /// (to node, value)
    pub fn edge(&self, id: usize) -> &E {
        &self.table_edge[id]
    }

    /// return the roots of the data-flow graph
    #[deprecated]
    pub fn roots(&self) -> Vec<usize> {
        let mut ans = vec![];
        let mut in_degree = vec![0; self.tabel_node.len()];
        self.children
            .iter()
            .for_each(|x| x.iter().for_each(|&y| in_degree[y] = in_degree[y] + 1));
        for c in 0..in_degree.len() {
            if (in_degree[c] == 0) {
                ans.push(c);
            }
        }
        ans
    }

    // O(n^2)
    // N is amount of edge
    /// use topolohival sort to get the order of caculation
    pub fn topological_iter<'a>(&'a self) -> impl Iterator<Item = Node<'a, N, E>> + 'a {
        let mut in_degree = vec![0; self.tabel_node.len()];
        self.children
            .iter()
            .for_each(|x| x.iter().for_each(|&y| in_degree[y] = in_degree[y] + 1));
        let mut zero = std::collections::LinkedList::new();
        for c in 0..in_degree.len() {
            if (in_degree[c] == 0) {
                zero.push_back(c);
            }
        }
        IterTopological {
            zero,
            g: &self,
            in_degree,
        }
    }
}

impl<'b, N, E> Iterator for IterTopological<'b, N, E> {
    type Item = Node<'b, N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        if (!self.zero.is_empty()) {
            let out = self.zero.pop_back().unwrap();
            for &c in self.g.children[out].iter() {
                self.in_degree[c] = self.in_degree[c] - 1;
                if (self.in_degree[c] == 0) {
                    self.zero.push_back(c);
                }
            }
            return Some(Node {
                g: self.g,
                index: out,
            });
        }
        None
    }
}

impl<'a, N, E> Node<'a, N, E> {
    pub fn children(&mut self) -> &Vec<usize> {
        &self.g.children[self.index]
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

        let nodes: Vec<usize> = g.topological_iter().map(|x| x.index).collect();

        dbg!(&nodes);

        assert_eq!(nodes, vec![node_f, node_e]);
    }
}
