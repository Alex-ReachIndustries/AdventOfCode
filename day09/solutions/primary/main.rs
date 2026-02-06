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
    if nums.len() % 2 != 0 {
        eprintln!("input does not contain xy pairs");
        std::process::exit(1);
    }

    let mut points: Vec<(i64, i64)> = Vec::new();
    for chunk in nums.chunks(2) {
        points.push((chunk[0], chunk[1]));
    }

    let mut best: u128 = 0;
    for i in 0..points.len() {
        let (xi, yi) = points[i];
        for j in (i + 1)..points.len() {
            let (xj, yj) = points[j];
            let dx = (xi - xj).abs() as u128;
            let dy = (yi - yj).abs() as u128;
            let area = (dx + 1) * (dy + 1);
            if area > best {
                best = area;
            }
        }
    }

    println!("{best}");
}
