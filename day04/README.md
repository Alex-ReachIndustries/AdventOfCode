# Day 04: Printing Department (Part 1)

## Summary

For each roll '@', count the number of adjacent rolls in the eight neighbors.
If the count is fewer than four, it is accessible. Sum across the grid.

Time complexity: O(H * W) with a constant 8-neighbor scan.
Space complexity: O(1) additional.

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day04/solutions/primary/solve \
  day04/solutions/primary/main.rs
```

Run:

```
./day04/solutions/primary/solve --part 1 --input day04/example.txt
```

### Baseline (Python)

Run:

```
python3 day04/solutions/baseline/main.py --part 1 --input day04/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day04/solutions/primary/solve --part 1 --input day04/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day04/solutions/baseline/main.py --part 1 --input day04/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.403 (best 1.041) | n/a | 8-neighbor scan |
| Baseline | Python 3 | cpython | 18.705 (best 18.193) | n/a | Direct grid loops |
