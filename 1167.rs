use std::io::{stdin};
use std::mem::swap;
use std::ops::Add;

struct TwoMax<T>  {
    first_max: Option<T>,
    second_max: Option<T>,
}

impl<T: PartialOrd + Copy + Add<Output = T>> TwoMax<T> {
    fn new() -> TwoMax<T> {
        Self {
            first_max: None,
            second_max: None,
        }
    }

    fn insert(&mut self, i: T) {
        let value = Some(i);

        if self.first_max.is_none() || (self.first_max.is_some() && self.first_max < value) {
            swap(&mut self.first_max, &mut self.second_max);
            self.first_max = value;
            return
        }

        if self.second_max.is_none() || (self.second_max.is_some() && self.second_max < value) {
            self.second_max = value;
            return;
        }
    }

    fn sum(&self) -> Option<T> {
        if self.first_max.is_some() && self.second_max.is_some() {
            Some(self.first_max.unwrap() + self.second_max.unwrap())
        } else if self.first_max.is_some() {
            self.first_max
        } else if self.second_max.is_some() {
            self.second_max
        } else {
            None
        }
    }
}

fn main() {
    let mut nodes: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut str = String::new();

    stdin().read_line(&mut str).unwrap();
    let n: i32 = str.trim().parse().unwrap();

    for _ in 0..n {
        nodes.push(Vec::new());
    }

    for _ in 0..n {
        str.clear();
        stdin().read_line(&mut str).unwrap();
        let inputs = str.trim_end().split_ascii_whitespace().flat_map(|x| x.trim().parse()).collect::<Vec<i32>>();
        let from = inputs[0] - 1;
        let from_edges = &mut nodes[from.abs() as usize];
        let mut it = 1;
        while it < inputs.len() {
            let to = inputs[it];
            if to == -1 {
                break;
            }
            let to = to - 1;
            let cost = inputs[it + 1];
            from_edges.push((to, cost));
            it += 2;
        }
    }

    fn recursive(vec: &Vec<Vec<(i32, i32)>>, prev: i32, curr: i32) -> (i32, i32) {
        let mut two_max_depth: TwoMax<i32> = TwoMax::new();
        let mut max_width = 0i32;

        let curr_node = &vec[curr.abs() as usize];

        for (next, next_cost) in curr_node.iter() {
            if *next != prev {
                let (next_max_width, next_max_depth) = recursive(vec, curr, *next);
                let next_max_depth = next_max_depth + *next_cost;
                two_max_depth.insert(next_max_depth);
                if max_width < next_max_width {
                    max_width = next_max_width;
                }
            }
        }

        let best_depth = two_max_depth.first_max.map_or(0, |x| x);
        let new_max_width = two_max_depth.sum().map_or(0, |x| x);

        (if max_width < new_max_width { new_max_width } else { max_width }, best_depth)
    }
    // println!("{:?}", nodes);
    let (answer, _) = recursive(&nodes, -1, 0);
    println!("{answer}");
}
