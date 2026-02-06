# Day 02: Gift Shop (Parts 1 & 2)

## Summary

Part 1: invalid IDs are numbers formed by repeating a digit sequence twice.
For each range [L, R], iterate over possible digit lengths d and compute all
s values such that n = s * (10^d + 1) lies in [L, R]. Sum via arithmetic series.

Part 2: allow any repetition count k >= 2. For each d and k, compute the
repetition factor (1 + 10^d + ... + 10^{(k-1)d}) and sum all s in range.

Time complexity: O(R * D * K) where D is digit length and K repeat count.
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
./day02/solutions/primary/solve --part 2 --input day02/example.txt
```

### Baseline (Python)

Run:

```
python3 day02/solutions/baseline/main.py --part 1 --input day02/example.txt
python3 day02/solutions/baseline/main.py --part 2 --input day02/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day02/solutions/primary/solve --part 1 --input day02/example.txt" \
  --part2 "./day02/solutions/primary/solve --part 2 --input day02/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day02/solutions/baseline/main.py --part 1 --input day02/example.txt" \
  --part2 "python3 day02/solutions/baseline/main.py --part 2 --input day02/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | TBD | TBD | Math-based range summation |
| Baseline | Python 3 | cpython | TBD | TBD | Regex parsing + arithmetic series |
