use gfaR::Gfa;
use std::collections::HashSet;

/// Reading gfa wrapper
pub fn read_gfa(s: &str) -> gfaR::Gfa{
    let graph = gfaR::readGFA(s);
    graph

}

pub fn check_path_nodes(graph: &Gfa) -> bool{
    let mut nodes: HashSet<String> = HashSet::new();
    for path in graph.paths.iter(){
        for node in path.nodes.iter(){
            nodes.insert(node.to_owned());
        }
    }
    let nnodes: HashSet<String> = graph.nodes.keys().cloned().collect();

    let inter: HashSet<String> = nnodes.symmetric_difference(&nodes).cloned().collect();

    if inter.len() == 0{
        true
    }
    else {
        false
    }
}

// Macro I did not use
// macro_rules! zip {
//     ($x: expr) => ($x);
//     ($x: expr, $($y: expr), +) => (
//         $x.iter().zip(
//             zip!($($y), +))
//     )
// }


// https://stackoverflow.com/questions/29669287/how-can-i-zip-more-than-two-iterators
pub fn check_path_edges(graph: &Gfa) -> bool {
    let mut test: HashSet<(String, bool, String, bool)> = HashSet::new();
    for path in graph.paths.iter(){
        for i in 0..path.nodes.len()-1{
            test.insert((path.nodes[i].clone(), path.dir[i], path.nodes[i+1].clone(), path.dir[i+1]));
        }

    }
    let mut test2: HashSet<(String, bool, String, bool)> = HashSet::new();
    for edge in graph.edges.iter(){
        test2.insert((edge.from.clone(), edge.from_dir, edge.to.clone(), edge.to_dir));
    }
    let inter: HashSet<(String, bool, String, bool)>  = test.symmetric_difference(&test2).cloned().collect();
    if inter.len() == 0{
        true
    } else {
        false
    }
}







pub fn add_one(num: &mut u32){
    *num += 1;
}


pub fn is_int(graph: &Gfa) -> bool{
    let k: Vec<String> = graph.nodes.keys().cloned().collect();
    let mut err :u32 = 0;
    for node in k.iter(){
        match node.parse::<u32>() {
            Ok(_n) => (),
            Err(_e) => add_one(& mut err) ,
        }

    }
    if err > 0{
        false
    }
    else {
        true
    }
}