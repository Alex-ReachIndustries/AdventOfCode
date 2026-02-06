# Day 09: Movie Theater (Part 1)

## Summary

For every pair of red tiles (x1, y1) and (x2, y2), compute the inclusive area
(|x1 - x2| + 1) * (|y1 - y2| + 1) and track the maximum.

Time complexity: O(N^2).
Space complexity: O(1) extra.

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day09/solutions/primary/solve \
  day09/solutions/primary/main.rs
```

Run:

```
./day09/solutions/primary/solve --part 1 --input day09/example.txt
```

### Baseline (Python)

Run:

```
python3 day09/solutions/baseline/main.py --part 1 --input day09/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day09/solutions/primary/solve --part 1 --input day09/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day09/solutions/baseline/main.py --part 1 --input day09/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.255 (best 0.910) | n/a | O(N^2) pair scan |
| Baseline | Python 3 | cpython | 18.532 (best 17.939) | n/a | O(N^2) pair scan |
