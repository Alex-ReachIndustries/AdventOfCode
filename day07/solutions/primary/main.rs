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

fn trim_empty_edges(lines: &mut Vec<&str>) {
    while !lines.is_empty() && lines[0].trim().is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines[lines.len() - 1].trim().is_empty() {
        lines.pop();
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

    let text = String::from_utf8_lossy(&input);
    let mut lines: Vec<&str> = text.lines().collect();
    trim_empty_edges(&mut lines);
    if lines.is_empty() {
        println!("0");
        return;
    }

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(lines.len());
    let mut start_row: usize = 0;
    let mut start_col: usize = 0;
    let mut found_start = false;

    for (r, line) in lines.iter().enumerate() {
        let mut row = line.as_bytes().to_vec();
        if row.len() < width {
            row.resize(width, b'.');
        }
        for (c, ch) in row.iter().enumerate() {
            if *ch == b'S' {
                start_row = r;
                start_col = c;
                found_start = true;
            }
        }
        grid.push(row);
    }

    if !found_start {
        eprintln!("start position S not found");
        std::process::exit(1);
    }

    let height = grid.len();
    let mut active = vec![false; width];
    active[start_col] = true;
    let mut splits: u64 = 0;

    for r in (start_row + 1)..height {
        let mut next = vec![false; width];
        for c in 0..width {
            if !active[c] {
                continue;
            }
            if grid[r][c] == b'^' {
                splits += 1;
                if c > 0 {
                    next[c - 1] = true;
                }
                if c + 1 < width {
                    next[c + 1] = true;
                }
            } else {
                next[c] = true;
            }
        }
        active = next;
    }

    println!("{splits}");
}
