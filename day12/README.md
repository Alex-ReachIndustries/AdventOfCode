# Day 12: Christmas Tree Farm (Part 1)

## Summary

Parse polyomino shapes, generate all unique rotations/reflections, then
backtrack to place the required counts into each region without overlap.
Empty space is allowed, so this is a packing search with pruning by the most
constrained shape.

Time complexity: exponential in number of pieces (backtracking).
Space complexity: O(P) for placement masks.

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day12/solutions/primary/solve \
  day12/solutions/primary/main.rs
```

Run:

```
./day12/solutions/primary/solve --part 1 --input day12/example.txt
```

### Baseline (Python)

Run:

```
python3 day12/solutions/baseline/main.py --part 1 --input day12/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day12/solutions/primary/solve --part 1 --input day12/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day12/solutions/baseline/main.py --part 1 --input day12/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 914.436 (best 909.431) | n/a | Backtracking + bitmasks |
| Baseline | Python 3 | cpython | 994.661 (best 979.496) | n/a | Backtracking + int bitmask |
