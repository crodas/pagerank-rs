//! # Simple Pagerank
//!
//! Pretty simple generic implementation of the PageRank graph sorting algorithm.
#![deny(missing_docs)]
#![allow(warnings)]
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

#[derive(Clone)]
struct Node<T>
where
    T: Eq + Hash + Clone,
{
    /// Edge type
    node: T,
    /// List of edges (the ids which are edges in `nodes`)
    in_edges: Vec<usize>,
    /// Number of out edges
    out_edges: usize,
    score: f64,
}

/// PageRank structure.
///
pub struct Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    /// Damping factor
    ///
    /// The PageRank theory holds that an imaginary surfer who is randomly clicking on edges will
    /// eventually stop clicking. The probability, at any step, that the person will continue is a
    /// damping factor d. Various studies have tested different damping factors, but it is generally
    /// assumed that the damping factor will be set around 0.85.
    damping: f64,
    /// List of nodes. Each node is uniquely identified by their type T.
    nodes: Vec<Node<T>>,
    /// Total number of elements
    edges: usize,
    /// Keeps track of nodes and their position in the nodes vector.
    node_positions: HashMap<T, usize>,
    /// Cache to keep the count of total nodes with incoming edges. This cache gets reset everytime
    /// a new node is being added to the graph.
    nodes_with_in_edges: Option<usize>,
}

impl<T> Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates a new instance
    pub fn new() -> Pagerank<T> {
        Pagerank::<T> {
            damping: 0.85,
            nodes: Vec::new(),
            edges: 0,
            node_positions: HashMap::<T, usize>::new(),
            nodes_with_in_edges: None,
        }
    }

    /// Sets the dumping factor. A value between 0 and 100 is expected.
    pub fn set_damping_factor(
        &mut self,
        factor: u8,
    ) -> Result<(), String> {
        if factor >= 100 {
            return Err("{val} needs to be bellow 100".to_string());
        }

        self.damping = factor as f64 / 100_f64;
        Ok(())
    }

    /// Adds an node between two nodes
    pub fn add_edge(&mut self, source: T, target: T) {
        let source = self.get_or_create_node(source);
        let target = self.get_or_create_node(target);
        self.nodes[source].out_edges += 1;
        self.nodes[target].in_edges.push(source);
        self.edges += 1;
    }

    /// Returns the current score of a gien node
    pub fn get_score(&self, node: T) -> Option<f64> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].score)
    }

    /// Returns the number of in edges for the given node
    pub fn get_in_edges(&self, node: T) -> Option<usize> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].in_edges.len())
    }

    /// Returns the number of out edges for the given node
    pub fn get_out_edges(&self, node: T) -> Option<usize> {
        self.node_positions
            .get(&node)
            .map(|id| self.nodes[*id].out_edges)
    }

    /// Returns the node_id for a given node name
    pub fn get_or_create_node(&mut self, node: T) -> usize {
        match self.node_positions.get(&node) {
            Some(&value) => value,
            _ => {
                let id = self.nodes.len();
                self.nodes.push(Node::<T> {
                    node: node.clone(),
                    in_edges: Vec::new(),
                    out_edges: 0,
                    score: 1f64 - self.damping,
                });
                self.node_positions.insert(node, id);
                self.nodes_with_in_edges = None;
                id
            }
        }
    }

    /// Calculates PageRank with custom convergence
    pub fn calculate_with_convergence(
        &mut self,
        convergence: f64,
    ) -> i32 {
        let mut iterations = 0;

        loop {
            if self.calculate_step() < convergence {
                break;
            }
            iterations += 1;
        }

        iterations
    }

    /// Calculates pagerank with custom convergence
    pub fn calculate(&mut self) -> i32 {
        self.calculate_with_convergence(0.01)
    }

    /// Return all nodes, sorted by their pagerank
    pub fn nodes(&self) -> Vec<(&T, f64)> {
        let mut nodes = self
            .nodes
            .iter()
            .map(|node| (&node.node, node.score))
            .collect::<Vec<(&T, f64)>>();

        nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        nodes
    }

    /// Calculates a single iteration of the PageRank
    pub fn calculate_step(&mut self) -> f64 {
        let mut current_iteration = self.nodes.clone();

        let nodes = &self.nodes;

        self.nodes
            .iter()
            .enumerate()
            .map(|(id, n)| {
                let score = n
                    .in_edges
                    .iter()
                    .map(|node| {
                        nodes[*node].score
                            / nodes[*node].out_edges as f64
                    })
                    .sum::<f64>();

                current_iteration[id].score =
                    (1f64 - self.damping) + (self.damping * score);
            })
            .for_each(drop);

        let convergence: f64 = self
            .nodes
            .iter()
            .enumerate()
            .map(|(id, n)| {
                let diff = n.score - current_iteration[id].score;
                diff * diff
            })
            .sum();

        self.nodes = current_iteration;

        convergence.sqrt() / self.len_nodes_with_in_edges() as f64
    }

    /// Len of all edges
    pub fn len_nodes_with_in_edges(&mut self) -> usize {
        if let Some(n) = self.nodes_with_in_edges {
            return n;
        }

        let mut total = 0;

        for node in self.nodes.iter() {
            if node.in_edges.len() > 0 {
                total += 1;
            }
        }

        self.nodes_with_in_edges = Some(total);

        total
    }

    /// Return the number of vertices/nodes in the current graph
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the current graph
    pub fn len_node(&self) -> usize {
        self.edges
    }

    /// If the graph is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

