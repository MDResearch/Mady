use super::Graph;

/// 找到所有有向圖裡的頂點(其他點無法到達的點)，輸出頂點 id 陣列
/// 
#[test]
pub fn think_the_name<N,E>(graph: &Graph<N,E>)->Vec<usize> {

    //let to : [ usize ; graph.node.len()] = [];
    let mut to = [ 0 ; graph.node.len()];
    let mut top:Vec<usize>;



    if graph.node.len()*graph.node.len() >= graph.edge.len() {
        for i in 0..graph.edge.len(){
            // 窩不知道你的edge存了甚麼
        }

    }else{

        for i in 0..graph.node.len(){
            for j in 0..graph.node[i].len(){
                to[j] += 1;
            }
        }
    
        for i in 0..graph.node.len(){
            if to[i] == 0{
                top.push(i);
            }
        }

    }


    return  top;

}//阿下面要幹嘛

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

