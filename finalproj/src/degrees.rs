// Collaborators: None. ChatGPT used for debugging.

// Importing the necessary functions and libraries.

use std::collections::HashMap;
use crate::bfs::{collect_distances, AdjacencyList};

// Collecting the shortest distances of each node based on a certain degree of separation.
// I used ChatGPT for much of the debugging and error handling, especially in the case of None types.

pub fn degree_of_separation(graph: Vec<Vec<usize>>, node_count: usize, threshold: Option<u32>) -> HashMap<usize, Vec<Option<u32>>> {
    let initial_vertex = 0;
    let distance_map: HashMap<usize, Option<u32>> = collect_distances(initial_vertex, graph.clone(), node_count);
    let mut shortest_distances: HashMap<usize, Vec<Option<u32>>> = HashMap::new();
    for node in 0..node_count {
        shortest_distances.insert(node, Vec::new());
    }
    for (node, distance) in distance_map.iter() {
        if let Some(dist) = distance {
            if let Some(threshold_value) = threshold {
                if *dist <= threshold_value {
                    shortest_distances.get_mut(node).unwrap().push(Some(*dist));
                }
            }
        }
    }

    shortest_distances
}

// Calculating the average number of citations, taking into account direct and indirect neighbors.
// I also used GPT here because I was initially unsure of how to iterate through the graph and collect every value, but now I understand and have cited + explained accordingly in the report.

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