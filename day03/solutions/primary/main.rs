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

fn max_joltage(line: &[u8]) -> u32 {
    if line.len() < 2 {
        return 0;
    }
    let mut best: u32 = 0;
    let mut max_right: u32 = (line[line.len() - 1] - b'0') as u32;
    for idx in (0..line.len() - 1).rev() {
        let d = (line[idx] - b'0') as u32;
        let value = d * 10 + max_right;
        if value > best {
            best = value;
        }
        if d > max_right {
            max_right = d;
        }
    }
    best
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

    let mut total: u64 = 0;
    let mut start: usize = 0;
    let mut i: usize = 0;
    while i <= input.len() {
        if i == input.len() || input[i] == b'\n' {
            let mut end = i;
            if end > start && input[end - 1] == b'\r' {
                end -= 1;
            }
            if end > start {
                total += max_joltage(&input[start..end]) as u64;
            }
            start = i + 1;
        }
        i += 1;
    }

    println!("{total}");
}
