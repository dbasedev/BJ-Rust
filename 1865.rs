use std::{collections::BinaryHeap, io::{stdin, BufWriter, Read}, ops::Index, str::FromStr};

fn read_line_to_vec<T: FromStr>() -> Vec<T> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
        .trim()
        .split_ascii_whitespace()
        .flat_map(|x| x.trim().parse::<T>())
        .collect::<Vec<T>>()
}

fn read_line_to_vec_to_three_value() -> (u32, u32, u32) {
    let line = read_line_to_vec::<u32>();
    (
        line[0],
        line[1],
        line[2],
    )
}

fn gi<'a, T>(dist: &'a Vec<Option<T>>, index: &'a u32) -> &'a Option<T> {
    &dist[*index as usize]
}

fn find_min_path(n: u32, edges: &Vec<(u32, u32, i32)>) -> &'static str {

    let mut dist: Vec<Option<i32>> = (0..n).map(|_| None).collect();
    
    for first in 0..(n as usize) {
        if dist[first].is_some() {
            continue;
        }
        
        dist[first] = Some(0);
        
        for l in 0..n {
            for (src, dst, cost) in edges.iter() {
                let left = *gi(&dist, src);
                if left.is_none() {
                    continue;
                }
                
                let right  = *gi(&dist, dst);
        
                let next_cost = left.map_or(None, |x| Some(x + *cost));
                if right.is_none() || next_cost < right {
                    if l == n - 1 {
                        return "YES";
                    }
                    dist[*dst as usize] = next_cost;
                }
            }
        }
    }
    
    "NO"
}


fn main() {
    let r = read_line_to_vec::<u32>()[0];
    let mut buffer = String::new();

    for _ in 0..r {
        let (n, m, w) = read_line_to_vec_to_three_value();

        let mut edges: Vec<(u32, u32, i32)> = vec![];

        for _ in 0..m {
            let (s, e, t) = read_line_to_vec_to_three_value();
            let t = t as i32;
            edges.push(( s - 1, e - 1, t));
            edges.push(( e - 1, s - 1, t));
        }
        
        for _ in 0..w {
            let (s, e, t) = read_line_to_vec_to_three_value();
            let t = t as i32;
            edges.push((s - 1, e - 1, -t));
        }
        buffer.push_str(find_min_path(n, &edges));
        buffer.push_str("\n");
    }

    print!("{}", buffer);
}
