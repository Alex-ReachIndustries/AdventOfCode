use std::cmp::Ordering;
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

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(u64, u64)> = Vec::new();
    let mut current = ranges[0];
    for (start, end) in ranges.into_iter().skip(1) {
        if start <= current.1 + 1 {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);
    merged
}

fn contains_range(ranges: &[(u64, u64)], value: u64) -> bool {
    let mut lo: usize = 0;
    let mut hi: usize = ranges.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let (start, end) = ranges[mid];
        match value.cmp(&start) {
            Ordering::Less => {
                hi = mid;
            }
            Ordering::Greater => {
                if value > end {
                    lo = mid + 1;
                } else {
                    return true;
                }
            }
            Ordering::Equal => return true,
        }
    }
    false
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

    let text = String::from_utf8_lossy(&input).replace("\r\n", "\n");
    let mut sections = text.split("\n\n");
    let ranges_section = sections.next().unwrap_or("");
    let ids_section = sections.next().unwrap_or("");

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in ranges_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.splitn(2, '-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();
        ranges.push((start, end));
    }

    let merged = merge_ranges(ranges);

    let mut count: u64 = 0;
    for line in ids_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let value = line.parse::<u64>().unwrap();
        if contains_range(&merged, value) {
            count += 1;
        }
    }

    println!("{count}");
}
