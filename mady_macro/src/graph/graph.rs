use crate::graph;

/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug)]
pub struct Graph<N, E> {
    pub children: Vec<Vec<usize>>,
    // Vec<usize> : an vec of out-degree node id(N)
    disjoint_set: Vec<usize>,
    edges: Vec<E>,
    // lookup table for edge id and edge value(E)
    nodes: Vec<N>,
    // lookup table for edge id and node value(N)
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            disjoint_set: vec![],
            edges: vec![],
            nodes: vec![],
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

    pub fn bfs<'a>(&'a self, root: usize) -> impl Iterator + 'a {
        let mut r = vec![];
        let mut st = std::collections::LinkedList::from([root]);
        while (!st.is_empty()) {
            let c = st.pop_back();
            match c {
                Some(x) => {
                    self.children[x].iter().for_each(|x| st.push_front((*x)));
                    r.push(x);
                }
                None => panic!(),
            }
        }
        r.into_iter().map(|x| self.node(x))
    }

    // pub fn bfs_iter<'a>(&'a self, root: usize) -> impl Iterator + 'a {
    //     struct iter_bfs<N, E> {
    //         st: std::collections::LinkedList<usize>,
    //         c: usize,
    //         g: &'b Graph<N, E>,
    //     }

    //     impl<N, E> iter_bfs<N, E> {
    //         fn new(root: usize, g: &Graph<N, E>) -> Self {
    //             iter_bfs {
    //                 st: std::collections::LinkedList::from([root]),
    //                 c: 0,
    //                 g,
    //             }
    //         }
    //     }

    //     impl<N, E> Iterator for iter_bfs<N, E> {
    //         type Item = usize;
    //         fn next(&mut self) -> Option<Self::Item> {
    //             if (self.st.is_empty()) {
    //                 None
    //             } else {
    //                 let n = self.st.pop_back();
    //                 match n {
    //                     Some(x) => {
    //                         self.g.children[x].for_each(|x| self.st.push_front(x));
    //                         return n;
    //                     }
    //                     None => return None,
    //                 }
    //             }
    //         }
    //     }

    //     iter_bfs::new(root, self)
    // }

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(value);

        self.disjoint_set.push(index);
        self.children.push(vec![]);

        index
    }

    /// nodes (parents, children)
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        let index = self.edges.len();
        self.edges.push(value);

        let (parents, children) = nodes;

        self.disjoint_set[children] = parents;
        self.children[parents].push(children);

        index
    }

    /// modify node value
    pub fn edit_node(&mut self, id: usize, value: N) {
        self.nodes[id] = value;
    }

    /// modify edge value
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

    /// return the roots of the data-flow graph
    pub fn roots(&self) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();

        for c in 0..self.disjoint_set.len() {
            if self.disjoint_set[c] == c {
                ans.push(c);
            }
        }

        ans
    }

    // O(n^2)
    // N is amount of node

    /// use topolohival sort to get the order of caculation
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();

        // sort nodes into tuple (in-degree,out-degree)
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); self.disjoint_set.len()];

        for i in 0..self.children.len() {
            self.children[i].iter().for_each(|&to| {
                nodes[i].0 += 1;
                nodes[to].1 += 1;
            });
        }

        while ans.len() != self.disjoint_set.len() {
            for c in 0..nodes.len() {
                if nodes[c].0 == 0 {
                    ans.push(c);
                    if self.disjoint_set[c] != c {
                        nodes[self.disjoint_set[c]].0 = nodes[self.disjoint_set[c]].0 - 1;
                    }
                }
            }
        }

        ans
    }
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self::new()
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
    pub fn roots_linear() {
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

    // #[test]
    // fn dfs() {
    //     let mut g: Graph<&str, &str> = Graph::new();

    //     let node_a = g.add_node("a");
    //     let node_b = g.add_node("b");
    //     let node_c = g.add_node("c");
    //     let node_d = g.add_node("d");

    //     g.add_edge("", (node_b, node_a));
    //     g.add_edge("", (node_c, node_b));
    //     g.add_edge("", (node_c, node_d));
    //     // c b d a
    //     let r = g.dfs(g.roots()[0]);
    //     assert_eq!(r, vec![node_c, node_d, node_b, node_a])
    // }

    #[test]
    fn topological() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));

        assert_eq!(g.topological_sort(), vec![node_a, node_b, node_c]);
    }
}
