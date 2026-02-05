# Day 11: Reactor (Part 1)

## Summary

Count all directed paths from `you` to `out` in a DAG using DFS with memoized
path counts. A lightweight big-integer is used in Rust to avoid overflow.

Time complexity: O(V + E).
Space complexity: O(V + E).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day11/solutions/primary/solve \
  day11/solutions/primary/main.rs
```

Run:

```
./day11/solutions/primary/solve --part 1 --input day11/example.txt
```

### Baseline (Python)

Run:

```
python3 day11/solutions/baseline/main.py --part 1 --input day11/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day11/solutions/primary/solve --part 1 --input day11/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day11/solutions/baseline/main.py --part 1 --input day11/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.186 (best 0.865) | n/a | DFS + memo + BigNum |
| Baseline | Python 3 | cpython | 18.086 (best 17.756) | n/a | DFS + memo |
