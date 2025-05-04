// src/graph.rs

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub type Graph = HashMap<usize, Vec<usize>>;

pub fn load_graph(path: &str) -> Graph {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in reader.lines() {
        if let Ok(edge) = line {
            let parts: Vec<usize> = edge
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect();
            if parts.len() == 2 {
                let (u, v) = (parts[0], parts[1]);
                graph.entry(u).or_default().push(v);
                graph.entry(v).or_default().push(u);
            }
        }
    }

    graph
}
