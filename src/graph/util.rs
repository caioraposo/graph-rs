use crate::graph::Graph;
use std::collections::{VecDeque, HashMap, BinaryHeap};
use std::cmp::Reverse;

impl<T: PartialEq> Graph<T> {

    pub fn dfs(&mut self, source: usize, target: usize) -> (u64, VecDeque<usize>) {
        self.unmark();
        let mut stack = Vec::new();
        let mut map = HashMap::new();
        let mut cicles = 0;
        let mut dist = 0;
        stack.push((dist, source));

        while let Some((dist_u, u)) = stack.pop() {
            if self.nodes[u].visited != 0 { continue; }
            cicles += 1;
            for (e, v) in self.adj_list(u) {
                if self.nodes[v].visited == 0 {
                    stack.push((dist_u + self.edges[e].weight, v));
                    map.insert(v, u);
                }
            }
            dist = dist_u;
            self.nodes[u].visited = cicles;
            if u == target { break; }
        }
        let path = Graph::<T>::parse_path(source, target, map);
        (dist, path)
    }

    pub fn bfs(&mut self, source: usize, target: usize) -> (u64, VecDeque<usize>) {
        self.unmark();
        let mut queue = VecDeque::new();
        let mut map = HashMap::new();
        let mut cicles = 0;
        let mut dist = 0;
        queue.push_back((dist, source));

        while let Some((dist_u, u)) = queue.pop_front() {
            if self.nodes[u].visited != 0 { continue; }
            cicles += 1;
            for (e, v) in self.adj_list(u) {
                if self.nodes[v].visited == 0 {
                    queue.push_back((dist_u + self.edges[e].weight, v));
                    map.insert(v, u);
                }
            }
            dist = dist_u;
            self.nodes[u].visited = cicles;
            if u == target { break; }
        }
        let path = Graph::<T>::parse_path(source, target, map);
        (dist, path)
    }

    // Single-source shortest paths on a graph with non-negative weights
    pub fn dijsktra(&mut self, source: usize, target: usize) -> (u64, VecDeque<usize>) {
        self.unmark();
        let mut heap = BinaryHeap::new();
        let mut map = HashMap::new();
        let mut dist = 0;
        let mut cicles = 0;

        heap.push((Reverse(0), source));
        while let Some((Reverse(dist_u), u)) = heap.pop() {
            if self.nodes[u].visited != 0 { continue; }
            cicles += 1;
            for (e, v) in self.adj_list(u) {
                if self.nodes[v].visited == 0 {
                    let dist_v = dist_u + self.edges[e].weight;
                    heap.push((Reverse(dist_v), v));
                    map.insert(v, u);
                }
            }
            dist = dist_u;
            self.nodes[u].visited = cicles;
            if u == target { break; }
        }

        let path = Graph::<T>::parse_path(source, target, map);
        (dist, path)
    }

    // Same as dijsktra but with heuristics
    pub fn astar(&mut self, source: usize, target: usize, heu: &Vec<u64>) -> (u64, VecDeque<usize>) {
        self.unmark();
        let mut heap = BinaryHeap::new();
        let mut map = HashMap::new();
        let mut dist = 0;
        let cost_h = heu[source];
        let mut cicles = 0;

        heap.push((Reverse(cost_h), dist, source));
        while let Some((Reverse(_), dist_u, u)) = heap.pop() {
            if self.nodes[u].visited != 0 { continue; }
            cicles += 1;
            for (e, v) in self.adj_list(u) {
                if self.nodes[v].visited == 0 {
                    let dist_v = dist_u + self.edges[e].weight; // g(n)
                    let cost_h = dist_v + heu[v]; // f(n) = g(n) + h(n)
                    heap.push((Reverse(cost_h), dist_v, v));
                    map.insert(v, u);
                }
            }
            dist = dist_u;
            self.nodes[u].visited = cicles;
            if u == target { break; }
        }

        let path = Graph::<T>::parse_path(source, target, map);
        (dist, path)
    }


    // Create the path Vector from the HashMap
    fn parse_path(source: usize, target: usize, map: HashMap<usize, usize>) -> VecDeque<usize> {
        let mut path = VecDeque::new();

        path.push_front(target);
        while let Some(n) = map.get(&path[0]) { 
            path.push_front(*n);
            if *n == source { break; }
        };

        path
    }
}