use std::cmp::{max, min};
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

fn parse_numbers(input: &[u8]) -> Vec<u128> {
    let mut nums = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        if input[i].is_ascii_digit() {
            let mut value: u128 = 0;
            while i < input.len() && input[i].is_ascii_digit() {
                value = value * 10 + (input[i] - b'0') as u128;
                i += 1;
            }
            nums.push(value);
        } else {
            i += 1;
        }
    }
    nums
}

fn ceil_div(a: u128, b: u128) -> u128 {
    if a == 0 {
        0
    } else {
        (a + b - 1) / b
    }
}

fn sum_invalid_range(left: u128, right: u128) -> u128 {
    let mut total: u128 = 0;
    let mut pow10: u128 = 1;

    for _ in 1..=19 {
        pow10 *= 10;
        let factor = pow10 + 1;
        let lower_k = pow10 / 10;
        let min_n = lower_k * factor;
        if min_n > right {
            break;
        }

        let k_min = max(lower_k, ceil_div(left, factor));
        let k_max = min(pow10 - 1, right / factor);
        if k_min <= k_max {
            let count = k_max - k_min + 1;
            let sum_k = (k_min + k_max) * count / 2;
            total += factor * sum_k;
        }
    }

    total
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
        eprintln!("input has odd number of bounds");
        std::process::exit(1);
    }

    let mut total: u128 = 0;
    let mut idx = 0;
    while idx < nums.len() {
        let left = nums[idx];
        let right = nums[idx + 1];
        total += sum_invalid_range(left, right);
        idx += 2;
    }

    println!("{total}");
}
