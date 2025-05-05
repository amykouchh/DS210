use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// Type alias for undirected graph
pub type Graph = HashMap<usize, Vec<usize>>;

// Reads in data from the text file and constructs an undirected graph
// It gets parsed into two parts--Node IDs and neighbor--to represent an edge
// It takes in a string reference as an input and outputs a Graph
pub fn load_graph(path: &str) -> Graph {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    // This loop iterates through each line and reads it
    for line in reader.lines() {
        // The if statement cleans the lines checks if the number is valid, 
        // before collecting it onto Vec<usize>
        if let Ok(edge) = line {
            let sect: Vec<usize> = edge
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number"))
                .collect();
            // This if statement checks if the text file can be sorted into exactly two parts
            // If so, the edges are added in both directions
            // This ensures they are undirected edges
            if sect.len() == 2 {
                let (u, v) = (sect[0], sect[1]);
                graph.entry(u).or_default().push(v);
                graph.entry(v).or_default().push(u);
            }
        }
    }

    graph
}
