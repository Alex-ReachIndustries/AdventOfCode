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

fn count_zero_positions(input: &[u8]) -> i64 {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        while i < input.len() && (input[i] == b'\n' || input[i] == b'\r' || input[i] == b' ') {
            i += 1;
        }
        if i >= input.len() {
            break;
        }

        let dir = input[i];
        i += 1;

        let mut value: i64 = 0;
        while i < input.len() && input[i].is_ascii_digit() {
            value = value * 10 + (input[i] - b'0') as i64;
            i += 1;
        }

        if dir == b'L' {
            pos = (pos - value).rem_euclid(100);
        } else if dir == b'R' {
            pos = (pos + value).rem_euclid(100);
        } else {
            // Skip unknown line prefix.
            while i < input.len() && input[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        if pos == 0 {
            count += 1;
        }

        while i < input.len() && input[i] != b'\n' {
            i += 1;
        }
    }

    count
}

fn count_zero_hits(input: &[u8]) -> i64 {
    let mut pos: i64 = 50;
    let mut count: i64 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        while i < input.len() && (input[i] == b'\n' || input[i] == b'\r' || input[i] == b' ') {
            i += 1;
        }
        if i >= input.len() {
            break;
        }

        let dir = input[i];
        i += 1;

        let mut value: i64 = 0;
        while i < input.len() && input[i].is_ascii_digit() {
            value = value * 10 + (input[i] - b'0') as i64;
            i += 1;
        }

        if dir != b'L' && dir != b'R' {
            while i < input.len() && input[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        let rem = if dir == b'R' {
            (100 - pos.rem_euclid(100)).rem_euclid(100)
        } else {
            pos.rem_euclid(100)
        };
        let first = if rem == 0 { 100 } else { rem };
        if value >= first {
            count += 1 + (value - first) / 100;
        }

        if dir == b'L' {
            pos = (pos - value).rem_euclid(100);
        } else {
            pos = (pos + value).rem_euclid(100);
        }

        while i < input.len() && input[i] != b'\n' {
            i += 1;
        }
    }

    count
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

    if part != 1 && part != 2 {
        eprintln!("unsupported part: {part}");
        std::process::exit(2);
    }

    let input = match read_input(input_path.as_deref()) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("failed to read input: {err}");
            std::process::exit(1);
        }
    };

    let answer = if part == 1 {
        count_zero_positions(&input)
    } else {
        count_zero_hits(&input)
    };
    println!("{answer}");
}
