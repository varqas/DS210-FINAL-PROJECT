// Implementation of the BFS algorithm to find the distances between each of the vertices

use std::collections::HashMap;
use std::collections::VecDeque;

pub type Vertex = usize;
pub type AdjacencyList = Vec<Vec<usize>>;



pub fn distances_bfs(initial_vertex: Vertex, graph: &AdjacencyList, node_count: usize) -> Vec<Option<u32>> {
    let mut distances: Vec<Option<u32>> = vec![None; node_count];
    distances[initial_vertex] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(initial_vertex);
    
    while let Some(current_vertex) = queue.pop_front() {
        println!("Visiting vertex {}", current_vertex);
        for &neighbor in &graph[current_vertex] {
            if distances[neighbor].is_none() {
                if let Some(distance_to_current) = distances[current_vertex] {
                    let new_distance = distance_to_current + 1;
                    println!("Updating distance to vertex {}: {}", neighbor, new_distance);
                    distances[neighbor] = Some(new_distance);
                } else {
                    eprintln!("Error: Distance to current vertex is None.");
                    continue;
                }
                queue.push_back(neighbor);
            }
        }
    }
    
    distances
}



// Output of BFS for a given node is the distances to the other nodes. From there, take the nodes that are less than x distance away.
// Deg sep.- For loop that sees if the distance is less than or equal to x. Make a hash map to collect the values that are less than or equal to.

pub fn collect_distances(initial_vertex: Vertex, graph: Vec<Vec<usize>>, node_count: usize) -> HashMap<usize, Option<u32>> {
    let distances = distances_bfs(initial_vertex, &graph, node_count);
    distances.into_iter().enumerate().collect()
}