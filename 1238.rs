use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::io::stdin;

#[derive(Eq, PartialEq, Ord)]
struct Value {
    node: u32,
    cost: u32,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost < other.cost {
            Some(Ordering::Less)
        }  else if self.cost > other.cost {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Debug)]
struct Graph {
    edges: Vec<Vec<(u32, u32)>>
}

impl Graph {
    fn new(num_nodes: u32) -> Graph {
        let mut edges = vec![];
        for _ in 0..num_nodes {
            edges.push(vec![])
        }
        Graph {
            edges,
        }
    }
    fn add_edges(&mut self, from: u32, to: u32, cost: u32) {
        self.edges[from as usize].push((to, cost));
    }
}

fn read_line() -> Option<(u32, u32, u32)> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    if buf.trim().is_empty() {
        return None;
    }
    let vec: Vec<u32> = buf.split_ascii_whitespace().flat_map(|x| x.trim().parse::<u32>()).collect();
    Some((vec[0], vec[1], vec[2]))
}

fn get_costs(graph: &Graph, k: u32) -> Vec<u32> {
    let n = graph.edges.len();
    let mut costs = (0..n).map(|_| u32::MAX).collect::<Vec<u32>>();
    let mut heap = BinaryHeap::from([Value { node: k, cost: 0 }]);

    while let Some(Value { node: from, cost }) = heap.pop() {
        if costs[from as usize] < cost {
            continue;
        }
        costs[from as usize] = cost;


        for (to, co) in &graph.edges[from as usize] {
            let next_cost = costs[from as usize] + co;
            if next_cost < costs[*to as usize] {
                costs[*to as usize] = next_cost; // for reduce useless loop
                heap.push(Value { node: *to, cost: next_cost });
            }
        }
    }
    costs
}

fn main() {
    let Some((n, _, k)) = read_line() else {
        return;
    };
    let k = k - 1;

    let mut graph1 = Graph::new(n);
    let mut graph2 = Graph::new(n);

    while let Some((fr, to, cost)) = read_line() {
        graph1.add_edges(fr - 1, to - 1, cost);
        graph2.add_edges(to - 1, fr - 1, cost);
    }

    let cost1 = get_costs(&graph1, k);
    let cost2 = get_costs(&graph2, k);

    let result = cost1.iter().zip(cost2.iter()).map(|(&a, &b)| a + b).max().unwrap();
    println!("{result}");
}
