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

fn digits_u128(mut value: u128) -> usize {
    if value == 0 {
        return 1;
    }
    let mut count = 0;
    while value > 0 {
        count += 1;
        value /= 10;
    }
    count
}

fn sum_invalid_range(left: u128, right: u128, pow10: &[u128], max_digits: usize) -> u128 {
    let mut total: u128 = 0;
    let max_d = max_digits / 2;
    if max_d == 0 {
        return 0;
    }

    for d in 1..=max_d {
        let pow10_d = pow10[d];
        let factor = pow10_d + 1;
        let lower_k = pow10[d - 1];
        let min_n = match lower_k.checked_mul(factor) {
            Some(value) => value,
            None => break,
        };
        if min_n > right {
            break;
        }

        let k_min = max(lower_k, ceil_div(left, factor));
        let k_max = min(pow10_d - 1, right / factor);
        if k_min <= k_max {
            let count = k_max - k_min + 1;
            let sum_k = (k_min + k_max) * count / 2;
            total += factor * sum_k;
        }
    }

    total
}

fn sum_primitive(
    d: usize,
    mut left: u128,
    mut right: u128,
    pow10: &[u128],
    divisors: &[Vec<usize>],
    memo: &mut std::collections::HashMap<(usize, u128, u128), u128>,
) -> u128 {
    let lower = pow10[d - 1];
    let upper = pow10[d] - 1;
    if left < lower {
        left = lower;
    }
    if right > upper {
        right = upper;
    }
    if left > right {
        return 0;
    }

    let key = (d, left, right);
    if let Some(value) = memo.get(&key) {
        return *value;
    }

    let count = right - left + 1;
    let mut total = (left + right) * count / 2;

    for &q in &divisors[d] {
        let repeats = d / q;
        let mut rep: u128 = 1;
        for _ in 1..repeats {
            rep = rep * pow10[q] + 1;
        }
        let t_min = ceil_div(left, rep);
        let t_max = right / rep;
        let sub = sum_primitive(q, t_min, t_max, pow10, divisors, memo);
        total -= rep * sub;
    }

    memo.insert(key, total);
    total
}

fn sum_invalid_range_repeat(
    left: u128,
    right: u128,
    pow10: &[u128],
    max_digits: usize,
) -> u128 {
    let mut total: u128 = 0;
    let max_d = max_digits / 2;
    if max_d == 0 {
        return 0;
    }

    let mut divisors: Vec<Vec<usize>> = vec![Vec::new(); max_d + 1];
    for d in 2..=max_d {
        let mut q = 1;
        while q * q <= d {
            if d % q == 0 {
                let other = d / q;
                if q < d {
                    divisors[d].push(q);
                }
                if other < d && other != q {
                    divisors[d].push(other);
                }
            }
            q += 1;
        }
        divisors[d].sort_unstable();
    }

    let mut memo: std::collections::HashMap<(usize, u128, u128), u128> =
        std::collections::HashMap::new();

    for d in 1..=max_d {
        let pow10_d = pow10[d];
        let lower_k = pow10[d - 1];
        let upper_k = pow10_d - 1;
        let mut rep: u128 = 1;
        let k_limit = max_digits / d;
        for _ in 2..=k_limit {
            rep = rep * pow10_d + 1;
            let min_n = match lower_k.checked_mul(rep) {
                Some(value) => value,
                None => break,
            };
            if min_n > right {
                break;
            }
            let k_min = max(lower_k, ceil_div(left, rep));
            let k_max = min(upper_k, right / rep);
            if k_min <= k_max {
                let sum_s = sum_primitive(d, k_min, k_max, pow10, &divisors, &mut memo);
                total += rep * sum_s;
            }
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

    let nums = parse_numbers(&input);
    if nums.len() % 2 != 0 {
        eprintln!("input has odd number of bounds");
        std::process::exit(1);
    }

    let mut max_r: u128 = 0;
    let mut idx = 0;
    while idx < nums.len() {
        let right = nums[idx + 1];
        if right > max_r {
            max_r = right;
        }
        idx += 2;
    }

    let max_digits = digits_u128(max_r);
    let max_d = max_digits / 2;
    let mut pow10: Vec<u128> = vec![1];
    for _ in 1..=max_d {
        let next = pow10.last().unwrap() * 10;
        pow10.push(next);
    }

    let mut total: u128 = 0;
    let mut idx = 0;
    while idx < nums.len() {
        let left = nums[idx];
        let right = nums[idx + 1];
        if part == 1 {
            total += sum_invalid_range(left, right, &pow10, max_digits);
        } else {
            total += sum_invalid_range_repeat(left, right, &pow10, max_digits);
        }
        idx += 2;
    }

    println!("{total}");
}
