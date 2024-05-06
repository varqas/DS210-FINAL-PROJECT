// Collaborators: None. ChatGPT used for debugging.

// Importing the necessary libraries and structures.

use std::collections::HashMap;
use std::collections::VecDeque;

pub type Vertex = usize;
pub type AdjacencyList = Vec<Vec<usize>>;

// Calculating the distances between each node from an initial vertex.
// I had to use ChatGPT to debug types and track the vertices.

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

// Collecting all of the vertex distances into a hash map.

pub fn collect_distances(initial_vertex: Vertex, graph: Vec<Vec<usize>>, node_count: usize) -> HashMap<usize, Option<u32>> {
    let distances = distances_bfs(initial_vertex, &graph, node_count);
    distances.into_iter().enumerate().collect()
}