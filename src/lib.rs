use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

struct Damping(f64);

impl Damping {
    pub fn new(val: u8) -> Result<Damping, String> {
        if val >= 100 {
            return Err("{val} needs to be bellow 100".to_string());
        }

        Ok(Damping(val as f64 / 100_f64))
    }
}

#[derive(Clone)]
struct Edge(usize, usize);

#[derive(Clone)]
pub struct Node<T>
where
    T: Eq + Hash + Clone,
{
    id: T,
    /// List of links (the ids which are edges in `nodes`)
    incoming_edges: Vec<Edge>,
    /// Number of out links
    outgoing_edges: usize,
    score: f64,
}

impl<T> Node<T>
where
    T: Eq + Hash + Clone,
{
    pub fn id(&self) -> &T {
        &self.id
    }

    pub fn score(&self) -> f64 {
        self.score
    }
}

pub struct Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    damping: Damping,
    nodes: Vec<Node<T>>,
    edges: usize,
    nodes_ids: HashMap<T, usize>,
    nodes_with_inconming_edges: Option<usize>,
    is_calculating: bool,
}

impl<T> Pagerank<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Pagerank<T> {
        Pagerank::<T> {
            damping: Damping::new(15).unwrap(),
            nodes: Vec::new(),
            edges: 0,
            nodes_ids: HashMap::<T, usize>::new(),
            nodes_with_inconming_edges: None,
            is_calculating: false,
        }
    }

    pub fn set_damping_factor(
        &mut self,
        factor: u8,
    ) -> Result<(), String> {
        self.damping = Damping::new(factor)?;
        Ok(())
    }

    /// Adds an edge between two nodes
    pub fn add_edge(&mut self, source: T, target: T) {
        let source = self.get_node_id(source);
        let target = self.get_node_id(target);
        self._add_edge(Edge(source, target))
    }

    /// Private function to add an edge
    fn _add_edge(&mut self, edge: Edge) {
        self.nodes[edge.0].outgoing_edges += 1;
        self.nodes[edge.1].incoming_edges.push(edge);
        self.edges += 1;
    }

    /// Returns a copy of a node
    pub fn get_node(&mut self, name: T) -> Node<T> {
        let id = self.get_node_id(name);

        self.nodes[id].clone()
    }

    /// Returns the node_id for a given node name
    pub fn get_node_id(&mut self, name: T) -> usize {
        match self.nodes_ids.get(&name) {
            Some(&value) => value,
            _ => {
                let node = Node::<T> {
                    id: name.clone(),
                    incoming_edges: Vec::new(),
                    outgoing_edges: 0,
                    score: 0.15,
                };

                self.nodes.push(node);
                let id = self.nodes.len() - 1;

                self.nodes_ids.insert(name, id);

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
    pub fn nodes(&self) -> Vec<Node<T>> {
        let mut nodes = self.nodes.clone();

        nodes.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        nodes
    }

    /// Calculates a single iteration of the PageRank
    pub fn calculate_step(&mut self) -> f64 {
        self.is_calculating = true;

        let mut current_iteration = self.nodes.clone();

        let nodes = &self.nodes;

        self.nodes
            .iter()
            .enumerate()
            .map(|(id, n)| {
                let score = n
                    .incoming_edges
                    .iter()
                    .map(|edge| {
                        nodes[edge.0].score
                            / nodes[edge.1].outgoing_edges as f64
                    })
                    .sum::<f64>();

                current_iteration[id].score =
                    self.damping.0 + (1f64 - self.damping.0) * score;
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

        convergence.sqrt() / self.len_with_incoming_edges() as f64
    }

    /// Len of all edges
    pub fn len_with_incoming_edges(&mut self) -> usize {
        if let Some(n) = self.nodes_with_inconming_edges {
            return n;
        }

        let total: usize = self
            .nodes
            .iter()
            .map(|r| if r.incoming_edges.is_empty() { 0 } else { 1 })
            .sum();

        if self.is_calculating {
            // it is calculating, save to remember the total
            self.nodes_with_inconming_edges = Some(total);
        }

        total
    }

    /// Return the number of vertices/nodes in the current graph
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the current graph
    pub fn len_edge(&self) -> usize {
        self.edges
    }

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
    fn test_links() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        assert_eq!(0, pr.get_node_id("foo"));
        assert_eq!(1, pr.get_node_id("bar"));

        let n1 = pr.get_node("foo");
        let n2 = pr.get_node("bar");

        assert_eq!(0, n1.incoming_edges.len());
        assert_eq!(1, n1.outgoing_edges);
        assert_eq!(1, n2.incoming_edges.len());
        assert_eq!(0, n2.outgoing_edges);
    }

    #[test]
    fn test_default_score() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        assert_eq!(0.15, pr.get_node("foo").score);
        assert_eq!(0.15, pr.get_node("bar").score);
        assert_eq!(0.15, pr.get_node("xxx").score);
        assert_eq!(0.15, pr.get_node("yyy").score);
    }

    #[test]
    fn test_iteration() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        pr.calculate_step();

        assert_eq!(0.27749999999999997, pr.get_node("foo").score);
        assert_eq!(0.405, pr.get_node("bar").score);
        assert_eq!(0.27749999999999997, pr.get_node("xxx").score);
        assert_eq!(0.15, pr.get_node("yyy").score);
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

        assert_eq!(0.6784874999999999, pr.get_node("foo").score);
        assert_eq!(0.8059875, pr.get_node("bar").score);
        assert_eq!(0.27749999999999997, pr.get_node("xxx").score);
        assert_eq!(0.15, pr.get_node("yyy").score);
    }

    #[test]
    fn test_full_run() {
        let mut pr = Pagerank::<&str>::new();
        pr.add_edge("foo", "bar");
        pr.add_edge("bar", "foo");
        pr.add_edge("xxx", "bar");
        pr.add_edge("yyy", "xxx");

        assert_eq!(16, pr.calculate());

        assert_eq!(1.6152071803888868, pr.get_node("foo").score);
        assert_eq!(1.7427071803888865, pr.get_node("bar").score);
        assert_eq!(0.27749999999999997, pr.get_node("xxx").score);
        assert_eq!(0.15, pr.get_node("yyy").score);
    }
}
