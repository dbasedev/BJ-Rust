use std::collections::VecDeque;
use std::io::{Read, stdin};

fn main() {
    let mut str = String::new();
    stdin().read_to_string(&mut str).unwrap();
    let mut stack = VecDeque::new();
    let mut result = Vec::new();
    for c in str.trim().chars() {
        match c {
            '(' => {
                stack.push_back(c);
            },
            ')' => {
                while let Some(v) = stack.pop_back() {
                    if v == '(' {
                        break;
                    }
                    result.push(v);
                }
            }
            '*' | '/' => {
                while let Some(v) = stack.pop_back() {
                    if ['*', '/'].contains(&v) {
                        result.push(v);
                    } else {
                        stack.push_back(v);
                        break;
                    }
                }
                stack.push_back(c);
            },
            '+' | '-' => {
                while let Some(v) = stack.pop_back() {
                    if v == '(' {
                        stack.push_back(v);
                        break;
                    }
                    result.push(v);
                }
                stack.push_back(c);
            },
            _ => { result.push(c) },
        }
    }

    while let Some(v) = stack.pop_back() {
        result.push(v);
    }

    println!("{}", result.into_iter().collect::<String>());
}
