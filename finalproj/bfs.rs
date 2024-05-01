// Implementation of the BFS algorithm to find the distances between each of the vertices

use std::collections::HashMap;
use std::collections::VecDeque;

type Vertex = usize;
type ListOfEdges = Vec<Vec<Vertex>>;


pub fn distance_bfs(initial_vertex: Vertex, graph: &graph_map) -> HashMap<(usize, usize)> {
    let initial_vertex = 0;
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distances[initial_vertex] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(initial_vertex);
    while let Some(v) = queue.pop_front() { 
        for u in graph.outedges[v].iter() {
            if let None = distances[*u] {
                distances[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    print!("vertex:distance");
    for v in 0..graph.n {
        print!("{}:{}", v, distances[v].unwrap());
    }
    println!();
}

// Output of BFS for a given node is the distances to the other nodes. From there, take the nodes that are less than x distance away.
// Deg sep.- For loop that sees if the distance is less than or equal to x. Make a hash map to collect the values that are less than or equal to.

pub fn collect_distances(graph: &data) {
    let mut distance_map = HashMap::new();
    for i in 0..graph.n {
        println!("Distances from node {}", i);
        let distances = bfs(i, &graph);
        distance_map.insert(i, distance[i]);
    }
}
