// find length of each list of connections from bfs

use std::collections::HashMap;
use crate::bfs::{collect_distances, AdjacencyList};


//used GPT
pub fn degree_of_separation(graph: Vec<Vec<usize>>, node_count: usize, threshold: Option<u32>) -> HashMap<usize, Vec<Option<u32>>> {
    let initial_vertex = 0;
    let distance_map: HashMap<usize, Option<u32>> = collect_distances(initial_vertex, graph.clone(), node_count);
    //println!("Distance Map: {:?}", distance_map); // Debugging print statement
    let mut shortest_distances: HashMap<usize, Vec<Option<u32>>> = HashMap::new();
    for node in 0..node_count {
        shortest_distances.insert(node, Vec::new());
    }
    println!("Threshold: {:?}", threshold); // Debugging print statement
    for (node, distance) in distance_map.iter() {
        if let Some(dist) = distance {
            if let Some(threshold_value) = threshold {
                if *dist <= threshold_value {
                    println!("Adding distance {} for node {}", dist, node); // Debugging print statement
                    shortest_distances.get_mut(node).unwrap().push(Some(*dist));
                }
            }
        }
    }
    println!("Shortest Distances: {:?}", shortest_distances); // Debugging print statement
    shortest_distances
}



pub fn average_number_citations(graph: &AdjacencyList, degree: usize) -> f64 {
    let mut citations_per_node: HashMap<usize, usize> = HashMap::new();
    for node_edges in graph.iter() {
        let count = node_edges.iter().filter(|&&d| d == degree).count();
        citations_per_node.insert(node_edges.len(), count);
    }
    let total_nodes = graph.len();
    let total_citations: usize = citations_per_node.values().sum();
    let average_citations = total_citations as f64 / total_nodes as f64;
    average_citations
}
// find number of citations per node at each degree
// this node has three direct citations but 12 at 2

// take avg over all nodes

// find length of vector for shortest_distances
//do for loop of different thresholds