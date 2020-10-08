mod util;

use std::fmt;
use std::fmt::Display;
use std::collections::VecDeque;

pub struct Node<T> {
    pub data: T,
    // By using a integer we can keep track of the order they were visited
    pub visited: u64,
    first: Option<usize>,
}

pub struct Edge {
    pub target: usize,
    pub weight: u64,
    next: Option<usize>,
}

pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl Edge {
    pub fn new(target: usize, weight: u64) -> Self {
        Edge { target, weight, next: None }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, visited: 0, first: None }
    }
}

impl<T: PartialEq> Graph<T> {

    pub fn new(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(nodes),
            edges: Vec::with_capacity(edges),
        }
    }

    pub fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    /// Given the index of two nodes create a edge between them
    pub fn add_edge(&mut self, source: usize, target: usize, weight: u64) {
        let mut e = Edge::new(target, weight);
        e.next = self.nodes[source].first;
        self.nodes[source].first = Some(self.edges.len());
        self.edges.push(e);
    }

    // Just two directed edges
    pub fn add_undirected_edge(&mut self, source: usize, target: usize, weight: u64) {
        self.add_edge(source, target, weight);
        self.add_edge(target, source, weight);
    }

    pub fn get_node_index(&self, data: T) -> Option<usize> {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.data == data {
                return Some(i);
            }
        }
        None
    }
 
    /// Gets vertex u's adjacency list.
    pub fn adj_list(&self, u: usize) -> AdjListIterator<T> {
        AdjListIterator {
            graph: self,
            next_e: self.nodes[u].first,
        }
    }

    // Unmark all nodes
    pub fn unmark(&mut self) {
        for node in &mut self.nodes {
            node.visited = 0;
        }
    }
               
}

impl<'a> Graph<&'a str> {

    pub fn add_from_str_vec(&mut self, vec: Vec<&'a str>) {
        for data in vec {
            self.nodes.push(Node::new(data));
        }
    }

    pub fn add_edge_str(&mut self, source: &'a str, target: &'a str, weight: u64) {
        let s = self.get_node_index(source).expect("Node not in the graph.");
        let t = self.get_node_index(target).expect("Node not in the graph.");
        self.add_edge(s, t, weight);
    }

    pub fn add_undirected_edge_str(&mut self, source: &'a str, target: &'a str, weight: u64) {
        let s = self.get_node_index(source).expect("Node not in the graph.");
        let t = self.get_node_index(target).expect("Node not in the graph.");
        self.add_edge(s, t, weight);
        self.add_edge(t, s, weight);
    }

    pub fn print_path(&self, path: &VecDeque<usize>) {
        for i in path {
            print!("{} -> ", self.nodes[*i].data);
        }
        println!("λ");
    }
}

impl<T: Display + PartialEq> Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for (i, node) in self.nodes.iter().enumerate() {
            if node.visited != 0 {
                write!(f, "{:2} {} [{}]: ", i, node.data, node.visited).ok();
            } else {
                write!(f, "{:2} {} [ ]: ", i, node.data).ok();
            }
            for (e, v) in self.adj_list(i) {
                write!(f, "{}({}) -> ", self.nodes[v].data, self.edges[e].weight).ok();
            }
            write!(f, "λ").ok();
            writeln!(f).ok();
        }
        Ok(())
    }
}


/// An iterator for convenient adjacency list traversal.
pub struct AdjListIterator<'a, T> {
    graph: &'a Graph<T>,
    next_e: Option<usize>,
}

impl<'a, T> Iterator for AdjListIterator<'a, T> {
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        self.next_e.map(|e| {
            let v = self.graph.edges[e].target;
            self.next_e = self.graph.edges[e].next;
            (e, v)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_node_is_in_the_graph() {
        let mut graph = Graph::new(1, 0);
        let n1= Node::new(1);
        let n2= Node::new(2);
        graph.add_node(n1);
        graph.add_node(n2);

        assert_eq!(graph.nodes[0].data, 1);
        assert_eq!(graph.nodes[1].data, 2);
    }

    #[test]
    fn add_directed_edge() {
        let mut graph = Graph::new(1, 0);
        let n1= Node::new(1);
        let n2= Node::new(2);
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_edge(0, 1, 0);

        assert_eq!(graph.edges[0].target, 1);
        assert_eq!(graph.edges[0].weight, 0);
    }

    #[test]
    fn add_undirected_edge() {
        let mut graph = Graph::new(1, 0);
        let n1= Node::new(1);
        let n2= Node::new(2);
        graph.add_node(n1);
        graph.add_node(n2);
        graph.add_undirected_edge(0, 1, 0);

        assert_eq!(graph.edges[0].target, 1);
        assert_eq!(graph.edges[0].weight, 0);
        assert_eq!(graph.edges[1].target, 0);
        assert_eq!(graph.edges[1].weight, 0);
    }
}
