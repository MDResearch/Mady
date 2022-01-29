use super::Graph;

/// 找到所有有向圖裡的頂點(其他點無法到達的點)，輸出頂點 id 陣列
/// 頂點是指啥?葉節點?

// #[test]
pub fn get_roots<N, E>(graph: &Graph<N, E>) -> Vec<usize> {
    //let to : [ usize ; graph.node.len()] = [];
    // let mut to = [0; graph.node.len()];
    // let mut top: Vec<usize>;

    let mut ancestors = graph.parents.clone();

    // some buggy thing happend
    // consider using function (instead of loop) to achieve this
    for node in graph.parents.clone() {
        let mut c = node;
        while (graph.parents[c] != c && ancestors[c] != ancestors[graph.parents[c]]) {
            c = ancestors[c];
            ancestors[c] = ancestors[c];
        }
    }

    ancestors.sort();

    let mut ans = vec![];

    let mut l = ancestors[0];
    ans.push(l);

    for node in ancestors {
        if node != l {
            ans.push(l);
            l = node;
        }
    }
    // if graph.node.len() * graph.node.len() >= graph.edge.len() {
    //     for i in 0..graph.edge.len() {
    //         //
    //     }
    // } else {
    //     for i in 0..graph.node.len() {
    //         for j in 0..graph.node[i].len() {
    //             to[j] += 1;
    //         }
    //     }

    //     for i in 0..graph.node.len() {
    //         if to[i] == 0 {
    //             top.push(i);
    //         }
    //     }
    // }

    ans
}
//阿下面要幹嘛

// 還在想
// pub fn think_the_name1<N,E>(graph: &Graph<N,E>) {
//     unimplemented!()
// }

// pub fn think_the_name2<N,E>(graph: &Graph<N,E>) {
//     unimplemented!()
// }

// pub fn think_the_name3<N,E>(graph: &Graph<N,E>) {
//     unimplemented!()
// }

#[test]
pub fn test() {
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

    dbg!(get_roots(&g));

    g.add_edge("edge to union two roots", (root1_id, root2_id));

    dbg!(get_roots(&g));

    // 無解，我要烙幹了
}
