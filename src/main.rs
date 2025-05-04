// 1 sentence abt what it does
mod graph;
// 1 sentence abt what it does
mod centrality;

use graph::load_graph;
use centrality::{
    compute_degree_centrality,
    compute_closeness_centrality,
    compute_betweenness_centrality,
};

fn main() {
    let graph = load_graph("facebook_combined.txt");

    // DEGREE CENTRALITY
    let degree = compute_degree_centrality(&graph);
    let mut degree_sorted: Vec<_> = degree.iter().collect();
    degree_sorted.sort_by_key(|&(_, deg)| std::cmp::Reverse(*deg));
    println!("\nTop 5 degree centrality:");
    for (node, deg) in degree_sorted.iter().take(5) {
        println!("Node {}: degree = {}", node, deg);
    }

    // CLOSENESS CENTRALITY
    println!("\nCalculating closeness centrality...");
    let closeness = compute_closeness_centrality(&graph);
    let mut closeness_sorted: Vec<_> = closeness.iter().collect();
    closeness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Top 5 closeness centrality:");
    for (node, score) in closeness_sorted.iter().take(5) {
        println!("Node {}: closeness = {:.6}", node, score);
    }

    // BETWEENNESS CENTRALITY
    println!("\nCalculating betweenness centrality (this may take a minute)...");
    let betweenness = compute_betweenness_centrality(&graph);
    let mut betweenness_sorted: Vec<_> = betweenness.iter().collect();
    betweenness_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Top 5 betweenness centrality:");
    for (node, score) in betweenness_sorted.iter().take(5) {
        println!("Node {}: betweenness = {:.6}", node, score);
    }
}
