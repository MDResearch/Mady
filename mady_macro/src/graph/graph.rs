use crate::graph;

/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    pub children: Vec<Vec<usize>>,
    // Vec<usize> : an vec of out-degree node id(N)
    disjoint_set: Vec<usize>,
    edges: Vec<E>,
    // lookup table for edge id and edge value(E)
    nodes: Vec<N>,
    // lookup table for edge id and node value(N)
}

#[derive(Debug)]
pub struct IterBfs<'b, N, E> {
    queue: std::collections::LinkedList<usize>,
    visited: Vec<bool>,
    graph: &'b Graph<N, E>,
}

#[derive(Debug)]
pub struct Node<'a, N, E> {
    index: usize,
    graph: &'a Graph<N, E>,
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
            disjoint_set: vec![],
            edges: vec![],
            nodes: vec![],
        }
    }

    pub fn bfs_iter<'a>(&'a self, root: usize) -> impl Iterator<Item = Node<N, E>> + 'a {
        IterBfs::new(root, self)
    }

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

impl<'b, N, E> IterBfs<'b, N, E> {
    fn new(root: usize, g: &'b Graph<N, E>) -> Self {
        IterBfs {
            queue: std::collections::LinkedList::from([root]),
            visited: vec![false; g.nodes.len()],
            graph: g,
        }
    }
}

impl<'b, N, E> Iterator for IterBfs<'b, N, E> {
    type Item = Node<'b, N, E>;
    fn next(&mut self) -> Option<Self::Item> {
        while (!self.queue.is_empty() && self.visited[*self.queue.back().unwrap()]) {
            self.queue.pop_back();
        }
        let n = self.queue.pop_back();
        if let Some(x) = n {
            self.graph.children[x]
                .iter()
                .for_each(|&x| self.queue.push_front(x));
            self.visited[x] = true;
            Some(Node {
                index: x,
                graph: self.graph,
            })
        } else {
            None
        }
    }
}

impl<'a, N, E> Node<'a, N, E> {
    pub fn parent(i: Node<N, E>) -> usize {
        i.graph.disjoint_set[i.index]
    }

    pub fn children(i: Node<N, E>) -> &Vec<usize> {
        &i.graph.children[i.index]
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
    fn bfs() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");
        let node_c = g.add_node("c");
        let node_d = g.add_node("d");
        let node_e = g.add_node("e");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_c, node_b));
        g.add_edge("", (node_c, node_d));
        g.add_edge("", (node_a, node_e));
        // c b d a
        let r: Vec<_> = g.bfs_iter(g.roots()[0]).map(|x| x.index).collect();
        assert_eq!(r, vec![node_c, node_b, node_d, node_a, node_e])
    }

    #[test]

    fn bfs_cyclic() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_a = g.add_node("a");
        let node_b = g.add_node("b");

        g.add_edge("", (node_b, node_a));
        g.add_edge("", (node_a, node_b));

        // c b d a
        let r: Vec<_> = g.bfs_iter(node_a).map(|x| x.index).collect();
        assert_eq!(r.len(), 2)
    }

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
