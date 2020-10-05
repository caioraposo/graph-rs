use graph_rs::graph::Graph;

fn main() {
    let mut graph = Graph::new(13,16);

    let input: Vec<&str> = include_str!("../template").lines().collect();
    let mut input = input.iter();
    // Add nodes
    graph.add_from_str_vec(input.next().unwrap().split_whitespace().collect());

    let heuristics: Vec<u64> = input.next()
                                    .unwrap()
                                    .split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();
    // Add edges
    for line in input {
        if line.len() == 0 { break; } // EOF
        let e: Vec<&str> = line.split_whitespace().collect();
        graph.add_undirected_edge_str(e[0], e[1], e[2].parse().unwrap());
    }

    println!("{}", graph);
    println!("Dijsktra");
    let (dist, path) = graph.dijsktra(0, 12, false);
    println!("{:?} {:?}", dist, path);

    println!();
    println!("{}", graph);

    println!("A*");
    let (dist, path) = graph.astar(0, 12, &heuristics, false);
    println!("{:?} {:?}", dist, path);
    println!("{}", graph);
}
