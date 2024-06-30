use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::stdin;

fn read_line() -> Vec<usize> {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_ascii_whitespace()
        .flat_map(|x| x.trim().parse())
        .collect()
}

#[derive(Eq, Ord)]
struct HeapValue {
    from: usize,
    cost: Option<usize>,
}

impl PartialEq for HeapValue {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for HeapValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost == other.cost {
            Some(Ordering::Equal)
        } else if self.cost.is_none() || self.cost < other.cost {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

fn main() {
    let vec = read_line();
    let (n, m) = (vec[0], vec[1]);
    let start = read_line()[0] - 1;
    let mut edges = vec![vec![]; n];

    for _ in 0..m {
        let line = read_line();
        let (fr, to, cost) = (line[0], line[1], line[2]);
        edges[fr - 1].push((to - 1, cost));
    }

    let mut queue = BinaryHeap::new();

    let mut dist = vec![None; n];

    queue.push(HeapValue {
        cost: Some(0usize),
        from: start,
    });

    while !queue.is_empty() {
        let Some(HeapValue { from: fr, cost }) = queue.pop() else {
            break;
        };

        if dist[fr].is_none() || cost <= dist[fr] {
            dist[fr] = cost;
        } else {
            continue;
        }

        for (to, to_cost) in edges[fr].iter() {
            let next_cost = dist[fr].map(|x| x + to_cost);
            if dist[*to].is_none() || dist[*to] > next_cost {
                dist[*to] = next_cost;
                queue.push(HeapValue {
                    cost: next_cost,
                    from: *to,
                });
            }
        }
    }

    for d in dist {
        if d.is_none() {
            println!("INF");
        } else {
            println!("{}", d.unwrap());
        }
    }
}
