use std::collections::HashMap;
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

fn parse_line(line: &str) -> Result<(u128, Vec<u128>), String> {
    let bytes = line.as_bytes();
    let mut i: usize = 0;
    let mut target: u128 = 0;
    let mut lights: usize = 0;
    let mut buttons: Vec<u128> = Vec::new();

    while i < bytes.len() {
        match bytes[i] {
            b'[' => {
                i += 1;
                let mut idx: usize = 0;
                while i < bytes.len() && bytes[i] != b']' {
                    match bytes[i] {
                        b'#' => {
                            if idx >= 128 {
                                return Err("too many indicator lights".into());
                            }
                            target |= 1u128 << idx;
                            idx += 1;
                        }
                        b'.' => {
                            idx += 1;
                        }
                        _ => {}
                    }
                    i += 1;
                }
                lights = idx;
            }
            b'(' => {
                i += 1;
                let mut mask: u128 = 0;
                let mut value: usize = 0;
                let mut in_num = false;
                while i < bytes.len() && bytes[i] != b')' {
                    if bytes[i].is_ascii_digit() {
                        value = value * 10 + (bytes[i] - b'0') as usize;
                        in_num = true;
                    } else if bytes[i] == b',' {
                        if in_num {
                            if value >= 128 {
                                return Err("toggle index too large".into());
                            }
                            mask |= 1u128 << value;
                            value = 0;
                            in_num = false;
                        }
                    }
                    i += 1;
                }
                if in_num {
                    if value >= 128 {
                        return Err("toggle index too large".into());
                    }
                    mask |= 1u128 << value;
                }
                buttons.push(mask);
            }
            _ => {}
        }
        i += 1;
    }

    if lights == 0 {
        return Err("no indicator lights found".into());
    }
    Ok((target, buttons))
}

fn min_presses(target: u128, buttons: &[u128]) -> u32 {
    let n = buttons.len();
    if n == 0 {
        return if target == 0 { 0 } else { u32::MAX };
    }
    let n1 = n / 2;
    let n2 = n - n1;
    let left = &buttons[..n1];
    let right = &buttons[n1..];

    let left_size = 1usize << n1;
    let mut left_xor = vec![0u128; left_size];
    let mut left_count = vec![0u16; left_size];
    for i in 1..left_size {
        let lsb = i & i.wrapping_neg();
        let bit = lsb.trailing_zeros() as usize;
        let prev = i ^ lsb;
        left_xor[i] = left_xor[prev] ^ left[bit];
        left_count[i] = left_count[prev] + 1;
    }

    let mut left_best: HashMap<u128, u16> = HashMap::with_capacity(left_size);
    for i in 0..left_size {
        let val = left_xor[i];
        let cnt = left_count[i];
        match left_best.get(&val) {
            Some(&prev) if prev <= cnt => {}
            _ => {
                left_best.insert(val, cnt);
            }
        }
    }

    let right_size = 1usize << n2;
    let mut best: u32 = u32::MAX;
    let mut right_xor = vec![0u128; right_size];
    let mut right_count = vec![0u16; right_size];
    for i in 1..right_size {
        let lsb = i & i.wrapping_neg();
        let bit = lsb.trailing_zeros() as usize;
        let prev = i ^ lsb;
        right_xor[i] = right_xor[prev] ^ right[bit];
        right_count[i] = right_count[prev] + 1;
    }

    for i in 0..right_size {
        let val = right_xor[i];
        let cnt = right_count[i];
        let needed = target ^ val;
        if let Some(&left_cnt) = left_best.get(&needed) {
            let total = left_cnt as u32 + cnt as u32;
            if total < best {
                best = total;
            }
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

    let text = String::from_utf8_lossy(&input);
    let mut total: u32 = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (target, buttons) = match parse_line(line) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("parse error: {err}");
                std::process::exit(1);
            }
        };
        let presses = min_presses(target, &buttons);
        if presses == u32::MAX {
            eprintln!("no solution for line: {line}");
            std::process::exit(1);
        }
        total += presses;
    }

    println!("{total}");
}
