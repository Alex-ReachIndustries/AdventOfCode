# Day 08: Playground (Part 1)

## Summary

Compute the closest K pairs by squared distance (K = min(1000, N/2)) and union
those endpoints. After processing these edges, multiply the sizes of the three
largest connected components. (For the example with N=20, this matches K=10.)

Time complexity: O(N^2 log K) with K=min(1000, N/2).
Space complexity: O(K + N).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day08/solutions/primary/solve \
  day08/solutions/primary/main.rs
```

Run:

```
./day08/solutions/primary/solve --part 1 --input day08/example.txt
```

### Baseline (Python)

Run:

```
python3 day08/solutions/baseline/main.py --part 1 --input day08/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day08/solutions/primary/solve --part 1 --input day08/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day08/solutions/baseline/main.py --part 1 --input day08/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.198 (best 0.841) | n/a | Heap of K shortest edges |
| Baseline | Python 3 | cpython | 18.887 (best 18.491) | n/a | Heap of K shortest edges |
