use std::collections::VecDeque;
use std::io::{Read, stdin};

#[derive(Debug)]
struct BJMap {
    n: usize,
    m: usize,
    map: Vec<Vec<char>>,
}

impl BJMap {
    fn from_stdin() -> BJMap {
        let mut stdin = stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).map_err(|_| ()).unwrap();
        let first_line = buf
            .trim()
            .split_ascii_whitespace()
            .flat_map(|x| x.trim().parse())
            .collect::<Vec<usize>>();

        let (n, m) = (first_line[0], first_line[1]);

        buf.clear();
        stdin.read_to_string(&mut buf).unwrap();
        let map = buf
            .trim()
            .split('\n')
            .map(|x| x.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();


        BJMap {
            n,
            m,
            map,
        }
    }

    fn iter(&self) -> BJMapIterator {
        BJMapIterator::new(self)
    }
}

#[derive(Debug)]
struct VisitedChar<'a>(&'a BJMap, [bool; 26]);

impl<'a> VisitedChar<'a> {
    fn new(map: &'a BJMap) -> VisitedChar<'a> {
        VisitedChar(map, [false; 26])
    }

    fn check_true(&mut self, i: usize, j: usize) {
        let ascii = self.0.map[i][j] as u8;
        let index = (ascii - 'A' as u8) as usize;
        self.1[index] = true;
    }

    fn check_false(&mut self, i: usize, j: usize) {
        let ascii = self.0.map[i][j] as u8;
        let index = (ascii - b'A') as usize;
        self.1[index] = false;
    }

    fn is_visited(&self, i: usize, j: usize) -> bool {
        let ascii = self.0.map[i][j] as u8;
        let index = (ascii - b'A') as usize;
        self.1[index]
    }
}

#[derive(Debug)]
struct BJMapIterator<'a> {
    origin_map: &'a BJMap,
    visited: VisitedChar<'a>,
    stack: VecDeque<(usize, usize, usize, Path)>,

}

impl<'a> BJMapIterator<'a> {
    fn new(bj_map: &BJMap) -> BJMapIterator {
        let mut stack = VecDeque::new();
        stack.push_back((0, 0, 1, Path::None));
        BJMapIterator {
            origin_map: bj_map,
            visited: VisitedChar::new(bj_map),
            stack,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Path {
    None = 0,
    Left = 1,
    Right = 2,
    Down = 3,
    Done = 4
}

impl<'a> Iterator for BJMapIterator<'a> {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, cost, path) = self.stack.pop_back()?;

        self.visited.check_true(i, j);

        if path <= Path::None && j != 0 && !self.visited.is_visited(i, j - 1) {
            self.stack.push_back((i, j, cost, Path::Left));
            self.stack.push_back((i, j - 1, cost + 1, Path::None));
        } else if path <= Path::Left && j + 1 < self.origin_map.m && !self.visited.is_visited(i, j + 1) {
            self.stack.push_back((i, j, cost, Path::Right));
            self.stack.push_back((i, j + 1, cost + 1, Path::None));
        } else if path <= Path::Right && i + 1 < self.origin_map.n && !self.visited.is_visited(i + 1, j) {
            self.stack.push_back((i, j, cost, Path::Down));
            self.stack.push_back((i + 1, j, cost + 1, Path::None));
        } else if path <= Path::Down && i != 0 && !self.visited.is_visited(i - 1, j) {
            self.stack.push_back((i, j, cost, Path::Done));
            self.stack.push_back((i - 1, j, cost + 1, Path::None));
        } else {
            self.visited.check_false(i, j);
        }
        Some((i, j, cost))
    }
}

fn main() {
    let answer = BJMap::from_stdin().iter().map(|(_, _, v)| v).max().unwrap();
    println!("{}", answer);
}
