use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

mod bfs;
use crate::bfs::{}
mod pagerank;
use crate::pagerank::{}
mod degrees;

fn main() {
    //let mut 
    let data = read_file("Cit-HepPh.txt");
    let mut bfs_graph = bfs::distance_bfs(usize, &data);
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

fn graph_to_hashmap(node_count: usize, edge_list: Vec<(usize, usize)>) {
    let (node_count, edge_list) = read_file("Cit-HepPh.txt");
    let mut graph_map = HashMap::new();
    for &(x,y) in &(edge_list) {
        graph_map.insert((x,y));
    }
    return graph_map
}


fn adj_list(my_vec: &Vec<(usize,usize)>, node_count: usize) -> Vec<Vec<usize>> {
    let mut adjacency_list: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    for (i, j) in my_vec.iter() {
        adjacency_list[*i as usize].push(*j);
    }
    return adjacency_list;
}

