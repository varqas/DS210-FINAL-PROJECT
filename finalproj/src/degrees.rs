// find length of each list of connections from bfs

use std::collections::HashMap;

mod bfs;
use crate::distances_bfs::{};

pub fn degree_of_separation(distance_map: &HashMap<usize, usize>, threshold: usize) -> HashMap<usize,usize> {
    let mut distance_map: HashMap<usize,usize> = collect_distances(&data);
    let mut shortest_distances: HashMap<usize,usize> = HashMap::new();
    for (x,y) in distance_map {
        if y <= threshold {
            shortest_distances.insert(x, y);
        }
    }
    return shortest_distances;
}   