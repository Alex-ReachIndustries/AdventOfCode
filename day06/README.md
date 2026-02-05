# Day 06: Trash Compactor (Part 1)

## Summary

Treat the worksheet as a character grid. Each problem is a contiguous column
segment with at least one non-space character. The last row holds the operator;
for each segment, read digits in each row above to form the operands and apply
the operator. Sum the per-problem results.

Time complexity: O(H * W).
Space complexity: O(H * W) for the grid.

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day06/solutions/primary/solve \
  day06/solutions/primary/main.rs
```

Run:

```
./day06/solutions/primary/solve --part 1 --input day06/example.txt
```

### Baseline (Python)

Run:

```
python3 day06/solutions/baseline/main.py --part 1 --input day06/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day06/solutions/primary/solve --part 1 --input day06/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day06/solutions/baseline/main.py --part 1 --input day06/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.190 (best 0.838) | n/a | Column segment parsing |
| Baseline | Python 3 | cpython | 18.906 (best 18.200) | n/a | Grid scan per segment |
