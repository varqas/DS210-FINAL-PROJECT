// find length of each list of connections from bfs

use std::collections::HashMap;
use crate::bfs::{distances_bfs, collect_distances};


pub fn degree_of_separation(graph: Vec<Vec<usize>>, node_count: usize, threshold: usize) -> HashMap<usize, Vec<usize>> {
    let mut distance_map: HashMap<usize,usize> = collect_distances(graph, node_count);
    let mut shortest_distances: HashMap<usize,usize> = HashMap::new();
    for (node, distance) in distance_map.iter() {
        if *distance <= threshold {
            shortest_distances[node].push(distance);
        }
    }
    shortest_distances;
}   



pub fn average_number_citations(graph: &ListOfEdges, degree: usize) -> f64 {
    let mut citations_per_node: Hashmap<usize, usize> = HashMap::new();
    //for (node, )



}
// find number of citations per node at each degree
// this node has three direct citations but 12 at 2

// take avg over all nodes

// find length of vector for shortest_distances
//do for loop of different thresholds