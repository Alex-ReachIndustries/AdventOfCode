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
    for line in lines {
        let mut row = line.as_bytes().to_vec();
        if row.len() < width {
            row.resize(width, b' ');
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = width;
    let mut segments: Vec<(usize, usize)> = Vec::new();
    let mut in_segment = false;
    let mut start = 0;

    for col in 0..cols {
        let mut all_space = true;
        for row in &grid {
            if row[col] != b' ' {
                all_space = false;
                break;
            }
        }
        if !all_space {
            if !in_segment {
                start = col;
                in_segment = true;
            }
        } else if in_segment {
            segments.push((start, col - 1));
            in_segment = false;
        }
    }
    if in_segment {
        segments.push((start, cols - 1));
    }

    let mut total: u128 = 0;
    for (start_col, end_col) in segments {
        let op_row = &grid[rows - 1];
        let mut op: u8 = b' ';
        for col in start_col..=end_col {
            if op_row[col] != b' ' {
                op = op_row[col];
                break;
            }
        }
        if op != b'+' && op != b'*' {
            continue;
        }

        let mut values: Vec<u128> = Vec::new();
        for row in &grid[..rows - 1] {
            let mut value: u128 = 0;
            let mut has_digit = false;
            for col in start_col..=end_col {
                let ch = row[col];
                if ch.is_ascii_digit() {
                    value = value * 10 + (ch - b'0') as u128;
                    has_digit = true;
                }
            }
            if has_digit {
                values.push(value);
            }
        }

        if values.is_empty() {
            continue;
        }

        let result = if op == b'+' {
            values.iter().copied().sum::<u128>()
        } else {
            values.iter().copied().product::<u128>()
        };
        total += result;
    }

    println!("{total}");
}
