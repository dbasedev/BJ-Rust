use std::io::Read;

fn read_matrix() -> Vec<Vec<bool>> {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let mut vecs = vec![];

    stdin.read_line(&mut input).unwrap();
    let sizes = input.split_ascii_whitespace().flat_map(str::parse::<usize>).collect::<Vec<usize>>();
    input.clear();
    for _ in 0..sizes[0] {
        stdin.read_line(&mut input).unwrap();
        let vec = input.trim().chars().map(|x| x == 'W').collect::<Vec<bool>>();
        vecs.push(vec);
        input.clear();
    }

    vecs
}

fn sanitizer(vecs: &mut Vec<Vec<bool>>, start: bool) {
    for (i, vec) in vecs.iter_mut().enumerate() {
        for (j, v) in vec.iter_mut().enumerate() {
            *v = if (i % 2 == 0 && j % 2 == 0) || (i % 2 == 1 && j % 2 == 1) {
                start != *v
            } else {
                start == *v
            };
        }
    }
}

struct Checker<'a> {
    vec: &'a Vec<Vec<bool>>,
    i: usize,
    j: usize,
}

impl<'a> Checker<'a> {
    fn new(vec: &Vec<Vec<bool>>) -> Checker {
        Checker {
            vec,
            i: 0,
            j: 0
        }
    }
}

impl Iterator for Checker<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let (n, m) = (self.vec.len(), self.vec[0].len());

        if self.i == n - 7 {
            return None
        }

        let mut cnt = 0u32;

        for i in self.i..(self.i+8) {
            for j in self.j..(self.j+8) {
                cnt += if self.vec[i][j] { 1 } else { 0 };
            }
        }

        self.j += 1;

        if self.j == m - 7 {
            self.i += 1;
            self.j = 0;
        }

        Some(cnt)
    }
}



fn main() {
    let vec = read_matrix();
    let mut start_with_black = vec.clone();
    let mut start_with_white = vec.clone();

    sanitizer(&mut start_with_white, true);
    sanitizer(&mut start_with_black, false);

    let min1 = Checker::new(&start_with_white).into_iter().min().unwrap();
    let min2 = Checker::new(&start_with_black).into_iter().min().unwrap();


    println!("{}", [min1, min2].iter().min().unwrap());
}
