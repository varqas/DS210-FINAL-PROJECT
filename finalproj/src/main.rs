// Collaborators: None. ChatGPT used for debugging.
// NOTE: As not to skew line count, I made small explanations for the functions in here, but each function get a far more detailed explanation in the report.

// Importing necessary libraries and structures. 

use std::fs::File;
use std::io::{BufRead, BufReader};

mod bfs;
use bfs::distances_bfs;

mod degrees;
use degrees::{degree_of_separation, average_number_citations};

// The main function of the program. This function creates the adjacency list from the graph's txt file and then uses it to collect the distances across the nodes and compute the average distance at different thresholds.
// For error handling purposes and based on read_file, the main function uses an Ok/Err enum in the case that it cannot read the provided file. I got the idea from ChatGPT and have explained/cited accordingly.

pub fn main() {
    let data = read_file(r#"C:\Users\smart\Downloads\DS210RUST\DS210-FINAL-PROJECT\finalproj\Cit-HepPh.txt"#);
    match data {
        Ok((num_nodes, list_edges)) => {
            let adjacency_list = adj_list(&list_edges, num_nodes);
            let num_nodes = adjacency_list.len(); 
            let bfs_graph: Vec<Option<u32>> = distances_bfs( 0,&adjacency_list, num_nodes);
            println!("Shortest distances of each vertex to 0: {:?}", bfs_graph);
            let degree = 2;
            let _threshold: i32 = 5;
            for threshold in 1..=5 {
                let threshold_option = Some(threshold as u32);
                let degree_result = degree_of_separation(adjacency_list.clone(), num_nodes, threshold_option);
                println!("At a threshold of {}, the shortest distance pairs are: {:?}", threshold, degree_result);
                let average_citations = average_number_citations(&adjacency_list, degree);
                println!("The average number of citations across all nodes is {:?}", average_citations);
            }
        }
        Err(err) => {
    
            eprintln!("Error reading file: {}", err);
            return;
        }
    }
}

// The function that reads the graph file and converts it into a vector for processing and analysis. 
// Like the main function, this function also uses Ok/Err and removes whitespace to handle errors effectively when reading over text files.

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

// Creating the adjacency list for the function. 

pub fn adj_list(my_vec: &Vec<(usize, usize)>, node_count: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); node_count];

    for &(i, j) in my_vec.iter() {
        if i < node_count && j < node_count {
            while adjacency_list.len() <= i || adjacency_list.len() <= j {
                adjacency_list.push(Vec::new());
            }
            adjacency_list[i].push(j);
        } else {
            eprintln!("Warning: Edge ({}, {}) is out of bounds", i, j);
        }
    }

    adjacency_list
}

// Testing the BFS function with a small sample data set.

#[test]
fn test_bfs() {
    let test_graph = vec![
            vec![1, 2],
            vec![0, 3],
            vec![0, 4],
            vec![1],
            vec![2],
        ];

        let initial_vertex = 0;
        let distances = distances_bfs(initial_vertex, &test_graph, test_graph.len());

        assert_eq!(distances.len(), test_graph.len());  

        assert_eq!(distances[0], Some(0));
        assert_eq!(distances[1], Some(1));
        assert_eq!(distances[2], Some(1));
        assert_eq!(distances[3], Some(2));
        assert_eq!(distances[4], Some(2));
}

#[test]

// Testing the degree of separation function on a smaller data set.

fn test_degree_of_separation() {
    let test_graph = vec![
        vec![1, 2],
        vec![0, 3],
        vec![0, 4],
        vec![1],
        vec![2],
    ];

    let node_count = test_graph.len();
    let thresholds = vec![Some(1), Some(2), Some(3)];

    for threshold in thresholds {
        let result = degree_of_separation(test_graph.clone(), node_count, threshold);
        match threshold {
            Some(t) => {
                for node in 0..node_count {
                    for &dist in &result[&node] {
                        if let Some(d) = dist {
                            assert!(d <= t, "Distance {} for node {} exceeds threshold {}", d, node, t);
                        }
                    }
                }
            }
            None => {
                for node in 0..node_count {
                    assert_eq!(result[&node], vec![Some(0)], "Incorrect distance for node {} with threshold None", node);
                }
            }
        }
    }
}