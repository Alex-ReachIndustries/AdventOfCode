use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read};

const BASE: u64 = 1_000_000_000;

#[derive(Clone, Debug)]
struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    fn zero() -> Self {
        Self { digits: Vec::new() }
    }

    fn one() -> Self {
        Self { digits: vec![1] }
    }

    fn add_assign(&mut self, other: &Self) {
        let max_len = self.digits.len().max(other.digits.len());
        if self.digits.len() < max_len {
            self.digits.resize(max_len, 0);
        }
        let mut carry: u64 = 0;
        for i in 0..max_len {
            let a = self.digits[i] as u64;
            let b = if i < other.digits.len() {
                other.digits[i] as u64
            } else {
                0
            };
            let sum = a + b + carry;
            self.digits[i] = (sum % BASE) as u32;
            carry = sum / BASE;
        }
        if carry > 0 {
            self.digits.push(carry as u32);
        }
        while self.digits.last() == Some(&0) {
            self.digits.pop();
        }
    }

    fn to_string(&self) -> String {
        if self.digits.is_empty() {
            return "0".to_string();
        }
        let mut iter = self.digits.iter().rev();
        let first = iter.next().unwrap();
        let mut out = first.to_string();
        for d in iter {
            out.push_str(&format!("{:09}", d));
        }
        out
    }
}

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

fn get_id(name: &str, ids: &mut HashMap<String, usize>, adj: &mut Vec<Vec<usize>>) -> usize {
    if let Some(&id) = ids.get(name) {
        return id;
    }
    let id = adj.len();
    ids.insert(name.to_string(), id);
    adj.push(Vec::new());
    id
}

fn dfs(
    node: usize,
    out_id: usize,
    adj: &[Vec<usize>],
    state: &mut [u8],
    memo: &mut [Option<BigNum>],
) -> BigNum {
    if state[node] == 2 {
        return memo[node].as_ref().unwrap().clone();
    }
    if state[node] == 1 {
        panic!("cycle detected in graph");
    }
    state[node] = 1;

    let result = if node == out_id {
        BigNum::one()
    } else {
        let mut total = BigNum::zero();
        for &child in &adj[node] {
            let child_count = dfs(child, out_id, adj, state, memo);
            total.add_assign(&child_count);
        }
        total
    };

    memo[node] = Some(result.clone());
    state[node] = 2;
    result
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

    let text = String::from_utf8_lossy(&input);
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut adj: Vec<Vec<usize>> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, ':');
        let name = parts.next().unwrap().trim();
        let outputs = parts.next().unwrap_or("").trim();
        let id = get_id(name, &mut ids, &mut adj);
        adj[id].clear();
        if !outputs.is_empty() {
            for out in outputs.split_whitespace() {
                let out_id = get_id(out, &mut ids, &mut adj);
                adj[id].push(out_id);
            }
        }
    }

    let start_id = match ids.get("you") {
        Some(&id) => id,
        None => {
            println!("0");
            return;
        }
    };
    let out_id = match ids.get("out") {
        Some(&id) => id,
        None => {
            println!("0");
            return;
        }
    };

    let mut state = vec![0u8; adj.len()];
    let mut memo = vec![None; adj.len()];
    let result = dfs(start_id, out_id, &adj, &mut state, &mut memo);
    println!("{}", result.to_string());
}
