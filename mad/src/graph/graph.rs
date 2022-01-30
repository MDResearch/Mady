/// fast add edit node & edge
/// cannot remove node & edge
/// raw method only
#[derive(Debug)]
pub struct Graph<N, E> {
    pub children: Vec<Vec<usize>>,
    // Vec<usize> : an vec of out-degree node id(N)
    pub disjoint_set: Vec<usize>,
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

    /// return id for the node
    pub fn add_node(&mut self, value: N) -> usize {
        let index = self.nodes.len();
        self.nodes.push(value);

        self.disjoint_set.push(index);
        self.children.push(vec![]);

        index
    }

    /// nodes (to, from) =>to: children node
    pub fn add_edge(&mut self, value: E, nodes: (usize, usize)) -> usize {
        let index = self.edges.len();
        self.edges.push(value);

        let (to, from) = nodes;

        self.disjoint_set[to] = from;
        self.children[from].push(to);

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

    /// 找到所有有向圖裡的頂點(其他點無法到達的點)，輸出頂點 id 陣列
    /// 頂點是指啥?葉節點?

    pub fn roots(&self) -> Vec<usize> {
        // @YZ
        // 給我寫
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
                    ans.push(nodes[c].0);
                    if self.disjoint_set[c] != c {
                        nodes[self.disjoint_set[c]].0 = nodes[self.disjoint_set[c]].0 - 1;
                    }
                }
            }
        }

        ans
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
            g.add_edge(c, (id, root_id));
        }

        dbg!(g);
    }

    #[test]
    pub fn roots_linear() {
        let mut g: Graph<&str, &str> = Graph::new();

        let node_name = vec!["a", "b", "c"];

        let root1_id = g.add_node("root1");
        let root2_id = g.add_node("root2");

        for c in node_name.clone() {
            let id = g.add_node(c);
            g.add_edge(c, (id, root1_id));
        }

        for c in node_name.clone() {
            let id = g.add_node(c);
            g.add_edge(c, (id, root2_id));
        }

        assert_eq!(g.roots().sort(), vec![root1_id, root2_id].sort());

        g.add_edge("edge to union two roots", (root1_id, root2_id));

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

        g.add_edge("", (node_a, node_b));
        g.add_edge("", (node_b, node_c));
        g.add_edge("", (node_c, node_d));
        g.add_edge("", (node_d, node_a));

        g.add_edge("", (node_c, node_a));

        dbg!(g.roots());

        assert_eq!(g.roots(), vec![node_e]);
    }
}
