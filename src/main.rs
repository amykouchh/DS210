// This module reads and loads in the graph using data from the txt.file, splits the data into the Node IDs
// and a list of its neighbors to represent an undirected graph--turned into an adjacency list
mod graph;
// This module computes the three main centrality scores--degree, closeness, betweenness for each of the nodes in the graph
mod centrality;

use graph::load_graph;
use centrality::{
    degree_centrality,
    closeness_centrality,
    betweenness_centrality,
};

fn main() {
    let graph = load_graph("facebook_combined.txt");

    // Using degree centrality, the nodes are sorted and the top five are printed
    let degree = degree_centrality(&graph);
    let mut degree_sorted: Vec<_> = degree.iter().collect();
    degree_sorted.sort_by_key(|&(_, deg)| std::cmp::Reverse(*deg));
    println!("\nTop 5 degree centrality:");
    for (node, deg) in degree_sorted.iter().take(5) {
        println!("Node {}: degree = {}", node, deg);
    }

    // This computes the closeness centrality for all nodes, then sorted into 
    // descending order and printed out (the top 5 closeness centrality valies)
    let closeness = closeness_centrality(&graph);
    let mut closeness_sorted: Vec<_> = closeness.iter().collect();
    closeness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("\nTop 5 closeness centrality:");
    for (node, score) in closeness_sorted.iter().take(5) {
        println!("Node {}: closeness = {:.6}", node, score);
    }

    // This computes the betweenness centrality for all of the nodes, sorts them in descending 
    // order and prints the top five most connected nodes in the dataset
    let betweenness = betweenness_centrality(&graph);
    let mut betweenness_sorted: Vec<_> = betweenness.iter().collect();
    betweenness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("\nTop 5 betweenness centrality:");
    for (node, score) in betweenness_sorted.iter().take(5) {
        println!("Node {}: betweenness = {:.6}", node, score);
    }
}
