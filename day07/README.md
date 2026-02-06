# Day 07: Laboratories (Part 1)

## Summary

Track active beam columns row by row. Each active beam moves downward until it
hits a splitter (^), where it stops and spawns beams to the left and right.
Active beams merge naturally by tracking unique columns per row.

Time complexity: O(H * W).
Space complexity: O(W).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day07/solutions/primary/solve \
  day07/solutions/primary/main.rs
```

Run:

```
./day07/solutions/primary/solve --part 1 --input day07/example.txt
```

### Baseline (Python)

Run:

```
python3 day07/solutions/baseline/main.py --part 1 --input day07/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day07/solutions/primary/solve --part 1 --input day07/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day07/solutions/baseline/main.py --part 1 --input day07/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.197 (best 0.869) | n/a | Row-wise beam propagation |
| Baseline | Python 3 | cpython | 18.926 (best 18.261) | n/a | Active column scan |
