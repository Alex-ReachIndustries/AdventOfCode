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

fn parse_grid(input: &[u8]) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut start: usize = 0;
    let mut i: usize = 0;
    while i <= input.len() {
        if i == input.len() || input[i] == b'\n' {
            let mut end = i;
            if end > start && input[end - 1] == b'\r' {
                end -= 1;
            }
            if end > start {
                grid.push(input[start..end].to_vec());
            }
            start = i + 1;
        }
        i += 1;
    }
    grid
}

fn count_accessible(grid: &[Vec<u8>]) -> u32 {
    if grid.is_empty() {
        return 0;
    }
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut total: u32 = 0;
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] != b'@' {
                continue;
            }
            let mut neighbors: u32 = 0;
            for (dy, dx) in offsets.iter() {
                let ny = y + dy;
                let nx = x + dx;
                if ny >= 0 && ny < h && nx >= 0 && nx < w {
                    if grid[ny as usize][nx as usize] == b'@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                total += 1;
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

    let grid = parse_grid(&input);
    let total = count_accessible(&grid);
    println!("{total}");
}
