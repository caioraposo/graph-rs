use graph_rs::graph::{Graph, Node};

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

    println!("DFS");
    let (dist, path) = graph.dfs(0, 12);
    print!("{:?} ", dist);
    graph.print_path(&path);
    println!("{}", graph);

    println!("BFS");
    let (dist, path) = graph.bfs(0, 12);
    print!("{:?} ", dist);
    graph.print_path(&path);
    println!("{}", graph);
    
    println!("Dijsktra");
    let (dist, path) = graph.dijsktra(0, 12);
    print!("{:?} ", dist);
    graph.print_path(&path);
    println!("{}", graph);

    println!("A*");
    let (dist, path) = graph.astar(0, 12, &heuristics);
    print!("{:?} ", dist);
    graph.print_path(&path);
    println!("{}", graph);
}
