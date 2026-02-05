use std::collections::BinaryHeap;
use std::env;
use std::fs;
use std::io::{self, Read};

fn read_input(path: Option<&str>) -> io::Result<Vec<u8>> {
    match path {
        Some(p) => fs::read(p),
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}

fn parse_numbers(input: &[u8]) -> Vec<i64> {
    let mut nums = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        if input[i].is_ascii_digit() || input[i] == b'-' {
            let mut sign = 1i64;
            if input[i] == b'-' {
                sign = -1;
                i += 1;
            }
            let mut value: i64 = 0;
            while i < input.len() && input[i].is_ascii_digit() {
                value = value * 10 + (input[i] - b'0') as i64;
                i += 1;
            }
            nums.push(value * sign);
        } else {
            i += 1;
        }
    }
    nums
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn main() {
    let mut part: i32 = 1;
    let mut input_path: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--part" => {
                if let Some(value) = args.next() {
                    part = value.parse::<i32>().unwrap_or(1);
                }
            }
            "--input" => {
                if let Some(value) = args.next() {
                    input_path = Some(value);
                }
            }
            _ => {}
        }
    }

    if part != 1 {
        eprintln!("part 2 not implemented yet");
        std::process::exit(2);
    }

    let input = match read_input(input_path.as_deref()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("failed to read input: {err}");
            std::process::exit(1);
        }
    };

    let nums = parse_numbers(&input);
    if nums.len() % 3 != 0 {
        eprintln!("input does not contain xyz triples");
        std::process::exit(1);
    }

    let mut points: Vec<(i64, i64, i64)> = Vec::new();
    for chunk in nums.chunks(3) {
        points.push((chunk[0], chunk[1], chunk[2]));
    }

    let n = points.len();
    if n < 2 {
        println!("0");
        return;
    }

    let total_pairs = n * (n - 1) / 2;
    let mut k = n / 2;
    if k > 1000 {
        k = 1000;
    }
    if k > total_pairs {
        k = total_pairs;
    }
    let mut heap: BinaryHeap<(u128, usize, usize)> = BinaryHeap::new();

    for i in 0..n {
        let (xi, yi, zi) = points[i];
        for j in (i + 1)..n {
            let (xj, yj, zj) = points[j];
            let dx = (xi - xj) as i128;
            let dy = (yi - yj) as i128;
            let dz = (zi - zj) as i128;
            let dist = (dx * dx + dy * dy + dz * dz) as u128;
            let item = (dist, i, j);
            if heap.len() < k {
                heap.push(item);
            } else if let Some(top) = heap.peek() {
                if item < *top {
                    heap.pop();
                    heap.push(item);
                }
            }
        }
    }

    let mut edges: Vec<(u128, usize, usize)> = heap.into_vec();
    edges.sort_unstable();

    let mut dsu = DSU::new(n);
    for (_, i, j) in edges {
        dsu.union(i, j);
    }

    let mut counts = vec![0usize; n];
    for i in 0..n {
        let root = dsu.find(i);
        counts[root] += 1;
    }
    let mut sizes: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let product: u128 = sizes.iter().take(3).fold(1u128, |acc, &v| acc * v as u128);
    println!("{product}");
}
