pub mod sleep_graph {
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Graph {
        pub n: usize,
        pub outedges: Vec<Vec<usize>>,
    }
    type Vertex = usize;
    type Edge = (Vertex, Vertex);
    type Component = usize;
    use std::collections::VecDeque;
    impl Graph {
        pub fn create_undirected(n:usize,edges:&Vec<Edge>) -> Graph {
            let mut outedges = vec![vec![];n];
            for (u, v) in edges {
                outedges[*u].push(*v);
                outedges[*v].push(*u)
            }
            Graph{n,outedges}
        }
        pub fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
            component[vertex] = Some(component_no);
            
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(vertex);
            
            while let Some(v) = queue.pop_front() {
                for w in graph.outedges[v].iter() {
                    if let None = component[*w] {
                        component[*w] = Some(component_no);
                        queue.push_back(*w);
                    }
                }
            }
        }
        pub fn compute_components(graph:&Graph) -> Vec<Vec<usize>>{
            let mut component: Vec<Option<Component>> = vec![None;graph.n];
            let mut component_count = 0;
            for v in 0..graph.n {
                if let None = component[v] {
                   component_count += 1;
                   Graph::mark_component_bfs(v, &graph, &mut component, component_count);
                }
            };
            //collects components into a vector that contains all the vertices in component x at component_results[x]
            let mut component_results: Vec<Vec<Vertex>> = vec![vec![];component_count];
            for comp in 1..=component_count{
                for v in 0..graph.n {
                    if component[v].unwrap() == comp{
                        component_results[comp-1].push(v);
                    }
                }
            }
            return component_results
        }
        pub fn compute_avg_distance_bfs(start: Vertex, graph: &Graph) -> f64{
            //computes distance to every other vertex and collects into vector
            let mut distance: Vec<Option<u32>> = vec![None;graph.n];
            distance[start] = Some(0);
            let mut queue: VecDeque<Vertex> = VecDeque::new();
            queue.push_back(start);
            while let Some(v) = queue.pop_front() {
                for u in graph.outedges[v].iter() {
                    if let None = distance[*u] {
                        distance[*u] = Some(distance[v].unwrap() + 1);
                        queue.push_back(*u);
                    }
                }
            }
            //sums distance of all vertices that can be reached
            let mut sum = 0.0;
            let mut count = 0.0;
            for v in 0..graph.n {
                if let None = distance[v]{

                } else {
                    sum += distance[v].unwrap() as f64;
                    count += 1.0;
                }
            }
            //returns average of those distances
            return sum / count
        }
        pub fn print_components_reps(components: Vec<Vec<usize>>, graph:&Graph, labels:&Vec<String>){
            let mut index = 1;
            print!("{} components total\n",components.len());
            print!("non-singular components:\n");
            for component in components{
                if component.len() != 1{
                    print!("{}: ",index);
                    index += 1;
                    //goes through each vertex in the component and gets the one with the minimum distance to all other vertices in the component
                    let mut min_dist = 1000.0;
                    let mut min_vertex: String = "".to_string();
                    for vertex in component{
                        print!("{} ",labels[vertex]);
                        let dist = Graph::compute_avg_distance_bfs(vertex,&graph);
                        if dist < min_dist{
                            min_dist = dist;
                            min_vertex = labels[vertex].to_owned();
                        } else if dist == min_dist{
                            min_vertex = min_vertex + ", " + &labels[vertex];
                        }
                }
                print!("\n");
                println!("Representative Style: {}",min_vertex);
                }
            }
        }
    }
}