mod graph_mod;
use graph_mod::sleep_graph::Graph;
type Vertex = usize;
type Edge = (Vertex, Vertex);
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[test]
fn test_graph_creation(){
    let edges = vec![(0,1),(0,2),(0,3),(3,4),(4,5),(6,7)];
    let n = 9;
    assert_eq!(Graph{n:9,outedges:vec![vec![1,2, 3],vec![0],vec![0],vec![0,4],vec![3,5],vec![4],vec![7],vec![6],vec![]]}, Graph::create_undirected(n, &edges));
}

#[test]
fn test_component_creation() {
    let edges = vec![(0,1),(0,2),(0,3),(3,4),(4,5),(6,7)];
    let graph = Graph::create_undirected(9,&edges);
    //this should result in 3 components, one with 0-5, one with 6 and 7, and one with 8
    let components = Graph::compute_components(&graph);
    assert_eq!(vec![0,1,2,3,4,5], components[0]);
    assert_eq!(vec![6,7], components[1]);
    assert_eq!(vec![8], components[2]);
}

#[test]
fn test_average_distance() {
    let edges = vec![(0,1),(0,2),(0,3),(0,4),(0,5),(5,6)];
    let graph = Graph::create_undirected(8,&edges);
    //0 is 0 away from itself, 1 away from 1,2,3,4,and 5, and 2 away from 6
    //this should result in (0 + 5 + 2) / 7 which is 1.0
    assert_eq!(Graph::compute_avg_distance_bfs(0,&graph), 1.0);
}

fn main() -> io::Result<()> {
//reads the data text file
let file_path = Path::new("data.txt");
let file_content = File::open(&file_path)?;
let reader = io::BufReader::new(file_content);
let mut edges: Vec<Edge> = Vec::new();
let mut labels: Vec<String> = vec![];
let mut n = 0;
let mut island_mod = 0;
for line in reader.lines() {
    let line = line?;
    let components: Vec<String> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    if components.len() == 1{
        n = components[0].parse::<usize>().unwrap()*4;
    } else if components.len() > 10{
        for comp in components{
            //each label gets added 4 times, one for each possible island
            labels.push("gg-".to_owned() + &comp);
            labels.push("cb-".to_owned() + &comp);
            labels.push("th-".to_owned() + &comp);
        }
    } else {
    //this variable allows for proper indexing while creating the edge list
    if components[0] == "gg"{
        island_mod = 0;
    } else if components[0] == "cb"{
        island_mod = 1;
    } else if components[0] == "th"{
        island_mod = 2;
    }
    for i in 1..components.len(){
        for j in 1..i{
            edges.push((((components[i].parse::<usize>().unwrap())*3)+island_mod,(((components[j].parse::<usize>().unwrap())*3)+island_mod)));
        }
    }
}
}
    let graph = Graph::create_undirected(n, &edges);
    let components = Graph::compute_components(&graph);
    Graph::print_components_reps(components, &graph, &labels);
    Ok(())
}