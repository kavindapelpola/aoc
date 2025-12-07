use std::collections::HashMap;
use std::hash::Hash;

const DEFAULT_CAPACITY: usize = 100_000;
const DEFAULT_EDGE_CAPACITY: usize = 10;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge<K: Hash + PartialEq + Eq + Clone> {
    pub to: K,
    pub dist: i32,
}

impl<K: Hash + PartialEq + Eq + Clone> Edge<K> {
    pub fn new(to: K, dist: i32) -> Self {
        Edge { to, dist }
    }
}

pub struct Graph<K: Hash + PartialEq + Eq + Clone, S: Clone + PartialEq + Eq + Hash> {
    pub nodes: HashMap<K, S>,
    pub edges: HashMap<K, Vec<Edge<K>>>,
}

impl<K: Hash + PartialEq + Eq + Clone, S: Clone + PartialEq + Eq + Hash> Graph<K, S> {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::with_capacity(DEFAULT_CAPACITY),
            edges: HashMap::with_capacity(DEFAULT_CAPACITY),
        }
    }

    pub fn add_node(&mut self, key: K, state: S) {
        self.edges
            .insert(key.clone(), Vec::with_capacity(DEFAULT_EDGE_CAPACITY));
        self.nodes.insert(key, state);
    }

    pub fn add_edge(&mut self, from: K, to: K, cost: i32) {
        let from_edges = self.edges.get_mut(&from).unwrap();
        from_edges.push(Edge::new(to, cost));
    }

    pub fn add_edges(&mut self, from: K, tos: Vec<Edge<K>>) {
        let from_edges = self.edges.get_mut(&from).unwrap();
        from_edges.extend(tos);
    }

    pub fn add_unit_edges(&mut self, from: K, tos: Vec<K>) {
        let from_edges = self.edges.get_mut(&from).unwrap();
        from_edges.extend(
            tos.iter()
                .map(|to| Edge::new(to.clone(), 1))
                .collect::<Vec<Edge<K>>>(),
        );
    }

    pub fn get_edges(&self, from: K) -> &Vec<Edge<K>> {
        self.edges.get(&from).unwrap()
    }

    // Floyd-Warshall algorithm to find the shortest path between all pairs of nodes in a directed weighted graph
    // https://en.wikipedia.org/wiki/Floyd–Warshall_algorithm
    pub fn shortest_paths(&self) -> HashMap<K, HashMap<K, i32>> {
        let mut dist = HashMap::with_capacity(self.nodes.len());

        // initialise dist matrix with infinity for all pairs except for the diagonal which is 0
        for (from, _) in &self.nodes {
            let mut from_dist = HashMap::with_capacity(self.nodes.len());
            for (to, _) in &self.nodes {
                if from == to {
                    from_dist.insert(to.clone(), 0);
                } else {
                    from_dist.insert(to.clone(), i32::MAX);
                }
            }
            dist.insert(from.clone(), from_dist);
        }

        // update dist matrix with edge distances
        for (from, _) in &self.nodes {
            for (to, _) in &self.nodes {
                let from_to_dist = dist.get_mut(&from).unwrap().get_mut(&to).unwrap();
                let from_to_edges = self.edges.get(&from).unwrap();
                for edge in from_to_edges {
                    if edge.to == *to {
                        *from_to_dist = edge.dist;
                    }
                }
            }
        }

        // Floyd-Warshall algorithm
        for (k, _) in &self.nodes {
            for (i, _) in &self.nodes {
                for (j, _) in &self.nodes {
                    let i_j_dist = dist.get(&i).unwrap().get(&j).unwrap().clone();
                    let i_k_dist = dist.get(&i).unwrap().get(&k).unwrap().clone();
                    let k_j_dist = dist.get(&k).unwrap().get(&j).unwrap().clone();
                    if i_k_dist != i32::MAX
                        && k_j_dist != i32::MAX
                        && i_j_dist > i_k_dist + k_j_dist
                    {
                        let i_j_dist_mut = dist.get_mut(&i).unwrap().get_mut(&j).unwrap();
                        *i_j_dist_mut = i_k_dist + k_j_dist;
                    }
                }
            }
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node_from_state_creates_node_and_edge() {
        let mut graph = Graph::<&str, i32>::new();
        graph.add_node("122", 122);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn add_edge_adds_edge_when_node_exists() {
        let mut graph = Graph::<&str, i32>::new();
        graph.add_node("122", 122);
        graph.add_edge("122", "123", 4);
        assert_eq!(graph.get_edges("122"), &vec![Edge::new("123", 4)]);
    }

    #[test]
    fn add_unit_edges_adds_multiple_edges_with_unit_cost() {
        let mut graph = Graph::<&str, i32>::new();
        graph.add_node("122", 122);
        graph.add_unit_edges("122", vec!["123", "124", "125"]);
        assert_eq!(
            graph.get_edges("122"),
            &vec![
                Edge::new("123", 1),
                Edge::new("124", 1),
                Edge::new("125", 1)
            ]
        );
    }

    #[test]
    fn shortest_path_returns_correct_matrix() {
        // testing with input from wikipedia example
        let mut graph = Graph::<&str, i32>::new();
        graph.add_node("1", 1);
        graph.add_node("2", 2);
        graph.add_node("3", 3);
        graph.add_node("4", 4);
        graph.add_edge("2", "1", 4);
        graph.add_edge("1", "3", -2);
        graph.add_edge("2", "3", 3);
        graph.add_edge("3", "4", 2);
        graph.add_edge("4", "2", -1);
        let shortest = graph.shortest_paths();
        assert_eq!(shortest.get("1").unwrap().get("1").unwrap(), &0);
        assert_eq!(shortest.get("1").unwrap().get("2").unwrap(), &-1);
        assert_eq!(shortest.get("1").unwrap().get("3").unwrap(), &-2);
        assert_eq!(shortest.get("1").unwrap().get("4").unwrap(), &0);

        assert_eq!(shortest.get("2").unwrap().get("1").unwrap(), &4);
        assert_eq!(shortest.get("2").unwrap().get("2").unwrap(), &0);
        assert_eq!(shortest.get("2").unwrap().get("3").unwrap(), &2);
        assert_eq!(shortest.get("2").unwrap().get("4").unwrap(), &4);

        assert_eq!(shortest.get("3").unwrap().get("1").unwrap(), &5);
        assert_eq!(shortest.get("3").unwrap().get("2").unwrap(), &1);
        assert_eq!(shortest.get("3").unwrap().get("3").unwrap(), &0);
        assert_eq!(shortest.get("3").unwrap().get("4").unwrap(), &2);

        assert_eq!(shortest.get("4").unwrap().get("1").unwrap(), &3);
        assert_eq!(shortest.get("4").unwrap().get("2").unwrap(), &-1);
        assert_eq!(shortest.get("4").unwrap().get("3").unwrap(), &1);
        assert_eq!(shortest.get("4").unwrap().get("4").unwrap(), &0);
    }
}
