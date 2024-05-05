use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

mod bfs;
use bfs::{distances_bfs, collect_distances};

mod degrees;
use degrees::{degree_of_separation, average_number_citations};



pub fn main() {
    let data = read_file(r#"C:\Users\smart\Downloads\DS210RUST\DS210-FINAL-PROJECT\finalproj\Cit-HepPh.txt"#);
    match data {
        Ok((num_nodes, list_edges)) => {
            let adjacency_list = adj_list(&list_edges, num_nodes);
            let num_nodes = adjacency_list.len(); // Using the length of the adjacency list as the number of nodes
            let bfs_graph: Vec<Option<u32>> = distances_bfs(0, &adjacency_list, num_nodes);
            let distance_map = collect_distances(0, adjacency_list.clone(), num_nodes);
            //println!("Shortest distances of each vertex to 0: {:?}", bfs_graph);
            let degree = 2;
            let _threshold: i32 = 5;
            for threshold in 1..=5 {
                let threshold_option = Some(threshold as u32);
                let degree_result = degree_of_separation(adjacency_list.clone(), num_nodes, threshold_option);
                //println!("At a threshold of {}, the shortest distance pairs are: {:?}", threshold, degree_result);
                let average_citations = average_number_citations(&adjacency_list, degree);
                println!("The average number of citations across all nodes is {:?}", average_citations);
            }
        }
        Err(err) => {
            // Error occurred while reading the file, handle the error
            eprintln!("Error reading file: {}", err);
            // Optionally, exit the program or handle the error gracefully
            return;
        }
    }
}



pub fn read_file(path: &str) -> Result<(usize, Vec<(usize, usize)>), Box<dyn std::error::Error>> {
    let mut node_count = 0;
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line_str = line?;
        let v: Vec<usize> = line_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap()) // Parse each vertex index as usize
            .collect();
        
        if v.len() >= 2 {
            edge_list.push((v[0], v[1])); // Add the edge to the edge list
        } else {
            eprintln!("Warning: Skipping incomplete line: {}", line_str);
        }
        
        // Determine the maximum vertex index to determine the node count
        for &vertex in &v {
            node_count = node_count.max(vertex + 1);
        }
    }
    
    Ok((node_count, edge_list))
}



pub fn adj_list(my_vec: &Vec<(usize, usize)>, node_count: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); node_count];

    for &(i, j) in my_vec.iter() {
        if i < node_count && j < node_count {
            // Extend the adjacency list if necessary
            while adjacency_list.len() <= i || adjacency_list.len() <= j {
                adjacency_list.push(Vec::new());
            }
            adjacency_list[i].push(j);
        } else {
            eprintln!("Warning: Edge ({}, {}) is out of bounds", i, j);
        }
    }

    // Print the nodes and the adjacency list
    //println!("Number of nodes: {}", node_count);
    //println!("Adjacency list:");
    //for (node, neighbors) in adjacency_list.iter().enumerate() {
        //println!("Node {}: {:?}", node, neighbors);
    //}

    adjacency_list
}



#[test]
fn test_bfs() {
    let test_graph = vec![
            vec![1, 2],  // Vertex 0 is connected to vertices 1 and 2
            vec![0, 3],  // Vertex 1 is connected to vertices 0 and 3
            vec![0, 4],  // Vertex 2 is connected to vertices 0 and 4
            vec![1],     // Vertex 3 is connected to vertex 1
            vec![2],     // Vertex 4 is connected to vertex 2
        ];
        let initial_vertex = 0;
        let distances = distances_bfs(initial_vertex, &test_graph, test_graph.len());

        // Verify the results
        assert_eq!(distances.len(), test_graph.len());  // Ensure distances vector has the correct length

        // Assert specific distances from the start vertex
        assert_eq!(distances[0], Some(0));  // Distance from vertex 0 to itself should be 0
        assert_eq!(distances[1], Some(1));  // Distance from vertex 0 to vertex 1 should be 1
        assert_eq!(distances[2], Some(1));  // Distance from vertex 0 to vertex 2 should be 1
        assert_eq!(distances[3], Some(2));  // Distance from vertex 0 to vertex 3 should be 2
        assert_eq!(distances[4], Some(2));  // Distance from vertex 0 to vertex 4 should be 2
}