// Implementation of the BFS algorithm to find the distances between each of the vertices

use std::collections::HashMap;
use std::collections::VecDeque;

type Vertex = usize;
type ListOfEdges = Vec<Vec<Vertex>>;


pub fn distances_bfs(initial_vertex: Vertex, graph: &ListOfEdges, node_count: usize) -> Vec<Option<u32>> {
    let mut distance: Vec<Option<u32>> = vec![None;node_count];
    distance[initial_vertex] = Some(0); 
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(initial_vertex);
    while let Some(v) = queue.pop_front() { 
        for u in graph[v].iter() {
            if let None = distance[*u] {
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    print!("vertex:distance");
    for v in 0..node_count {
        print!("{}:{}", v, distance[v].unwrap());
    }
    return distance;
}

// Output of BFS for a given node is the distances to the other nodes. From there, take the nodes that are less than x distance away.
// Deg sep.- For loop that sees if the distance is less than or equal to x. Make a hash map to collect the values that are less than or equal to.

pub fn collect_distances(initial_vertex: Vertex, graph: Vec<Vec<usize>>, node_count: usize) -> HashMap<usize, Option<u32>>{
    let mut distance_map = HashMap::new();
    for i in 0..node_count {
        println!("Distances from node {}", i);
        let distances = distances_bfs(i, &graph, node_count);
        distance_map.insert(i, distances[i]);
    }
    return distance_map;
}
