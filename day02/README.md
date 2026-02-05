# Day 02: Gift Shop (Part 1)

## Summary

Invalid IDs are numbers formed by repeating a digit sequence twice. For each
range [L, R], iterate over possible half-lengths d and compute all k with
d digits such that n = k * (10^d + 1) lies in [L, R]. Sum via arithmetic series.

Time complexity: O(R * D) where D <= 19 (digit lengths).
Space complexity: O(1).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day02/solutions/primary/solve \
  day02/solutions/primary/main.rs
```

Run:

```
./day02/solutions/primary/solve --part 1 --input day02/example.txt
```

### Baseline (Python)

Run:

```
python3 day02/solutions/baseline/main.py --part 1 --input day02/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day02/solutions/primary/solve --part 1 --input day02/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day02/solutions/baseline/main.py --part 1 --input day02/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.312 (best 1.023) | n/a | Math-based range summation |
| Baseline | Python 3 | cpython | 18.672 (best 18.110) | n/a | Regex parsing + arithmetic series |
