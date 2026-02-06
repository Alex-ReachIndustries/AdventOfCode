# Day 05: Cafeteria (Part 1)

## Summary

Parse fresh ID ranges, merge overlaps, then count how many available IDs fall
within the merged intervals via binary search.

Time complexity: O(R log R + I log R).
Space complexity: O(R).

## Implementations

### Primary (Rust)

Build:

```
rustc -O -C target-cpu=native -o day05/solutions/primary/solve \
  day05/solutions/primary/main.rs
```

Run:

```
./day05/solutions/primary/solve --part 1 --input day05/example.txt
```

### Baseline (Python)

Run:

```
python3 day05/solutions/baseline/main.py --part 1 --input day05/example.txt
```

## Benchmarking

Commands used:

```
python3 tools/bench.py \
  --part1 "./day05/solutions/primary/solve --part 1 --input day05/example.txt" \
  --runs 20 --warmup 3

python3 tools/bench.py \
  --part1 "python3 day05/solutions/baseline/main.py --part 1 --input day05/example.txt" \
  --runs 20 --warmup 3
```

## Results

| Implementation (method) | Language/Runtime | Build flags / mode | Part 1 time (ms) | Part 2 time (ms) | Notes |
| --- | --- | --- | --- | --- | --- |
| Primary | Rust | rustc -O -C target-cpu=native | 1.328 (best 1.046) | n/a | Merge ranges + binary search |
| Baseline | Python 3 | cpython | 18.819 (best 18.311) | n/a | Merge ranges + bisect |
