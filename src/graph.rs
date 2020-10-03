#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub marked: bool,
    first: Option<usize>,
}

#[derive(Debug)]
pub struct Edge {
    pub target: usize,
    pub weight: u32,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

impl Edge {
    pub fn new(target: usize, weight: u32) -> Self {
        Edge { target, weight, next: None }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, marked: false, first: None }
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
    pub fn add_edge(&mut self, source: usize, target: usize, weight: u32) {
        let mut e = Edge::new(target, weight);
        e.next = self.nodes[source].first;
        self.nodes[source].first = Some(self.edges.len());
        self.edges.push(e);
    }

    // An undirected edge is just two directed edges
    pub fn add_undirected_edge(&mut self, source: usize, target: usize, weight: u32) {
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
                
}

impl<'a> Graph<&'a str> {

    pub fn add_from_str_vec(&mut self, vec: Vec<&'a str>) {
        for data in vec {
            self.nodes.push(Node::new(data));
        }
    }

    pub fn add_edge_str(&mut self, source: &'a str, target: &'a str, weight: u32) {
        let s = self.get_node_index(source).expect("Node not in the graph.");
        let t = self.get_node_index(target).expect("Node not in the graph.");
        self.add_edge(s, t, weight);
    }

    pub fn add_undirected_edge_str(&mut self, source: &'a str, target: &'a str, weight: u32) {
        let s = self.get_node_index(source).expect("Node not in the graph.");
        let t = self.get_node_index(target).expect("Node not in the graph.");
        self.add_edge(s, t, weight);
        self.add_edge(t, s, weight);
    }
}
