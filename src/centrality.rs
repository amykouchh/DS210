use std::collections::{HashMap, HashSet, VecDeque};
use super::graph::Graph;


pub fn degree_centrality(graph: &Graph) -> HashMap<usize, usize> {
    let mut degrees = HashMap::new();
    for (node, neighbors) in graph {
        degrees.insert(*node, neighbors.len());
    }
    degrees
}


fn bfs_shortest_paths(graph: &Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    visited.insert(start);
    queue.push_back(start);


    while let Some(node) = queue.pop_front() {
        let current_distance = distances[&node];
        // explain for loop
        for &neighbor in graph.get(&node).unwrap_or(&Vec::new()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}


pub fn closeness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut closeness = HashMap::new();

    for &node in graph.keys() {
        let distances = bfs_shortest_paths(graph, node);
        let total_distance: usize = distances.values().sum();
        if total_distance > 0 {
            closeness.insert(node, 1.0 / total_distance as f64);
        } else {
            closeness.insert(node, 0.0);
        }
    }

    closeness
}

pub fn betweenness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut betweenness = HashMap::new();

    for &s in graph.keys() {
        let mut stack = Vec::new();
        let mut pred: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut sigma = HashMap::new();
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();

        for &v in graph.keys() {
            pred.insert(v, Vec::new());
            sigma.insert(v, 0);
            dist.insert(v, -1);
        }

        sigma.insert(s, 1);
        dist.insert(s, 0);
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in graph.get(&v).unwrap_or(&Vec::new()) {
                if dist[&w] < 0 {
                    dist.insert(w, dist[&v] + 1);
                    queue.push_back(w);
                }
                if dist[&w] == dist[&v] + 1 {
                    sigma.insert(w, sigma[&w] + sigma[&v]);
                    pred.get_mut(&w).unwrap().push(v);
                }
            }
        }

        let mut delta = HashMap::new();
        for &v in graph.keys() {
            delta.insert(v, 0.0);
        }

        while let Some(w) = stack.pop() {
            for &v in &pred[&w] {
                let contrib = (sigma[&v] as f64 / sigma[&w] as f64) * (1.0 + delta[&w]);
                *delta.get_mut(&v).unwrap() += contrib;
            }
            if w != s {
                *betweenness.entry(w).or_insert(0.0) += delta[&w];
            }
        }
    }

    for val in betweenness.values_mut() {
        *val /= 2.0;
    }

    betweenness
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    fn sample_graph() -> Graph {
        
        let mut graph: Graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 3]);
        graph.insert(3, vec![1, 2]);
        graph
    }

    #[test]
 
    fn test_degree_centrality() {
        let graph = sample_graph();
        let degrees = degree_centrality(&graph);
        assert_eq!(degrees.get(&1), Some(&2));
        assert_eq!(degrees.get(&2), Some(&2));
        assert_eq!(degrees.get(&3), Some(&2));
    }

    #[test]
    fn test_closeness_centrality() {
        let graph = sample_graph();
        let closeness = closeness_centrality(&graph);
        let value = closeness.get(&1).unwrap();
        assert!(*value > 0.0 && *value < 1.0); // just check it calculated
    }

    #[test]
    fn test_betweenness_centrality() {
        let graph = sample_graph();
        let betweenness = betweenness_centrality(&graph);
        for (_, score) in betweenness {
            assert!(score >= 0.0); // basic sanity check
        }
    }

}
