use std::fs::File;
//use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

mod bfs;
use bfs::distances_bfs;

mod degrees;
use degrees::{degree_of_separation, average_number_citations};



fn main() {
    let data = read_file("Cit-HepPh.txt");
    let num_nodes = data.0;
    let list_edges = data.1;
    let adjacency_list = adj_list(&list_edges, num_nodes);
    let mut bfs_graph = distances_bfs(0, &adjacency_list, num_nodes);
    let threshold = 5;
    for threshold in 1..=5 {
        let mut degree_result = degree_of_separation(adjacency_list, node_count, threshold);
        println!("{:?}", degree_result);
        let mut average_citations = average_number_citations(&adjacency_list);
        //println!("{:?}", )
    }
    let mut shortest_distance_pairs: HashMap<usize,usize> = degree_of_separation(bfs_graph, node_count, threshold);
}



fn read_file(path: &str) -> (usize, Vec<(usize, usize)>) {
    let mut node_count = 0;
    let mut edge_list: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file);
    for (index, line) in buf_reader.lines().enumerate() {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        if index == 0 {
            node_count = v[0].parse::<usize>().unwrap();
        }else{
            let x = v[0].parse::<usize>().unwrap();
            let y = v[1].parse::<usize>().unwrap();
            edge_list.push((x, y));
        }
    }
    return (node_count, edge_list);
}



fn adj_list(my_vec: &Vec<(usize,usize)>, node_count: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (i, j) in my_vec.iter() {
        adjacency_list[*i as usize].push(*j);
    }
    return adjacency_list;
}

// test the important functions