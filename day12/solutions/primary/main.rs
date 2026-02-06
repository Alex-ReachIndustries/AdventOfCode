use std::collections::{HashMap, HashSet};
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

#[derive(Clone)]
struct Shape {
    area: usize,
    orientations: Vec<Vec<(usize, usize)>>,
}

#[derive(Clone)]
struct ShapeInstance {
    area: usize,
    count: usize,
    placements: Vec<Vec<u64>>,
}

fn normalize(coords: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let min_x = coords.iter().map(|c| c.0).min().unwrap_or(0);
    let min_y = coords.iter().map(|c| c.1).min().unwrap_or(0);
    let mut norm: Vec<(i32, i32)> = coords
        .iter()
        .map(|(x, y)| (x - min_x, y - min_y))
        .collect();
    norm.sort_unstable();
    norm
}

fn generate_orientations(coords: &[(i32, i32)]) -> Vec<Vec<(usize, usize)>> {
    let mut set: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let mut out: Vec<Vec<(usize, usize)>> = Vec::new();

    for &flip in &[false, true] {
        for rot in 0..4 {
            let mut transformed: Vec<(i32, i32)> = coords
                .iter()
                .map(|&(x, y)| {
                    let mut nx = x;
                    let mut ny = y;
                    if flip {
                        nx = -nx;
                    }
                    for _ in 0..rot {
                        let tmp = nx;
                        nx = ny;
                        ny = -tmp;
                    }
                    (nx, ny)
                })
                .collect();
            transformed = normalize(&transformed);
            if set.insert(transformed.clone()) {
                let converted: Vec<(usize, usize)> = transformed
                    .iter()
                    .map(|(x, y)| (*x as usize, *y as usize))
                    .collect();
                out.push(converted);
            }
        }
    }
    out
}

fn parse_input(text: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes: HashMap<usize, Vec<String>> = HashMap::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    let mut current_idx: Option<usize> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(colon_pos) = line.find(':') {
            let (left, rest) = line.split_at(colon_pos);
            if left.contains('x') {
                let dims = left.split('x').collect::<Vec<_>>();
                let width = dims[0].parse::<usize>().unwrap();
                let height = dims[1].parse::<usize>().unwrap();
                let counts: Vec<usize> = rest[1..]
                    .trim()
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                regions.push((width, height, counts));
                current_idx = None;
            } else {
                let idx = left.parse::<usize>().unwrap();
                current_idx = Some(idx);
                shapes.entry(idx).or_default();
            }
        } else if let Some(idx) = current_idx {
            shapes.entry(idx).or_default().push(line.to_string());
        }
    }

    let mut shape_vec: Vec<(usize, Shape)> = Vec::new();
    for (idx, rows) in shapes {
        let mut coords: Vec<(i32, i32)> = Vec::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    coords.push((x as i32, y as i32));
                }
            }
        }
        let area = coords.len();
        let orientations = generate_orientations(&coords);
        shape_vec.push((idx, Shape { area, orientations }));
    }
    shape_vec.sort_by_key(|(idx, _)| *idx);
    let shapes = shape_vec.into_iter().map(|(_, s)| s).collect();
    (shapes, regions)
}

fn make_mask(
    cells: &[(usize, usize)],
    width: usize,
    offset_x: usize,
    offset_y: usize,
    words: usize,
) -> Vec<u64> {
    let mut mask = vec![0u64; words];
    for (x, y) in cells {
        let idx = (offset_y + y) * width + (offset_x + x);
        let word = idx / 64;
        let bit = idx % 64;
        mask[word] |= 1u64 << bit;
    }
    mask
}

fn can_place(board: &[u64], mask: &[u64]) -> bool {
    for i in 0..board.len() {
        if board[i] & mask[i] != 0 {
            return false;
        }
    }
    true
}

fn apply_mask(board: &mut [u64], mask: &[u64]) {
    for i in 0..board.len() {
        board[i] |= mask[i];
    }
}

fn remove_mask(board: &mut [u64], mask: &[u64]) {
    for i in 0..board.len() {
        board[i] ^= mask[i];
    }
}

fn search(shapes: &mut [ShapeInstance], board: &mut [u64]) -> bool {
    let mut best_idx: Option<usize> = None;
    let mut best_count = usize::MAX;

    for (i, shape) in shapes.iter().enumerate() {
        if shape.count == 0 {
            continue;
        }
        let placements = shape.placements.len();
        if placements == 0 {
            return false;
        }
        if placements < best_count {
            best_count = placements;
            best_idx = Some(i);
            if best_count == 1 {
                break;
            }
        }
    }

    let idx = match best_idx {
        Some(i) => i,
        None => return true,
    };

    let placements = shapes[idx].placements.clone();
    for mask in placements.iter() {
        if !can_place(board, mask) {
            continue;
        }
        apply_mask(board, mask);
        shapes[idx].count -= 1;
        if search(shapes, board) {
            return true;
        }
        shapes[idx].count += 1;
        remove_mask(board, mask);
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

    let text = String::from_utf8_lossy(&input);
    let (shapes, regions) = parse_input(&text);

    let mut fit_count = 0usize;
    for (width, height, counts) in regions {
        let mut total_area = 0usize;
        let mut instances: Vec<ShapeInstance> = Vec::new();
        let words = (width * height + 63) / 64;

        for (idx, shape) in shapes.iter().enumerate() {
            let count = if idx < counts.len() { counts[idx] } else { 0 };
            total_area += count * shape.area;
            let mut placements: Vec<Vec<u64>> = Vec::new();
            for orient in shape.orientations.iter() {
                let max_x = orient.iter().map(|(x, _)| *x).max().unwrap_or(0);
                let max_y = orient.iter().map(|(_, y)| *y).max().unwrap_or(0);
                if max_x + 1 > width || max_y + 1 > height {
                    continue;
                }
                for y in 0..=(height - (max_y + 1)) {
                    for x in 0..=(width - (max_x + 1)) {
                        placements.push(make_mask(orient, width, x, y, words));
                    }
                }
            }
            instances.push(ShapeInstance {
                area: shape.area,
                count,
                placements,
            });
        }

        if total_area > width * height {
            continue;
        }

        instances.sort_by_key(|s| (s.placements.len(), usize::MAX - s.area));
        let mut board = vec![0u64; words];
        if search(&mut instances, &mut board) {
            fit_count += 1;
        }
    }

    println!("{fit_count}");
}
