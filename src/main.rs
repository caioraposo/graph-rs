use graph_rs::graph::Graph;

fn main() {
    let mut graph = Graph::new(1,1);
    let v = vec!["test", "again"];
    graph.add_from_str_vec(v);
    graph.add_undirected_edge(0, 1, 10);

    assert_eq!(graph.get_node_index("test"), Some(0));
    assert_eq!(graph.get_node_index("again"), Some(1));
}