impl<T> Default for Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Pagerank;

    #[test]
    fn test_two_nodes_are_created() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        assert_eq!(2, pr.len())
    }

    #[test]
    fn test_edges() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        assert_eq!(0, pr.get_or_create_node("foo"));
        assert_eq!(1, pr.get_or_create_node("bar"));

        assert_eq!(Some(0), pr.get_in_edges("foo"));
        assert_eq!(Some(1), pr.get_out_edges("foo"));
        assert_eq!(Some(1), pr.get_in_edges("bar"));
        assert_eq!(Some(0), pr.get_out_edges("bar"));
    }

    #[test]
    fn test_default_score() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        assert_eq!(
            15_i64,
            (pr.get_score("foo").expect("float") * 100_f64) as i64
        );
        assert_eq!(pr.get_score("foo"), pr.get_score("bar"));
        assert_eq!(pr.get_score("foo"), pr.get_score("xxx"));
        assert_eq!(pr.get_score("foo"), pr.get_score("yyy"));
    }

    #[test]
    fn test_iteration() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        pr.calculate_step();

        assert_eq!(
            vec!["bar", "foo", "xxx", "yyy"],
            pr.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_iterations() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        assert_eq!(true, pr.calculate_step() > pr.calculate_step());
        pr.calculate_step();

        assert_eq!(
            vec!["bar", "foo", "xxx", "yyy"],
            pr.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_full_run() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        assert_eq!(16, pr.calculate());

        assert_eq!(
            vec!["bar", "foo", "xxx", "yyy"],
            pr.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    /// https://en.wikipedia.org/wiki/PageRank#/media/File:PageRanks-Example.svg
    fn test_pagerank_example() {
        let mut pr = Pagerank::new();
        let edges = vec![
            ("D", "A"),
            ("D", "B"),
            ("B", "C"),
            ("C", "B"),
            ("E", "B"),
            ("E", "F"),
            ("F", "B"),
            ("F", "E"),
            ("G", "B"),
            ("G", "E"),
            ("H", "B"),
            ("H", "E"),
            ("I", "B"),
            ("I", "E"),
            ("J", "E"),
            ("K", "E"),
        ];

        edges
            .iter()
            .map(|(l1, l2)| pr.add_edge(*l1, *l2))
            .for_each(drop);

        pr.calculate();

        assert_eq!(
            vec![
                "B", "C", "E", "F", "A", "D", "G", "H", "I", "J", "K"
            ],
            pr.nodes()
                .iter()
                .map(|(node, _)| **node)
                .collect::<Vec<&str>>()
        );
    }
}
